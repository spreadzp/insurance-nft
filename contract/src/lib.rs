use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::env::block_timestamp;
use near_sdk::env::sha256;
use near_sdk::ext_contract;
use near_sdk::serde::ser::{Serialize, SerializeStruct, Serializer};
use near_sdk::serde_json;
use near_sdk::{env, near_bindgen, PanicOnDefault, Promise, PromiseResult, ReturnData};
use near_sdk::{AccountId, BorshStorageKey, PromiseOrValue};
near_sdk::setup_alloc!();

const MIN_DEPOSIT: u128 = 1_000_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct NftData {
    pub nft_id: String,
    pub nft_contract_address: String,
    pub deposit: u128,
    pub image_hash: String,
    pub media: String,
    pub finish_data: u64,
}

#[ext_contract(ext_nft)]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
trait Contract {
    fn nft_token(&self, token_id: TokenId) -> Option<Token>;
}

impl Serialize for NftData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("NftData", 5)?;
        state.serialize_field("nft_id", &self.nft_id)?;
        state.serialize_field("nft_contract_address", &self.nft_contract_address)?;
        state.serialize_field("deposit", &self.deposit)?;
        state.serialize_field("image_hash", &self.image_hash)?;
        state.serialize_field("media", &self.media)?;
        state.serialize_field("finish_data", &self.finish_data)?;
        state.end()
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Insurance {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    pub insurance_book: LookupMap<String, NftData>,
    pub nft_keys: LookupMap<String, String>, // map?
    pub agents: UnorderedSet<String>,
}
impl Default for Insurance {
    fn default() -> Self {
        Self {
            insurance_book: LookupMap::new(b"i".to_vec()),
            nft_keys: LookupMap::new(b"n".to_vec()),
            agents: UnorderedSet::new(b"a".to_vec()),
        }
    }
}

#[near_bindgen]
impl Insurance {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            insurance_book: LookupMap::new(b"i".to_vec()),
            nft_keys: LookupMap::new(b"n".to_vec()),
            agents: UnorderedSet::new(b"a".to_vec()),
        }
    }

    #[payable]
    pub fn make_new_insurance(
        &mut self,
        contract_address: String,
        nft_id: String,
        media: String,
        image_hash: String,
    ) -> bool {
        let deposit = env::attached_deposit();
        assert!(
            deposit >= MIN_DEPOSIT,
            "The amount of deposit is {} and it should be greater or equal to {}",
            deposit,
            MIN_DEPOSIT
        );
        // let owner = self.get_owner_address_nft(&contract_address, &nft_id);
        // assert!(
        //     format!("{:?}", env::predecessor_account_id()) == format!("{:?}", owner),
        //     "only owner can do deposit",
        // );
        let finish_data = block_timestamp() + 10000000;
        let data_insurance = &NftData {
            nft_id: String::from(&nft_id),
            nft_contract_address: String::from(&contract_address),
            image_hash: image_hash,
            media: String::from(&media),
            deposit: deposit,
            finish_data: finish_data,
        };
        let data_as_string = format!("{:?}", &data_insurance);
        let hasher = sha256(data_as_string.as_bytes());
        let hash_key_1 = &hasher.iter().map(|&c| c as char).collect::<String>();
        // let hash_key = format!("hash_key: {:?}", &hasher);
        // let utf_hash = String::from_utf8_lossy(&hasher);
        let plain_hash = &hash_key_1
            .as_bytes()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();
        self.insurance_book.insert(&plain_hash, &data_insurance);
        let nft_key = format!("{}-{}", contract_address, nft_id);
        self.nft_keys.insert(&nft_key, &plain_hash);
        env::log(
            format!(
                "plain_hash {} &nft_key {} media {}",
                &plain_hash, &nft_key, &media
            )
            .as_bytes(),
        );
        true
    }

    pub fn get_owner_address_nft(&self, contract_address: String, nft_id: String) -> Promise {
        let res = ext_nft::nft_token(
            nft_id,
            &contract_address,
            0, // yocto NEAR to attach
            5_000_000_000_000,
        );
        res
        //.as_return();
        // .then(result => env::log(format!("nft {:#?}", &result).as_bytes()))
        // // env::log(format!("nft {:#?}", &result).as_bytes());
        // .then(match env::promise_result(0) {
        //     PromiseResult::NotReady => unreachable!(),
        //     PromiseResult::Failed => env::panic(b"Unable to make comparison"),
        //     PromiseResult::Successful(result) => {
        //         let balance = near_sdk::serde_json::from_slice::<u8>(&result).unwrap();
        //         let res = &result.iter().map(|&c| c as char).collect::<String>();
        //         env::log(format!("nft {:#?}", &res).as_bytes());
        //         balance
        //     } //}
        // });
        //env::log(format!("length {}" ,env::promise_results_count()).as_bytes());
        // .then(self.callback(
        //     &env::current_account_id(), // this contract's account id
        //     0,                          // yocto NEAR to attach to the callback
        //     5_000_000_000_000,          // gas to attach to the callback
        // ));

        // result_promise
    }

    pub fn callback(&self) -> String {
        let res = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"Unable to make comparison"),
            PromiseResult::Successful(nft) => {
                let nft = near_sdk::serde_json::from_slice::<u8>(&nft).unwrap();
                let data = format!("nft {:#?}", &nft);
                env::log(format!("nft {:#?}", &data).as_bytes());
                data
            }
        };
        res
    }
    pub fn get_owner_address_of_nft(&self, contract_address: String, nft_id: String) {
        let res = self.get_owner_address_nft(contract_address, nft_id);
        // .as_return();
        res.then(self.callback());
        // let str_res = format!("res {:#?}", &res);
        // env::log(str_res.as_bytes());
        // str_res
    }
    pub fn is_insurance_case(
        &self,
        contract_address: &String,
        nft_id: &String,
        current_nft_image_hash: String,
    ) -> bool {
        assert!(
            self.is_expire_insurance(contract_address, nft_id) == false,
            "The date of the insurance is expired"
        );
        let insurance_data = self.get_insurance_data(&contract_address, &nft_id);
        match insurance_data {
            Some(insurance_data) => insurance_data.image_hash != current_nft_image_hash,
            None => false,
        }
    }

    pub fn set_agent(&mut self, agent_address: &String) -> String {
        self.agents.insert(agent_address);
        format!("{}", agent_address)
    }

    pub fn is_agent(&self, agent_address: &String) -> bool {
        let agent = self.agents.iter().find(|ag| ag == agent_address);
        match agent {
            Some(_agent) => true,
            None => false,
        }
    }

    pub fn is_expire_insurance(&self, contract_address: &String, nft_id: &String) -> bool {
        let insurance_data = self.get_insurance_data(&contract_address, &nft_id);
        match insurance_data {
            Some(insurance_data) => insurance_data.finish_data >= block_timestamp(),
            None => false,
        }
    }

    pub fn get_insurance_data(
        &self,
        contract_address: &String,
        nft_id: &String,
    ) -> Option<NftData> {
        match self.get_hash_image_nft(&contract_address, &nft_id) {
            Some(hash) => match self.insurance_book.get(&hash) {
                Some(insurance_data) => Some(insurance_data),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_insurance_item(&self, hash: &String) -> Option<NftData> {
        match self.insurance_book.get(&hash) {
            Some(insurance_data) => {
                let data_as_string = format!("data_as_string: {:?}", &insurance_data);
                env::log(data_as_string.as_bytes());
                Some(insurance_data)
            }
            None => None,
        }
    }

    pub fn get_hash_image_nft(&self, contract_address: &String, nft_id: &String) -> Option<String> {
        let nft_key = format!("{}-{}", &contract_address, &nft_id);
        let hash = self.nft_keys.get(&nft_key);
        match hash {
            Some(hash) => Some(hash),
            None => None,
        }
    }

    pub fn redeem_insurance_payment(
        &mut self,
        contract_address: &String,
        nft_id: &String,
        current_nft_image_hash: String,
    ) -> String {
        "result".to_string()
    }

    // pub fn get_all_books(&mut self)  {
    //     // let books = &self.insurance_book;
    //     // books
    //     // for (key, value) in &self.insurance_book{
    //     //     println!("{}: {}", key, value);
    //     // }
    //     // env::log(
    //     //     format!(
    //     //         "{:?}",
    //     //         self.insurance_book
    //     //     )
    //     //         .as_bytes(),
    //     // );
    // }
}
