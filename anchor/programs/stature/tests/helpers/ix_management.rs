use anchor_litesvm::{AnchorContext, Instruction, Pubkey};
//litesvm::solana_sdk::account::Account;

use crate::helpers::setup::*;

use crate::stature::client;

pub fn ix_manage_program_suspension(
    ctx: &mut AnchorContext,
    admin: &Pubkey,
    target_program: &Pubkey,
    is_suspended: bool,
) -> Instruction {
    let config_pda = get_config_pda();
    let registered_program_pda = get_registered_program_pda(target_program);

    ctx.program()
        .accounts(client::accounts::UpdateProgramSuspension {
            admin: *admin,
            config: config_pda,
            target_program: *target_program,
            registered_program: registered_program_pda,
        })
        .args(client::args::UpdateProgramSuspension { is_suspended })
        .instruction()
        .unwrap()
}

pub fn ix_manage_admin(
    ctx: &mut AnchorContext,
    current_admin: &Pubkey,
    new_admin: &Pubkey,
) -> Instruction {
    let config_pda = get_config_pda();

    ctx.program()
        .accounts(client::accounts::UpdateAdmin {
            admin: *current_admin,
            config: config_pda,
        })
        .args(client::args::UpdateAdmin {
            new_admin: *new_admin,
        })
        .instruction()
        .unwrap()
}

pub fn ix_manage_program_stature(
    ctx: &mut AnchorContext,
    admin: &Pubkey,
    target_program: &Pubkey,
    stature: i64,
) -> Instruction {
    let config_pda = get_config_pda();
    let registered_program_pda = get_registered_program_pda(target_program);

    ctx.program()
        .accounts(client::accounts::UpdateProgramStature {
            admin: *admin,
            config: config_pda,
            target_program: *target_program,
            registered_program: registered_program_pda,
        })
        .args(client::args::UpdateProgramStature {
            new_stature: stature,
        })
        .instruction()
        .unwrap()
}
