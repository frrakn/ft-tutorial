use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage};

pub mod ft_core;
pub mod metadata;
pub mod storage;

use crate::metadata::*;

/// The image URL for the default icon
const DATA_IMAGE_ICON: &str = "data:image/png;base64,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";

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub metadata: LazyOption<FungibleTokenMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: String::from(FT_METADATA_SPEC),
                name: String::from("DefaultFT"),
                icon: Some(String::from(DATA_IMAGE_ICON)),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        Self {
            metadata: LazyOption::new(StorageKey::Metadata.try_to_vec().unwrap(), Some(&metadata)),
        }
    }
}
