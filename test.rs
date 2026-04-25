#![cfg(test)]

use soroban_sdk::{Env, Address};
use crate::TaskEscrow;

#[test]
fn test_happy_path_full_flow() {
    let env = Env::default();
    let contract = TaskEscrow;

    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);

    contract.create_escrow(env.clone(), 1, client, freelancer.clone(), 1000);
    contract.deposit(env.clone(), 1);
    contract.approve_by_freelancer(env.clone(), 1);
    contract.approve_by_client(env.clone(), 1);

    let recipient = contract.release_payment(env.clone(), 1);

    assert_eq!(recipient, freelancer);
}

#[test]
fn test_release_fails_without_approval() {
    let env = Env::default();
    let contract = TaskEscrow;

    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);

    contract.create_escrow(env.clone(), 1, client, freelancer, 1000);
    contract.deposit(env.clone(), 1);

    let result = std::panic::catch_unwind(|| {
        contract.release_payment(env.clone(), 1);
    });

    assert!(result.is_err());
}

#[test]
fn test_storage_state_after_deposit() {
    let env = Env::default();
    let contract = TaskEscrow;

    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);

    contract.create_escrow(env.clone(), 1, client, freelancer, 1000);
    contract.deposit(env.clone(), 1);

    let escrow = env
        .storage()
        .instance()
        .get(&crate::DataKey::Escrow(1))
        .unwrap();

    assert!(escrow.funded);
}

