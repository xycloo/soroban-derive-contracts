# Soroban Derive Contracts

This repo is an experimental workspace which contains derive macros to inherit implementations from standard contracts such as the token standard, nft standard, etc.

## Progress so far

Currently the `hello_token_contract` contract uses the `SimpleTokenMacro` derive macro to derive the implementation of a simplified version of the standard token interface.

You can try the macro as follows:

```rust
#![no_std]
use soroban_sdk::{contractimpl, contracttype, vec, Address, Bytes, Env, Symbol, Vec};
use token_macro_derive::SimpleTokenMacro;

// create a new struct in the contract and derive SimpleToken
#[derive(SimpleTokenMacro)]
pub struct SimpleToken;

pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), to]
    }
}
```

When compiled to WASM, this code will bundle into one contract that does both `hello()` and the simple token.


## Limitations

This is not the inheritance pattern that solidity offers but it's a start and will hopefully inspire using derive macros more. Currently it's not much useful apart from very specific use cases, but it will become more interesting once the token interface is split into different interfaces where there may be whole traits that don't need to be different from the standard implementation.

If you have any questions or suggestions, please open an issue on this repo.
