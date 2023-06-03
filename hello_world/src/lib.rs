#![no_std]
use soroban_sdk::{contractimpl, contracttype, vec, Address, Bytes, Env, Symbol, Vec};
use token_macro_derive::SimpleTokenMacro;

#[derive(SimpleTokenMacro)]
pub struct SimpleToken;

pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), to]
    }
}

mod test;
