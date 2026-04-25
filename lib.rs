#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Address, Symbol, symbol_short, log,
};

// Contract struct
#[contract]
pub struct TaskEscrow;

// Escrow state
#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub client: Address,
    pub freelancer: Address,
    pub amount: i128,
    pub funded: bool,
    pub freelancer_approved: bool,
    pub client_approved: bool,
}

// Storage key
#[contracttype]
pub enum DataKey {
    Escrow(u64),
}

#[contractimpl]
impl TaskEscrow {

    // Create escrow (client defines deal)
    pub fn create_escrow(
        env: Env,
        escrow_id: u64,
        client: Address,
        freelancer: Address,
        amount: i128,
    ) {
        let escrow = Escrow {
            client,
            freelancer,
            amount,
            funded: false,
            freelancer_approved: false,
            client_approved: false,
        };

        env.storage().instance().set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Client deposits funds (locks USDC)
    pub fn deposit(env: Env, escrow_id: u64) {
        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap();

        escrow.funded = true;

        // NOTE: Real implementation would transfer USDC into contract
        log!(&env, "Funds deposited");

        env.storage().instance().set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Freelancer submits work (approval)
    pub fn approve_by_freelancer(env: Env, escrow_id: u64) {
        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap();

        escrow.freelancer_approved = true;

        env.storage().instance().set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Client approves work
    pub fn approve_by_client(env: Env, escrow_id: u64) {
        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap();

        escrow.client_approved = true;

        env.storage().instance().set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Release payment when both approve
    pub fn release_payment(env: Env, escrow_id: u64) -> Address {
        let escrow: Escrow = env
            .storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap();

        if escrow.funded && escrow.freelancer_approved && escrow.client_approved {
            let event = symbol_short!("released");
            env.events().publish((event,), escrow.amount);

            // NOTE: Real implementation would transfer USDC to freelancer
            log!(&env, "Payment released");

            return escrow.freelancer;
        }

        panic!("Conditions not met");
    }
}

