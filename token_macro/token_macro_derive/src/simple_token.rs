use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn impl_simple_token_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
       use soroban_token_sdk::{TokenMetadata, TokenUtils};

    #[derive(Clone)]
    #[contracttype]
    pub struct AllowanceDataKey {
        pub from: Address,
        pub spender: Address,
    }

    #[derive(Clone)]
    #[contracttype]
    pub enum TokenDataKey {
        Allowance(AllowanceDataKey),
        Balance(Address),
        Nonce(Address),
        State(Address),
        Admin,
    }

    pub fn read_balance(e: &Env, addr: Address) -> i128 {
        let key = TokenDataKey::Balance(addr);
        if let Some(balance) = e.storage().get(&key) {
            balance.unwrap()
        } else {
            0
        }
    }

    fn write_balance(e: &Env, addr: Address, amount: i128) {
        let key = TokenDataKey::Balance(addr);
        e.storage().set(&key, &amount);
    }

    pub fn receive_balance(e: &Env, addr: Address, amount: i128) {
        let balance = read_balance(e, addr.clone());
        if !is_authorized(e, addr.clone()) {
            panic!("can't receive when deauthorized");
        }
        write_balance(e, addr, balance + amount);
    }

    pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
        let balance = read_balance(e, addr.clone());
        if !is_authorized(e, addr.clone()) {
            panic!("can't spend when deauthorized");
        }
        if balance < amount {
            panic!("insufficient balance");
        }
        write_balance(e, addr, balance - amount);
    }

    pub fn is_authorized(e: &Env, addr: Address) -> bool {
        let key = TokenDataKey::State(addr);
        if let Some(state) = e.storage().get(&key) {
            state.unwrap()
        } else {
            true
        }
    }

    pub fn write_authorization(e: &Env, addr: Address, is_authorized: bool) {
        let key = TokenDataKey::State(addr);
        e.storage().set(&key, &is_authorized);
    }

    pub fn read_decimal(e: &Env) -> u32 {
        let util = TokenUtils::new(e);
        util.get_metadata_unchecked().unwrap().decimal
    }

    pub fn read_name(e: &Env) -> Bytes {
        let util = TokenUtils::new(e);
        util.get_metadata_unchecked().unwrap().name
    }

    pub fn read_symbol(e: &Env) -> Bytes {
        let util = TokenUtils::new(e);
        util.get_metadata_unchecked().unwrap().symbol
    }

    pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
        let util = TokenUtils::new(e);
        util.set_metadata(&metadata);
    }

    pub fn has_administrator(e: &Env) -> bool {
        let key = TokenDataKey::Admin;
        e.storage().has(&key)
    }

    pub fn read_administrator(e: &Env) -> Address {
        let key = TokenDataKey::Admin;
        e.storage().get_unchecked(&key).unwrap()
    }

    pub fn write_administrator(e: &Env, id: &Address) {
        let key = TokenDataKey::Admin;
        e.storage().set(&key, id);
    }

    mod event {
        use soroban_sdk::{Address, Env, Symbol};

        pub(crate) fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
            let topics = (Symbol::short("transfer"), from, to);
            e.events().publish(topics, amount);
        }

        pub(crate) fn mint(e: &Env, admin: Address, to: Address, amount: i128) {
            let topics = (Symbol::short("mint"), admin, to);
            e.events().publish(topics, amount);
        }

        pub(crate) fn clawback(e: &Env, admin: Address, from: Address, amount: i128) {
            let topics = (Symbol::short("clawback"), admin, from);
            e.events().publish(topics, amount);
        }

        pub(crate) fn set_authorized(e: &Env, admin: Address, id: Address, authorize: bool) {
            let topics = (Symbol::new(e, "set_authorized"), admin, id);
            e.events().publish(topics, authorize);
        }

        pub(crate) fn set_admin(e: &Env, admin: Address, new_admin: Address) {
            let topics = (Symbol::short("set_admin"), admin);
            e.events().publish(topics, new_admin);
        }

        pub(crate) fn burn(e: &Env, from: Address, amount: i128) {
            let topics = (Symbol::short("burn"), from);
            e.events().publish(topics, amount);
        }
    }

    fn check_nonnegative_amount(amount: i128) {
        if amount < 0 {
            panic!("negative amount is not allowed: {}", amount)
        }
    }

    pub trait SimpleTokenTrait {
        fn initialize(e: Env, admin: Address, decimal: u32, name: Bytes, symbol: Bytes);

        fn balance(e: Env, id: Address) -> i128;

        fn authorized(e: Env, id: Address) -> bool;

        fn transfer(e: Env, from: Address, to: Address, amount: i128);

        fn burn(e: Env, from: Address, amount: i128);

        fn clawback(e: Env, from: Address, amount: i128);

        fn set_authorized(e: Env, id: Address, authorize: bool);

        fn mint(e: Env, to: Address, amount: i128);

        fn set_admin(e: Env, new_admin: Address);

        fn decimals(e: Env) -> u32;

        fn name(e: Env) -> Bytes;

        fn symbol(e: Env) -> Bytes;
    }

    #[contractimpl]
    impl SimpleTokenTrait for #name {
        fn initialize(e: Env, admin: Address, decimal: u32, name: Bytes, symbol: Bytes) {
            if has_administrator(&e) {
                panic!("already initialized")
            }
            write_administrator(&e, &admin);
            if decimal > u8::MAX.into() {
                panic!("Decimal must fit in a u8");
            }

            write_metadata(
                &e,
                TokenMetadata {
                    decimal,
                    name,
                    symbol,
                },
            )
        }

        fn balance(e: Env, id: Address) -> i128 {
            read_balance(&e, id)
        }

        fn authorized(e: Env, id: Address) -> bool {
            is_authorized(&e, id)
        }

        fn transfer(e: Env, from: Address, to: Address, amount: i128) {
            from.require_auth();

            check_nonnegative_amount(amount);
            spend_balance(&e, from.clone(), amount);
            receive_balance(&e, to.clone(), amount);
            event::transfer(&e, from, to, amount);
        }

        fn burn(e: Env, from: Address, amount: i128) {
            from.require_auth();

            check_nonnegative_amount(amount);
            spend_balance(&e, from.clone(), amount);
            event::burn(&e, from, amount);
        }

        fn clawback(e: Env, from: Address, amount: i128) {
            check_nonnegative_amount(amount);
            let admin = read_administrator(&e);
            admin.require_auth();
            spend_balance(&e, from.clone(), amount);
            event::clawback(&e, admin, from, amount);
        }

        fn set_authorized(e: Env, id: Address, authorize: bool) {
            let admin = read_administrator(&e);
            admin.require_auth();
            write_authorization(&e, id.clone(), authorize);
            event::set_authorized(&e, admin, id, authorize);
        }

        fn mint(e: Env, to: Address, amount: i128) {
            check_nonnegative_amount(amount);
            let admin = read_administrator(&e);
            admin.require_auth();
            receive_balance(&e, to.clone(), amount);
            event::mint(&e, admin, to, amount);
        }

        fn set_admin(e: Env, new_admin: Address) {
            let admin = read_administrator(&e);
            admin.require_auth();
            write_administrator(&e, &new_admin);
            event::set_admin(&e, admin, new_admin);
        }

        fn decimals(e: Env) -> u32 {
            read_decimal(&e)
        }

        fn name(e: Env) -> Bytes {
            read_name(&e)
        }

        fn symbol(e: Env) -> Bytes {
            read_symbol(&e)
        }
    }

        };
    gen.into()
}
