use soroban_sdk::{contracttype, Address, Env, Map, String};
use subtrackr_types::StorageKey;

use crate::{storage_instance_get, storage_instance_set, storage_persistent_get, storage_persistent_set};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PaymentLink {
    pub id: u64,
    pub merchant: Address,
    pub title: String,
    pub price: i128,
    pub token: Address,
    pub active: bool,
    pub max_uses: u32,
    pub expires_at: u64,
    pub custom_url: String,
    pub metadata: Option<Map<String, String>>,
    pub created_at: u64,
}

pub struct PaymentLinkManager;

impl PaymentLinkManager {
    /// Creates a new payment link with 9 parameters including `metadata`.
    pub fn create_link(
        env: &Env,
        storage: &Address,
        merchant: &Address,
        title: String,
        price: i128,
        token: Address,
        active: bool,
        max_uses: u32,
        expires_at: u64,
        custom_url: String,
        metadata: Option<Map<String, String>>,
    ) -> PaymentLink {
        merchant.require_auth();
        assert!(price > 0, "Price must be positive");

        let mut count: u64 = storage_instance_get(env, storage, StorageKey::PlanCount).unwrap_or(0);
        count += 1;

        let link = PaymentLink {
            id: count,
            merchant: merchant.clone(),
            title,
            price,
            token,
            active,
            max_uses,
            expires_at,
            custom_url,
            metadata,
            created_at: env.ledger().timestamp(),
        };

        storage_persistent_set(env, storage, StorageKey::Plan(count), link.clone());
        storage_instance_set(env, storage, StorageKey::PlanCount, count);

        link
    }

    pub fn get_link(env: &Env, storage: &Address, id: u64) -> Option<PaymentLink> {
        storage_persistent_get(env, storage, StorageKey::Plan(id))
    }
}
