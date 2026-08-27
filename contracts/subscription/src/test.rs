#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env};
use subtrackr_types::{ChargeCommitment, Interval, Plan};

#[test]
fn test_mev_commit_reveal_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let proxy = Address::generate(&env);
    let storage = Address::generate(&env);
    let admin = Address::generate(&env);
    let subscriber = Address::generate(&env);
    let merchant = Address::generate(&env);
    let token = Address::generate(&env);

    let contract_id = env.register_contract(None, SubTrackrSubscription);
    let client = SubTrackrSubscriptionClient::new(&env, &contract_id);

    client.initialize(&proxy, &storage, &admin);

    // Set a low threshold to trigger commit-reveal for this test
    client.set_large_charge_threshold(&proxy, &storage, &100);

    // Create plan
    let plan_id = client.create_plan(&proxy, &storage, &merchant, &String::from_str(&env, "Premium"), &500, &token, &Interval::Monthly);

    // Subscribe
    let sub_id = client.subscribe(&proxy, &storage, &subscriber, &plan_id);

    // Setup commit
    let expected_gas_bid: i128 = 1000;
    let is_private_mempool = true;

    let mut payload = Bytes::new(&env);
    payload.append(&Bytes::from_array(&env, &sub_id.to_be_bytes()));
    payload.append(&Bytes::from_array(&env, &expected_gas_bid.to_be_bytes()));
    let is_priv = if is_private_mempool { 1u8 } else { 0u8 };
    payload.append(&Bytes::from_array(&env, &[is_priv]));

    let hash: BytesN<32> = env.crypto().sha256(&payload).into();

    client.commit_charge(&proxy, &storage, &sub_id, &hash);

    // Reveal
    client.reveal_charge(&proxy, &storage, &sub_id, &expected_gas_bid, &is_private_mempool);
    
    // Check it succeeded (no panic)
    // Next attempt without commit should fail
    let res = client.try_reveal_charge(&proxy, &storage, &sub_id, &expected_gas_bid, &is_private_mempool);
    assert!(res.is_err());
}

#[test]
fn test_get_contract_health() {
    let env = Env::default();
    env.mock_all_auths();

    let proxy = Address::generate(&env);
    let storage = Address::generate(&env);
    let admin = Address::generate(&env);

    let contract_id = env.register_contract(None, SubTrackrSubscription);
    let client = SubTrackrSubscriptionClient::new(&env, &contract_id);

    client.initialize(&proxy, &storage, &admin);

    let health = client.get_contract_health(&storage);
    assert_eq!(health.version, String::from_str(&env, "2.0.0"));
    assert_eq!(health.is_paused, false);
    assert_eq!(health.is_creation_paused, false);
    assert_eq!(health.treasury_balance, 0);
    assert_eq!(health.active_payment_count, 0);
    assert_eq!(health.merchant_registry_configured, true);
}

#[test]
fn test_daily_payment_index() {
    let env = Env::default();
    env.mock_all_auths();

    let proxy = Address::generate(&env);
    let storage = Address::generate(&env);
    let admin = Address::generate(&env);
    let merchant = Address::generate(&env);

    let contract_id = env.register_contract(None, SubTrackrSubscription);
    let client = SubTrackrSubscriptionClient::new(&env, &contract_id);

    client.initialize(&proxy, &storage, &admin);

    // Verify revenue analytics with daily payment index date buckets
    let analytics = revenue::get_revenue_analytics_by_period(
        &env,
        &storage,
        &merchant,
        86400,
        0,
        259200, // 3 days
    );
    assert_eq!(analytics.len(), 3);
}

