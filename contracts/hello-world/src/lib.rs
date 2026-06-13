#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    token, Address, Env, String, Symbol,
};

const ADMIN: Symbol = symbol_short!("ADMIN");
const TOTAL: Symbol = symbol_short!("TOTAL");
const NEXT_ID: Symbol = symbol_short!("NEXTID");

#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub receiver: Address,
    pub amount: i128,
    pub yes: u32,
    pub no: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Proposal(u32),
    Voted(u32, Address),
}

#[contract]
pub struct CommunityFund;

#[contractimpl]
impl CommunityFund {

    //=========================
    // INIT
    //=========================
    pub fn init(env: Env, admin: Address) {

        if env.storage().persistent().has(&ADMIN) {
            panic!("Already initialized");
        }

        admin.require_auth();

        env.storage().persistent().set(&ADMIN, &admin);

        env.storage().persistent().set(&TOTAL, &0_i128);

        env.storage().persistent().set(&NEXT_ID, &1_u32);
    }

    //=========================
    // DONATE
    //=========================
    pub fn donate(
        env: Env,
        from: Address,
        token_address: Address,
        amount: i128,
    ) {

        from.require_auth();

        if amount <= 0 {
            panic!("Invalid amount");
        }

        let client = token::Client::new(&env, &token_address);

        client.transfer(
            &from,
            &env.current_contract_address(),
            &amount,
        );

        let mut total: i128 = env
            .storage()
            .persistent()
            .get(&TOTAL)
            .unwrap_or(0);

        total += amount;

        env.storage().persistent().set(&TOTAL, &total);
    }

    //=========================
    // CREATE PROPOSAL
    //=========================
    pub fn create_proposal(
        env: Env,
        title: String,
        receiver: Address,
        amount: i128,
    ) {

        let admin: Address = env
            .storage()
            .persistent()
            .get(&ADMIN)
            .unwrap();

        admin.require_auth();

        if amount <= 0 {
            panic!("Invalid amount");
        }

        let mut next_id: u32 = env
            .storage()
            .persistent()
            .get(&NEXT_ID)
            .unwrap();

        let proposal = Proposal {
            id: next_id,
            title,
            receiver,
            amount,
            yes: 0,
            no: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(next_id), &proposal);

        next_id += 1;

        env.storage()
            .persistent()
            .set(&NEXT_ID, &next_id);
    }

    //=========================
    // VOTE
    //=========================
    pub fn vote(
        env: Env,
        proposal_id: u32,
        voter: Address,
        approve: bool,
    ) {

        voter.require_auth();

        let voted_key = DataKey::Voted(
            proposal_id,
            voter.clone(),
        );

        if env.storage().persistent().has(&voted_key) {
            panic!("Already voted");
        }

        let proposal_key = DataKey::Proposal(proposal_id);

        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&proposal_key)
            .unwrap();

        if approve {
            proposal.yes += 1;
        } else {
            proposal.no += 1;
        }

        env.storage()
            .persistent()
            .set(&proposal_key, &proposal);

        env.storage()
            .persistent()
            .set(&voted_key, &true);
    }
        //=========================
    // GET PROPOSAL
    //=========================
    pub fn get_proposal(
        env: Env,
        proposal_id: u32,
    ) -> Proposal {

        env.storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap()
    }

    //=========================
    // GET TOTAL FUND
    //=========================
    pub fn get_total(env: Env) -> i128 {

        env.storage()
            .persistent()
            .get(&TOTAL)
            .unwrap_or(0)
    }

    //=========================
    // GET ADMIN
    //=========================
    pub fn get_admin(env: Env) -> Address {

        env.storage()
            .persistent()
            .get(&ADMIN)
            .unwrap()
    }
}