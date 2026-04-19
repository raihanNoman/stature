#[cfg(test)]
mod tests {
    use crate::state::*;
    use anchor_lang::{AccountDeserialize, InstructionData};
    use litesvm::LiteSVM;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        transaction::Transaction,
    };

    const SYSTEM_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");
    const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("9VFHpUQnHsG94AKzGfzf4mAeunxcQw8G9am6FfVEBVZb");
    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    // -------------------------------
    // PDA HELPERS
    // -------------------------------
    fn get_config_pda() -> Pubkey {
        Pubkey::find_program_address(&[b"config"], &PROGRAM_ID).0
    }

    fn get_registered_program_pda(target_program: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[b"registered_program", target_program.as_ref()],
            &PROGRAM_ID,
        )
        .0
    }

    fn get_user_pda(wallet: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[b"user", wallet.as_ref()], &PROGRAM_ID).0
    }

    fn get_program_user_state_pda(program_pda: &Pubkey, user_pda: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[b"state", program_pda.as_ref(), user_pda.as_ref()],
            &PROGRAM_ID,
        )
        .0
    }

    fn get_record_pda(user_pda: &Pubkey, program_pda: &Pubkey, source: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[
                b"record",
                user_pda.as_ref(),
                program_pda.as_ref(),
                source.as_ref(),
            ],
            &PROGRAM_ID,
        )
        .0
    }

    // -------------------------------
    // INSTRUCTION BUILDERS
    // -------------------------------

    fn ix_register_program(payer: &Pubkey, target: &Pubkey, name: String) -> Instruction {
        let data = crate::instruction::CreateProgram { name }.data();
        Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(*payer, true),
                AccountMeta::new_readonly(*target, false),
                AccountMeta::new(get_registered_program_pda(target), false),
                AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            ],
            data,
        }
    }

    fn ix_verify_program(admin: &Pubkey, target: &Pubkey) -> Instruction {
        let data = crate::instruction::UpdateProgramVerifiedStatus {}.data();
        Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new_readonly(*admin, true),
                AccountMeta::new_readonly(get_config_pda(), false),
                AccountMeta::new(get_registered_program_pda(target), false),
            ],
            data,
        }
    }

    fn ix_update_stature(
        payer: &Pubkey,
        authority: &Pubkey, // The target program's key/PDA
        user_wallet: &Pubkey,
        source: &Pubkey,
        stature: i64,
        memo: String,
    ) -> Instruction {
        let program_pda = get_registered_program_pda(authority);
        let user_pda = get_user_pda(user_wallet);

        let data = crate::instruction::UpdateUserStature { stature, memo }.data();
        Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(*payer, true),
                AccountMeta::new_readonly(*authority, true), // The program authority must sign
                AccountMeta::new(program_pda, false),
                AccountMeta::new_readonly(*user_wallet, false),
                AccountMeta::new(user_pda, false),
                AccountMeta::new_readonly(*source, false),
                AccountMeta::new(get_program_user_state_pda(&program_pda, &user_pda), false),
                AccountMeta::new(get_record_pda(&user_pda, &program_pda, source), false),
                AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            ],
            data,
        }
    }

    // -------------------------------
    // TEST SUITE
    // -------------------------------

    #[test]
    fn test_full_stature_lifecycle() {
        let mut svm = LiteSVM::new();
        let program_bytes = include_bytes!("../../../target/deploy/stature.so");
        svm.add_program(PROGRAM_ID, program_bytes).unwrap();

        let admin = Keypair::new();
        let external_program = Keypair::new();
        let user_wallet = Keypair::new();
        let source_action = Keypair::new();

        svm.airdrop(&admin.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();
        svm.airdrop(&external_program.pubkey(), 10 * LAMPORTS_PER_SOL)
            .unwrap();
        svm.airdrop(&user_wallet.pubkey(), 10 * LAMPORTS_PER_SOL)
            .unwrap();

        // ---------------------------------------------------------
        // 1. Setup Config (NEW STEP)
        // ---------------------------------------------------------
        // We simulate the 'initialize_config' call.
        // If you have an InitConfig instruction, call it here.
        // Otherwise, we manually set the state in LiteSVM:

        // Anchor discriminators are 8 bytes.
        // For a struct named "Config", the discriminator is:
        // sha256("account:Config")[..8]
        // 1. Setup Config
        let config_pda = get_config_pda();

        // 8 (discriminator) + 32 (pubkey) + 1 (u8) = 41 bytes
        let mut config_data = Vec::with_capacity(41);

        // The Anchor Discriminator
        let discriminator = &solana_sdk::hash::hash(b"account:Config").to_bytes()[..8];
        config_data.extend_from_slice(discriminator);

        // The Admin Pubkey
        config_data.extend_from_slice(&admin.pubkey().to_bytes());

        // The Bump (use any value, e.g., 255)
        config_data.push(255);

        let config_account = solana_sdk::account::Account {
            lamports: LAMPORTS_PER_SOL,
            data: config_data,
            owner: PROGRAM_ID, // Ensure this is the correct PROGRAM_ID
            executable: false,
            rent_epoch: 0,
        };

        svm.set_account(config_pda, config_account).unwrap();

        // ---------------------------------------------------------
        // 2. Register Program
        // ---------------------------------------------------------
        let ix = ix_register_program(
            &user_wallet.pubkey(),
            &external_program.pubkey(),
            "GameProt".to_string(),
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user_wallet.pubkey()),
            &[&user_wallet],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx).unwrap();

        // ---------------------------------------------------------
        // 3. Verify Program (Now this won't fail with 3012!)
        // ---------------------------------------------------------
        let ix = ix_verify_program(&admin.pubkey(), &external_program.pubkey());
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&admin.pubkey()),
            &[&admin],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx).unwrap();

        // 4. Update Stature
        let ix = ix_update_stature(
            &user_wallet.pubkey(),
            &external_program.pubkey(),
            &user_wallet.pubkey(),
            &source_action.pubkey(),
            100,
            "example 64 bit uuid from data base".to_string(),
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user_wallet.pubkey()),
            &[&user_wallet, &external_program],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx).unwrap();

        // 5. Verification
        let user_pda = get_user_pda(&user_wallet.pubkey());
        let user_acc = svm.get_account(&user_pda).unwrap();
        let user_data: User = User::try_deserialize(&mut &user_acc.data[..]).unwrap();

        assert!(user_data.stature > 0);
        assert_eq!(user_data.wallet.to_bytes(), user_wallet.pubkey().to_bytes());
    }
    #[test]
    fn test_unverified_program_fails_stature_update() {
        let mut svm = LiteSVM::new();
        let program_bytes = include_bytes!("../../../target/deploy/stature.so");
        svm.add_program(PROGRAM_ID, program_bytes).unwrap();

        let user = Keypair::new();
        let fake_program = Keypair::new();
        svm.airdrop(&user.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();

        // Register but DON'T verify
        let ix = ix_register_program(&user.pubkey(), &fake_program.pubkey(), "Evil".into());
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user.pubkey()),
            &[&user],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx).unwrap();

        // Try to update stature
        let ix = ix_update_stature(
            &user.pubkey(),
            &fake_program.pubkey(),
            &user.pubkey(),
            &Pubkey::new_unique(),
            10,
            "example 64 bit uuid from data base".to_string(),
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user.pubkey()),
            &[&user, &fake_program],
            svm.latest_blockhash(),
        );

        // Should fail because is_verified is false
        assert!(svm.send_transaction(tx).is_err());
    }
}
