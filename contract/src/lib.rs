use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::env::sha256;
use near_sdk::json_types::ValidAccountId;
use near_sdk::json_types::U128;
use near_sdk::{ext_contract, Balance, Promise};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use serde::Serialize;

pub static GAS: u64 = 10_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct NearTrustTransaction {
    from_account_id: AccountId,
    amount: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct NftTrustTransaction {
    from_account_id: AccountId,
    nft_contract_id: AccountId,
    nft_token_id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct FtTrustTransaction {
    from_account_id: AccountId,
    ft_contract_id: AccountId,
    ft_amount: Balance,
}

#[ext_contract(nft_contract)]
pub trait NftContract {
    fn nft_transfer(&self, receiver_id: String, token_id: String);
}

#[ext_contract(ft_contract)]
pub trait FtContract {
    fn ft_transfer(&self, receiver_id: String, amount: U128);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    near_trusts: UnorderedMap<String, NearTrustTransaction>,
    ft_trusts: UnorderedMap<String, FtTrustTransaction>,
    nft_trusts: UnorderedMap<String, NftTrustTransaction>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            near_trusts: UnorderedMap::new(b"n".to_vec()),
            ft_trusts: UnorderedMap::new(b"ft".to_vec()),
            nft_trusts: UnorderedMap::new(b"nft".to_vec()),
        }
    }

    pub fn receive_transfer(
        &mut self,
        request_id: String,
        key: String,
        kind: u8,
        account_id: ValidAccountId,
    ) {
        let key_b = bs58::decode(key).into_vec().unwrap();
        let request_id_b = bs58::decode(request_id.clone()).into_vec().unwrap();
        assert_eq!(sha256(key_b.as_ref()), request_id_b, "Incorrect key");

        match kind {
            1=> {
                let nt = self.near_trusts.get(&request_id).unwrap();
                Promise::new(account_id.clone().to_string()).transfer(nt.amount);
                self.near_trusts.remove(&request_id);
            },
            2=> {
                let nft = self.nft_trusts.get(&request_id).unwrap();

                env::log(format!("Send NFT {}", nft.nft_token_id).as_bytes());
                nft_contract::nft_transfer(
                    account_id.clone().to_string(),
                    nft.nft_token_id,
                    &nft.nft_contract_id,
                    1,
                    GAS,
                );
                self.nft_trusts.remove(&request_id);
            },
            3=>{
                let ft = self.ft_trusts.get(&request_id).unwrap();

                env::log(format!("Send FT {}", ft.ft_contract_id).as_bytes());
                ft_contract::ft_transfer(
                    account_id.clone().to_string(),
                    U128(ft.ft_amount),
                    &ft.ft_contract_id,
                    1,
                    GAS,
                );
                self.ft_trusts.remove(&request_id);

            },
            _ => panic!()
        }
    }

    #[payable]
    pub fn send_near(&mut self, request_id: String) {
        self.near_trusts.insert(&request_id, &NearTrustTransaction {
            from_account_id: env::predecessor_account_id(),
            amount: env::attached_deposit(),
        });
    }

    pub fn nft_on_transfer(
        &mut self,
        sender_id: String,
        previous_owner_id: String,
        token_id: String,
        msg: String,
    ) -> bool {
        env::log(format!("Send NFT {} from {}", &token_id, &previous_owner_id).as_bytes());
            self.nft_trusts.insert(&msg, &NftTrustTransaction {
                from_account_id: previous_owner_id,
                nft_contract_id: env::predecessor_account_id(),
                nft_token_id: token_id,
            });
        false
    }

    pub fn ft_on_transfer(&mut self, sender_id: String, amount: U128, msg: String) -> U128 {
        env::log(
            format!(
                "Send FT {} from {}",
                &env::predecessor_account_id(),
                &sender_id
            )
            .as_bytes(),
        );
        self.ft_trusts.insert(&msg, &FtTrustTransaction {
            from_account_id: sender_id,
            ft_contract_id: env::predecessor_account_id(),
            ft_amount: amount.into(),
        });
        U128(0)
    }

    pub fn get_request(&self, request_id: String) -> Option<NearTrustTransaction> {
        self.near_trusts.get(&request_id)
    }

    pub fn get_ft_transfers(&self, request_id: String) -> Option<FtTrustTransaction> {
        self.ft_trusts.get(&request_id)
    }

    pub fn get_nft_transfers(&self, request_id: String) -> Option<NftTrustTransaction> {
        self.nft_trusts.get(&request_id)
    }
}
