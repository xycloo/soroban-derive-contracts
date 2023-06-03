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

Below we test both `hello()` and the simple token interface:

```rust
use soroban_sdk::{testutils::Address as _, vec, Address, Env, IntoVal, Symbol};

mod contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_hello_world_contract.wasm"
    );
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(&None, contract::WASM);
    let client = contract::Client::new(&env, &contract_id);

    let words = client.hello(&Symbol::short("Dev"));
    assert_eq!(
        words,
        vec![&env, Symbol::short("Hello"), Symbol::short("Dev"),]
    );
}

fn create_token<'a>(e: &Env, admin: &Address) -> contract::Client<'a> {
    let token = contract::Client::new(e, &e.register_contract_wasm(None, contract::WASM));
    token.initialize(admin, &7, &"name".into_val(e), &"symbol".into_val(e));
    token
}

#[test]
fn test_simple_token() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = Address::random(&e);
    let admin2 = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let token = create_token(&e, &admin1);

    token.mint(&user1, &1000);
    assert_eq!(
        e.auths(),
        [(
            admin1.clone(),
            token.address.clone(),
            Symbol::short("mint"),
            (&user1, 1000_i128).into_val(&e),
        )]
    );
    assert_eq!(token.balance(&user1), 1000);

    token.mint(&user3, &1000);

    token.transfer(&user1, &user2, &600);
    assert_eq!(
        e.auths(),
        [(
            user1.clone(),
            token.address.clone(),
            Symbol::short("transfer"),
            (&user1, &user2, 600_i128).into_val(&e),
        )]
    );
    assert_eq!(token.balance(&user1), 400);
    assert_eq!(token.balance(&user2), 600);

    token.transfer(&user1, &user3, &300);
    assert_eq!(token.balance(&user1), 100);
    assert_eq!(token.balance(&user3), 1300);

    token.set_admin(&admin2);
    assert_eq!(
        e.auths(),
        [(
            admin1.clone(),
            token.address.clone(),
            Symbol::short("set_admin"),
            (&admin2,).into_val(&e), //THIS DOESN'T WORK
        )]
    );

    token.set_authorized(&user2, &false);
    assert_eq!(
        e.auths(),
        [(
            admin2.clone(),
            token.address.clone(),
            Symbol::new(&e, "set_authorized"),
            (&user2, false).into_val(&e),
        )]
    );
    assert_eq!(token.authorized(&user2), false);

    token.set_authorized(&user3, &true);
    assert_eq!(token.authorized(&user3), true);

    token.clawback(&user3, &100);
    assert_eq!(
        e.auths(),
        [(
            admin2.clone(),
            token.address.clone(),
            Symbol::short("clawback"),
            (&user3, 100_i128).into_val(&e),
        )]
    );
    assert_eq!(token.balance(&user3), 1200);
}
```


## Limitations

This is not the inheritance pattern that solidity offers but it's a start and will hopefully inspire using derive macros more. Currently it's not much useful apart from very specific use cases, but it will become more interesting once the token interface is split into different interfaces where there may be whole traits that don't need to be different from the standard implementation.

If you have any questions or suggestions, please open an issue on this repo.
