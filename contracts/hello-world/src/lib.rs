#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Symbol, token,
};

const ADMIN: Symbol = symbol_short!("ADMIN");

#[contract]
pub struct CommunityFund;

#[contractimpl]
impl CommunityFund {
    /// Khởi tạo contract (chỉ chạy 1 lần)
    pub fn init(env: Env, admin: Address) {
        // Chỉ admin mới có thể tự khởi tạo
        admin.require_auth();

        // Không cho phép khởi tạo nhiều lần
        if env.storage().instance().has(&ADMIN) {
            panic!("Contract already initialized");
        }

        env.storage().instance().set(&ADMIN, &admin);
    }

    /// Quyên góp token vào quỹ
    pub fn donate(
        env: Env,
        donor: Address,
        token_address: Address,
        amount: i128,
    ) {
        donor.require_auth();

        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }

        let token_client = token::Client::new(&env, &token_address);

        token_client.transfer(
            &donor,
            &env.current_contract_address(),
            &amount,
        );
    }

    /// Admin rút tiền khỏi quỹ
    pub fn withdraw(
        env: Env,
        token_address: Address,
        amount: i128,
        destination: Address,
    ) {
        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }

        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .expect("Contract not initialized");

        admin.require_auth();

        let token_client = token::Client::new(&env, &token_address);

        token_client.transfer(
            &env.current_contract_address(),
            &destination,
            &amount,
        );
    }

    /// Trả về địa chỉ admin
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&ADMIN)
            .expect("Contract not initialized")
    }
}