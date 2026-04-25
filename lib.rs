#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct PadalaLock;

const LOCKED: Symbol = Symbol::short("LOCK");

#[contractimpl]
impl PadalaLock {

    // Deposit funds into escrow
    pub fn deposit(env: Env, sender: Address, beneficiary: Address, amount: i128) {
        let key = (LOCKED, beneficiary.clone());
        let current: i128 = env.storage().instance().get(&key).unwrap_or(0);
        env.storage().instance().set(&key, &(current + amount));

        env.events().publish((Symbol::short("DEP"), beneficiary), amount);
    }

    // Release funds to school
    pub fn release(env: Env, beneficiary: Address, school: Address) {
        let key = (LOCKED, beneficiary.clone());
        let amount: i128 = env.storage().instance().get(&key).unwrap_or(0);

        env.storage().instance().set(&key, &0);

        env.events().publish((Symbol::short("REL"), school), amount);
    }
}