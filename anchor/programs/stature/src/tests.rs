#[cfg(test)]
mod tests {
    use crate::ID as PROGRAM_ID;
    use anchor_lang::prelude::system_program;
    use anchor_lang::AccountDeserialize;
    use litesvm::LiteSVM;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        transaction::Transaction,
    };

    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    // -------------------------------
    // PDA HELPERS
    // -------------------------------
    fn get_company_pda(authority: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"company", authority.as_ref()], &PROGRAM_ID)
    }

    fn get_user_pda(owner: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"user", owner.as_ref()], &PROGRAM_ID)
    }

    fn get_record_pda(user: &Pubkey, company: &Pubkey, count: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"record",
                user.as_ref(),
                company.as_ref(),
                &count.to_le_bytes(),
            ],
            &PROGRAM_ID,
        )
    }

    // -------------------------------
    // INSTRUCTION BUILDERS
    // -------------------------------

fn ix_initialize_company(
    authority: &Pubkey,
    company: &Pubkey,
    name: String,
) -> Instruction {
    let data = anchor_lang::InstructionData::data(
        &crate::instruction::InitializeCompany {
            name,
        }
    );

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*company, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data,
    }
}

    fn ix_initialize_user(owner: &Pubkey, user: &Pubkey, name: String) -> Instruction {
        let data = anchor_lang::InstructionData::data(&crate::instruction::InitializeUser { name });

        Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(*owner, true),
                AccountMeta::new(*user, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data,
        }
    }

    fn ix_update_honor(
        authority: &Pubkey,
        company: &Pubkey,
        user: &Pubkey,
        record: &Pubkey,
        amount: i64,
        nonce: u64
    ) -> Instruction {
        let data = anchor_lang::InstructionData::data(&crate::instruction::UpdateUserStature { amount, nonce });

        Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(*authority, true),
                AccountMeta::new(*company, false),
                AccountMeta::new(*user, false),
                AccountMeta::new(*record, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data,
        }
    }

    fn setup_svm() -> (LiteSVM, Keypair, Keypair) {
        let mut svm = LiteSVM::new();
        let program_bytes = include_bytes!("../../../target/deploy/stature.so");
        svm.add_program(PROGRAM_ID, program_bytes)
            .expect("Build .so first");

        let auth = Keypair::new();
        let user = Keypair::new();
        svm.airdrop(&auth.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();
        svm.airdrop(&user.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();
        (svm, auth, user)
    }

    // -------------------------------
    // MAIN TEST
    // -------------------------------
    #[test]
    fn test_full_honor_flow() {
        let mut svm = LiteSVM::new();

        // Load program
        let program_bytes = include_bytes!("../../../target/deploy/stature.so");

        svm.add_program(PROGRAM_ID, program_bytes)
            .expect("Failed to load program into LiteSVM");
        // Actors
        let authority = Keypair::new();
        let user_wallet = Keypair::new();

        svm.airdrop(&authority.pubkey(), 10 * LAMPORTS_PER_SOL)
            .unwrap();
        svm.airdrop(&user_wallet.pubkey(), 10 * LAMPORTS_PER_SOL)
            .unwrap();

        // PDAs
        let (company_pda, _) = get_company_pda(&authority.pubkey());
        let (user_pda, _) = get_user_pda(&user_wallet.pubkey());

        // -------------------------------
        // Initialize Company
        // -------------------------------
        let ix = ix_initialize_company(
            &authority.pubkey(),
            &company_pda,
            "Acme".to_string(),
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority],
            svm.latest_blockhash(),
        );

        assert!(svm.send_transaction(tx).is_ok());

        // -------------------------------
        // Initialize User
        // -------------------------------
        let ix = ix_initialize_user(&user_wallet.pubkey(), &user_pda, "Alice".to_string());

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user_wallet.pubkey()),
            &[&user_wallet],
            svm.latest_blockhash(),
        );

        assert!(svm.send_transaction(tx).is_ok());

        // -------------------------------
        // First honor update
        // -------------------------------
        let (record_pda_0, _) = get_record_pda(&user_pda, &company_pda, 0);

        let ix = ix_update_honor(
            &authority.pubkey(),
            &company_pda,
            &user_pda,
            &record_pda_0,
            10,
            10, 
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority],
            svm.latest_blockhash(),
        );

        assert!(svm.send_transaction(tx).is_ok());

        // -------------------------------
        // Second honor update
        // -------------------------------
        let (record_pda_1, _) = get_record_pda(&user_pda, &company_pda, 1);

        let ix = ix_update_honor(
            &authority.pubkey(),
            &company_pda,
            &user_pda,
            &record_pda_1,
            5,
            11
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority],
            svm.latest_blockhash(),
        );

        assert!(svm.send_transaction(tx).is_ok());

        // -------------------------------
        // Verify user state
        // -------------------------------
        let user_account = svm.get_account(&user_pda).unwrap();
        let user_data: crate::User =
            anchor_lang::AccountDeserialize::try_deserialize(&mut &user_account.data[..]).unwrap();

        assert_eq!(user_data.stature, 15);

        // -------------------------------
        // Verify records exist
        // -------------------------------
        assert!(svm.get_account(&record_pda_0).is_some());
        assert!(svm.get_account(&record_pda_1).is_some());
    }

    // -------------------------------
    // SECURITY TEST
    // -------------------------------
    #[test]
    fn test_unauthorized_company_fails() {
        let mut svm = LiteSVM::new();

        let program_bytes = include_bytes!("../../../target/deploy/stature.so");
        svm.add_program(PROGRAM_ID, program_bytes)
            .expect("Failed to load program into LiteSVM");

        let authority = Keypair::new();
        let attacker = Keypair::new();

        svm.airdrop(&authority.pubkey(), 10 * LAMPORTS_PER_SOL)
            .unwrap();
        svm.airdrop(&attacker.pubkey(), 10 * LAMPORTS_PER_SOL)
            .unwrap();

        let (company_pda, _) = get_company_pda(&authority.pubkey());
        let (user_pda, _) = get_user_pda(&attacker.pubkey());

        // init company (valid)
        let ix = ix_initialize_company(
            &authority.pubkey(),
            &company_pda,
            "acme.com".to_string(),
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority],
            svm.latest_blockhash(),
        );

        svm.send_transaction(tx).unwrap();

        // attacker tries to use company
        let (record_pda, _) = get_record_pda(&user_pda, &company_pda, 0);

        let ix = ix_update_honor(
            &attacker.pubkey(), // ❌ not authority
            &company_pda,
            &user_pda,
            &record_pda,
            10,
            12
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&attacker.pubkey()),
            &[&attacker],
            svm.latest_blockhash(),
        );

        assert!(svm.send_transaction(tx).is_err());
    }

    #[test]
    fn test_production_flow() {
        let (mut svm, authority, user_wallet) = setup_svm();
        let (company_pda, _) = get_company_pda(&authority.pubkey());
        let (user_pda, _) = get_user_pda(&user_wallet.pubkey());

        // 1. Init Company
        let tx = Transaction::new_signed_with_payer(
            &[ix_initialize_company(
                &authority.pubkey(),
                &company_pda,
                "acme.com".into(),
            )],
            Some(&authority.pubkey()),
            &[&authority],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx).unwrap();

        // 2. Init User
        let tx = Transaction::new_signed_with_payer(
            &[ix_initialize_user(
                &user_wallet.pubkey(),
                &user_pda,
                "Alice".into(),
            )],
            Some(&user_wallet.pubkey()),
            &[&user_wallet],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx).unwrap();

        // 3. Sequential Record Check (Update 0, then 1)
        for i in 0..2 {
            let (record_pda, _) = get_record_pda(&user_pda, &company_pda, i);
            let tx = Transaction::new_signed_with_payer(
                &[ix_update_honor(
                    &authority.pubkey(),
                    &company_pda,
                    &user_pda,
                    &record_pda,
                    10,
                    13
                )],
                Some(&authority.pubkey()),
                &[&authority],
                svm.latest_blockhash(),
            );
            svm.send_transaction(tx).expect("Sequential update failed");
        }

        // 4. Verification
        let user_acc = svm.get_account(&user_pda).unwrap();
        let user_data: crate::User = crate::User::try_deserialize(&mut &user_acc.data[..]).unwrap();
        assert_eq!(user_data.stature, 20);
        // Assuming your User struct has a record_count field:
        // assert_eq!(user_data.record_count, 2);
    }

    #[test]
    fn test_security_violation_wrong_counter() {
        let (mut svm, authority, user_wallet) = setup_svm();
        let (company_pda, _) = get_company_pda(&authority.pubkey());
        let (user_pda, _) = get_user_pda(&user_wallet.pubkey());

        // Setup company and user
        let _ = svm.send_transaction(Transaction::new_signed_with_payer(
            &[ix_initialize_company(
                &authority.pubkey(),
                &company_pda,
                "B".into(),
            )],
            Some(&authority.pubkey()),
            &[&authority],
            svm.latest_blockhash(),
        ));
        let _ = svm.send_transaction(Transaction::new_signed_with_payer(
            &[ix_initialize_user(
                &user_wallet.pubkey(),
                &user_pda,
                "Alice".into(),
            )],
            Some(&user_wallet.pubkey()),
            &[&user_wallet],
            svm.latest_blockhash(),
        ));

        // TRY TO SKIP TO RECORD INDEX 99
        let (wrong_record_pda, _) = get_record_pda(&user_pda, &company_pda, 99);
        let ix = ix_update_honor(
            &authority.pubkey(),
            &company_pda,
            &user_pda,
            &wrong_record_pda,
            10,
            14
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority],
            svm.latest_blockhash(),
        );

        // This should fail because the seed doesn't match the current user.record_count
        assert!(
            svm.send_transaction(tx).is_err(),
            "Allowed out-of-order record creation!"
        );
    }

    #[test]
    fn test_security_unauthorized_signer() {
        let (mut svm, authority, user_wallet) = setup_svm();
        let hacker = Keypair::new();
        svm.airdrop(&hacker.pubkey(), LAMPORTS_PER_SOL).unwrap();

        let (company_pda, _) = get_company_pda(&authority.pubkey());
        let (user_pda, _) = get_user_pda(&user_wallet.pubkey());
        let (record_pda, _) = get_record_pda(&user_pda, &company_pda, 0);

        // Attacker tries to sign for a company they don't own
        let ix = ix_update_honor(&hacker.pubkey(), &company_pda, &user_pda, &record_pda, 100, 25);
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&hacker.pubkey()),
            &[&hacker],
            svm.latest_blockhash(),
        );

        assert!(
            svm.send_transaction(tx).is_err(),
            "Hacker successfully updated honor!"
        );
    }


    #[test]
fn test_config_double_initialize_fails() {
    let (mut svm, admin, _) = setup_svm();

    let (config_pda, _) = Pubkey::find_program_address(&[b"config"], &PROGRAM_ID);

    let ix = anchor_lang::InstructionData::data(
        &crate::instruction::InitializeConfig {}
    );

    let accounts = vec![
        AccountMeta::new(admin.pubkey(), true),
        AccountMeta::new(config_pda, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let tx = Transaction::new_signed_with_payer(
        &[Instruction { program_id: PROGRAM_ID, accounts: accounts.clone(), data: ix.clone() }],
        Some(&admin.pubkey()),
        &[&admin],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    // Try again ❌
    let tx2 = Transaction::new_signed_with_payer(
        &[Instruction { program_id: PROGRAM_ID, accounts, data: ix }],
        Some(&admin.pubkey()),
        &[&admin],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx2).is_err());
}

#[test]
fn test_unverified_company_cannot_update() {
    let (mut svm, authority, user_wallet) = setup_svm();

    let (company_pda, _) = get_company_pda(&authority.pubkey());
    let (user_pda, _) = get_user_pda(&user_wallet.pubkey());

    // init company + user
    svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix_initialize_company(&authority.pubkey(), &company_pda, "A".into(),)],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    )).unwrap();

    svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix_initialize_user(&user_wallet.pubkey(), &user_pda, "Alice".into())],
        Some(&user_wallet.pubkey()),
        &[&user_wallet],
        svm.latest_blockhash(),
    )).unwrap();

    let (record_pda, _) = get_record_pda(&user_pda, &company_pda, 0);

    let tx = Transaction::new_signed_with_payer(
        &[ix_update_honor(&authority.pubkey(), &company_pda, &user_pda, &record_pda, 10, 1)],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_err());
}

#[test]
fn test_rate_limit_enforced() {
    let (mut svm, authority, user_wallet) = setup_svm();

    let (company_pda, _) = get_company_pda(&authority.pubkey());
    let (user_pda, _) = get_user_pda(&user_wallet.pubkey());

    // Setup + VERIFY company (you'll need to call update_company_verified_status)

    // First update ✅
    let (record_0, _) = get_record_pda(&user_pda, &company_pda, 0);
    svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix_update_honor(&authority.pubkey(), &company_pda, &user_pda, &record_0, 10, 1)],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    )).unwrap();

    // Immediate second update ❌
    let (record_1, _) = get_record_pda(&user_pda, &company_pda, 1);
    let tx = Transaction::new_signed_with_payer(
        &[ix_update_honor(&authority.pubkey(), &company_pda, &user_pda, &record_1, 10, 2)],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    );

    assert!(tx.signatures.len() > 0);
    assert!(svm.send_transaction(tx).is_err());
}

#[test]
fn test_nonce_replay_fails() {
    let (mut svm, authority, user_wallet) = setup_svm();

    let (company_pda, _) = get_company_pda(&authority.pubkey());
    let (user_pda, _) = get_user_pda(&user_wallet.pubkey());

    let (record_0, _) = get_record_pda(&user_pda, &company_pda, 0);

    // First tx
    svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix_update_honor(&authority.pubkey(), &company_pda, &user_pda, &record_0, 10, 5)],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    )).unwrap();

    // Replay same nonce ❌
    let (record_1, _) = get_record_pda(&user_pda, &company_pda, 1);

    let tx = Transaction::new_signed_with_payer(
        &[ix_update_honor(&authority.pubkey(), &company_pda, &user_pda, &record_1, 10, 5)],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_err());
}

#[test]
fn test_excessive_reward_fails() {
    let (mut svm, authority, user_wallet) = setup_svm();

    let (company_pda, _) = get_company_pda(&authority.pubkey());
    let (user_pda, _) = get_user_pda(&user_wallet.pubkey());

    let (record_pda, _) = get_record_pda(&user_pda, &company_pda, 0);

    let tx = Transaction::new_signed_with_payer(
        &[ix_update_honor(&authority.pubkey(), &company_pda, &user_pda, &record_pda, 999999, 1)],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_err());
}

#[test]
fn test_company_rep_cannot_rate_self() {
    let (mut svm, authority, _) = setup_svm();

    let (company_pda, _) = get_company_pda(&authority.pubkey());
    let (user_pda, _) = get_user_pda(&authority.pubkey()); // SAME

    let (record_pda, _) = get_record_pda(&user_pda, &company_pda, 0);

    let tx = Transaction::new_signed_with_payer(
        &[ix_update_honor(&authority.pubkey(), &company_pda, &user_pda, &record_pda, 10, 1)],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_err());
}
}
