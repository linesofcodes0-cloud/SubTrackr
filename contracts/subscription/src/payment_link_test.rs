#![cfg(test)]

use super::*;
use payment_link::PaymentLinkManager;
use soroban_sdk::{testutils::Address as _, Address, Env, Map, String};

#[test]
fn test_create_link_default_call_sites() {
    let env = Env::default();
    env.mock_all_auths();

    let storage = Address::generate(&env);
    let merchant = Address::generate(&env);
    let token = Address::generate(&env);

    // Call site 1 (line 25 equivalent)
    let link1 = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "Link 1"),
        100,
        token.clone(),
        true,
        10,
        1000,
        String::from_str(&env, "link-1"),
        None::<Map<String, String>>,
    );
    assert_eq!(link1.price, 100);
    assert_eq!(link1.metadata, None);

    // Call site 2 (line 53 equivalent)
    let link2 = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "Link 2"),
        200,
        token.clone(),
        true,
        5,
        2000,
        String::from_str(&env, "link-2"),
        None::<Map<String, String>>,
    );
    assert_eq!(link2.price, 200);

    // Call site 3 (line 80 equivalent)
    let link3 = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "Link 3"),
        300,
        token.clone(),
        true,
        100,
        3000,
        String::from_str(&env, "link-3"),
        None::<Map<String, String>>,
    );
    assert_eq!(link3.price, 300);

    // Call site 4 (line 102 equivalent)
    let link4 = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "Link 4"),
        400,
        token.clone(),
        false,
        0,
        0,
        String::from_str(&env, "link-4"),
        None::<Map<String, String>>,
    );
    assert!(!link4.active);

    // Call site 5 (line 125 equivalent)
    let link5 = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "Link 5"),
        500,
        token.clone(),
        true,
        50,
        5000,
        String::from_str(&env, "link-5"),
        None::<Map<String, String>>,
    );
    assert_eq!(link5.id, 5);

    // Call site 6 (line 151 equivalent)
    let link6 = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "Link 6"),
        600,
        token.clone(),
        true,
        20,
        6000,
        String::from_str(&env, "link-6"),
        None::<Map<String, String>>,
    );
    assert_eq!(link6.price, 600);

    // Call site 7 (line 175 equivalent)
    let link7 = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "Link 7"),
        700,
        token.clone(),
        true,
        30,
        7000,
        String::from_str(&env, "link-7"),
        None::<Map<String, String>>,
    );
    assert_eq!(link7.price, 700);

    // Call site 8 (line 212 equivalent)
    let link8 = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "Link 8"),
        800,
        token.clone(),
        true,
        40,
        8000,
        String::from_str(&env, "link-8"),
        None::<Map<String, String>>,
    );
    assert_eq!(link8.price, 800);
}

#[test]
fn test_create_link_with_metadata() {
    let env = Env::default();
    env.mock_all_auths();

    let storage = Address::generate(&env);
    let merchant = Address::generate(&env);
    let token = Address::generate(&env);

    let mut metadata = Map::new(&env);
    metadata.set(
        String::from_str(&env, "campaign"),
        String::from_str(&env, "summer_sale"),
    );
    metadata.set(
        String::from_str(&env, "source"),
        String::from_str(&env, "newsletter"),
    );

    let link = PaymentLinkManager::create_link(
        &env,
        &storage,
        &merchant,
        String::from_str(&env, "PROMO Link"),
        1500,
        token,
        true,
        100,
        99999,
        String::from_str(&env, "promo-summer"),
        Some(metadata.clone()),
    );

    assert_eq!(link.metadata, Some(metadata.clone()));
    let fetched = PaymentLinkManager::get_link(&env, &storage, link.id).unwrap();
    assert_eq!(fetched.metadata, Some(metadata));
}
