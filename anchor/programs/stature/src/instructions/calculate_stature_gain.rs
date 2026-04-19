use crate::error::ErrorCode;

use anchor_lang::prelude::*;

pub fn calculate_stature_gain(
    tx_value: i64,
    domain_weight: u16,
    total_records: u64,
    first_action_ts: i64,
    current_ts: i64,
) -> Result<i64> {
    // 1. Logarithmic Scaling (V_log)
    // We use the position of the highest bit to simulate a log2 curve.
    // This scales: 10 -> 4, 100 -> 7, 1000 -> 10, 1000000 -> 20.
    let absolute_value = tx_value.abs() as u64;
    // Safety check: if the "work" reported is 0, stature gain is 0
    if absolute_value == 0 {
        return Ok(0);
    }
    let v_log = (64 - absolute_value.leading_zeros()) as i64;

    // 2. Activity Density (T_c)
    // Density = Total Records / Months Active
    let seconds_active = (current_ts - first_action_ts).max(1);
    let months_active = seconds_active / (30 * 24 * 60 * 60); // 30-day month
    
    // Pulse Multiplier: We reward consistency. 
    // If they have many records in a short time, density is high.
    // We cap this to prevent hyper-inflation from spam.
    let density = if months_active < 1 {
        total_records // First month bonus
    } else {
        total_records / months_active as u64
    };

    // T_c Multiplier: Base 100 (1.0x). Max 200 (2.0x).
    // Every 5 actions/month adds 10% to the multiplier.
    let t_c_bonus = (density * 2).min(100) as i64; 

    // T_c represents how "reliable" this human is across the whole network
    let t_c = 100 + t_c_bonus; 

    // 3. Combine with Domain Weight (D_w)
    // Formula: (V_log * D_w * T_c) / 100 (to account for the T_c percentage)
    let domain_weight_i64 = domain_weight as i64;
    
    let step1 = v_log.checked_mul(domain_weight_i64).ok_or(ErrorCode::Overflow)?;
    let step2 = step1.checked_mul(t_c).ok_or(ErrorCode::Overflow)?;
    let final_gain = step2.checked_div(100).ok_or(ErrorCode::Overflow)?;

    // Preserve the sign (if the stature was negative/penalty)
    if tx_value < 0 {
        Ok(-final_gain)
    } else {
        Ok(final_gain)
    }
}