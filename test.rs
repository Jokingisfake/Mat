#![cfg(test)]
use soroban_sdk::{Env, Address};
use crate::PadalaLock;

#[test]
fn test_happy_path() {
    let env = Env::default();
    let contract = PadalaLock;
    let user = Address::random(&env);
    let school = Address::random(&env);

    contract.deposit(env.clone(), user.clone(), user.clone(), 100);
    contract.release(env.clone(), user.clone(), school.clone());
}

#[test]
fn test_no_funds() {
    let env = Env::default();
    let contract = PadalaLock;
    let user = Address::random(&env);
    let school = Address::random(&env);

    contract.release(env.clone(), user.clone(), school.clone());
}

#[test]
fn test_state_reset() {
    let env = Env::default();
    let contract = PadalaLock;
    let user = Address::random(&env);
    let school = Address::random(&env);

    contract.deposit(env.clone(), user.clone(), user.clone(), 50);
    contract.release(env.clone(), user.clone(), school.clone());
}

#[test]
fn test_multiple_deposits() {
    let env = Env::default();
    let contract = PadalaLock;
    let user = Address::random(&env);

    contract.deposit(env.clone(), user.clone(), user.clone(), 50);
    contract.deposit(env.clone(), user.clone(), user.clone(), 50);
}

#[test]
fn test_zero_deposit() {
    let env = Env::default();
    let contract = PadalaLock;
    let user = Address::random(&env);

    contract.deposit(env.clone(), user.clone(), user.clone(), 0);
}