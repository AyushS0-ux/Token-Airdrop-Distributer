#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Map, Address, Vec, log};

const AIRDROP_COUNT: Symbol = symbol_short!("DROP_CNT");

#[contracttype]
#[derive(Clone)]
pub struct AirdropInfo {
    pub amount: i128,
    pub claimed: bool,
}

#[contract]
pub struct TokenAirdropDistributor;

#[contractimpl]
impl TokenAirdropDistributor {
    // Initialize Airdrop for a list of recipients and amounts
    pub fn init_airdrop(env: Env, recipients: Vec<Address>, amounts: Vec<i128>) {
        assert!(recipients.len() == amounts.len(), "Recipients and amounts must be same length");

        let mut drop_map: Map<Address, AirdropInfo> = Map::new(&env);

        for (addr, amt) in recipients.iter().zip(amounts.iter()) {
            drop_map.set(addr.clone(), AirdropInfo {
                amount: amt.clone(),
                claimed: false,
            });
        }

        env.storage().instance().set(&AIRDROP_COUNT, &drop_map);
        env.storage().instance().extend_ttl(10000, 10000);
        log!(&env, "Airdrop Initialized for {} users", recipients.len());
    }

    // Claim airdrop for the calling user
    pub fn claim_airdrop(env: Env, user: Address) -> i128 {
        let mut map: Map<Address, AirdropInfo> = env.storage().instance().get(&AIRDROP_COUNT).unwrap();
        
        let mut info = map.get(user.clone()).unwrap_or_else(|| panic!("User not eligible"));
        
        if info.claimed {
            panic!("Already claimed!");
        }

        info.claimed = true;
        map.set(user.clone(), info.clone());
        env.storage().instance().set(&AIRDROP_COUNT, &map);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Airdrop claimed by: {}", user);

        info.amount
    }

    // Check airdrop status
    pub fn view_status(env: Env, user: Address) -> AirdropInfo {
        let map: Map<Address, AirdropInfo> = env.storage().instance().get(&AIRDROP_COUNT).unwrap();
        map.get(user).unwrap_or_else(|| panic!("User not registered in airdrop"))
    }
}
