use cosmwasm_std::{coin, coins};
use cw_multi_test::App;

use crate::{
    multitest::{alice, owner, zero_atom},
    state::Bid,
    ContractError, ATOM_DENOM,
};

use super::{bob, ten_atom, BiddingContract};

#[test]
fn query_total_bid_should_works() {
    let mut app = App::default();

    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(&mut app, code_id, alice(), "bidding", 2).unwrap();

    let resp = contract.query_total_bid(&app).unwrap();

    assert_eq!(resp.total, zero_atom());
}

#[test]
fn query_winner_highest_not_closed_should_works() {
    let mut app = App::default();

    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(&mut app, code_id, alice(), "bidding", 1).unwrap();

    let resp = contract.query_winner(&app).unwrap();

    assert_eq!(resp.winner, None);

    let resp = contract.query_highest_of_bid(&app).unwrap();
    assert_eq!(resp.bid, None);
}

#[test]
fn bid_should_works() {
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice(), vec![ten_atom()])
            .unwrap();
        router
            .bank
            .init_balance(storage, &bob(), vec![ten_atom()])
            .unwrap();
    });

    let code_id = BiddingContract::store_code(&mut app);
    let contract = BiddingContract::instantiate(&mut app, code_id, owner(), "bidding", 1).unwrap();

    // alice bid first
    contract
        .bid(&mut app, alice(), &coins(1, ATOM_DENOM))
        .unwrap();

    let total_bid = contract.query_total_bid(&app).unwrap();
    assert_eq!(total_bid.total, coin(1, ATOM_DENOM));

    let highest = contract.query_highest_of_bid(&app).unwrap();
    assert_eq!(
        highest.bid,
        Some(Bid {
            bid: coin(1, ATOM_DENOM),
            bidder: alice()
        })
    );

    let winner = contract.query_winner(&app).unwrap();
    assert!(winner.winner.is_none());

    // bob bid second, but fails
    let err = contract
        .bid(&mut app, bob(), &coins(1, ATOM_DENOM))
        .unwrap_err();
    assert_eq!(
        err,
        ContractError::BidTooLow {
            less_than: coin(1, ATOM_DENOM)
        }
    );

    // bob bid again
    contract
        .bid(&mut app, bob(), &coins(2, ATOM_DENOM))
        .unwrap();

    let total_bid = contract.query_total_bid(&app).unwrap();
    assert_eq!(total_bid.total, coin(3, ATOM_DENOM));

    let highest = contract.query_highest_of_bid(&app).unwrap();
    assert_eq!(
        highest.bid,
        Some(Bid {
            bid: coin(2, ATOM_DENOM),
            bidder: bob()
        })
    );

    // bob bid add amount
    contract
        .bid(&mut app, bob(), &coins(2, ATOM_DENOM))
        .unwrap();

    let highest = contract.query_highest_of_bid(&app).unwrap();
    assert_eq!(
        highest.bid,
        Some(Bid {
            bid: coin(4, ATOM_DENOM),
            bidder: bob()
        })
    );

    // alice bid again
    let err = contract
        .bid(&mut app, alice(), &coins(3, ATOM_DENOM))
        .unwrap_err();
    assert_eq!(
        err,
        ContractError::BidTooLow {
            less_than: coin(4, ATOM_DENOM)
        }
    );

    contract
        .bid(&mut app, alice(), &coins(4, ATOM_DENOM))
        .unwrap();

    let highest = contract.query_highest_of_bid(&app).unwrap();
    assert_eq!(
        highest.bid,
        Some(Bid {
            bid: coin(5, ATOM_DENOM),
            bidder: alice()
        })
    );

    let total = contract.query_total_bid(&app).unwrap();
    assert_eq!(total.total, coin(9, ATOM_DENOM));

    let balance = contract.query_balance(&app, ATOM_DENOM).unwrap();
    assert_eq!(balance, coin(9, ATOM_DENOM));
}

#[test]
fn close_bid_retract_should_works() {
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice(), vec![ten_atom()])
            .unwrap();
        router
            .bank
            .init_balance(storage, &bob(), vec![ten_atom()])
            .unwrap();
    });

    let code_id = BiddingContract::store_code(&mut app);
    let contract = BiddingContract::instantiate(&mut app, code_id, owner(), "bidding", 1).unwrap();

    // alice bid first
    contract
        .bid(&mut app, alice(), &coins(1, ATOM_DENOM))
        .unwrap();

    // bob bid second
    contract
        .bid(&mut app, bob(), &coins(2, ATOM_DENOM))
        .unwrap();

    // alice bid again
    contract
        .bid(&mut app, alice(), &coins(4, ATOM_DENOM))
        .unwrap();

    let highest = contract.query_highest_of_bid(&app).unwrap();
    assert_eq!(
        highest.bid,
        Some(Bid {
            bid: coin(5, ATOM_DENOM),
            bidder: alice()
        })
    );

    // check the contract total and balance
    let total = contract.query_total_bid(&app).unwrap();
    assert_eq!(total.total, coin(7, ATOM_DENOM));

    let balance = contract.query_balance(&app, ATOM_DENOM).unwrap();
    assert_eq!(balance, coin(7, ATOM_DENOM));

    // owner close the bid
    contract.close(&mut app, owner()).unwrap();

    let balance = contract.query_balance(&app, ATOM_DENOM).unwrap();
    assert_eq!(balance, coin(2, ATOM_DENOM));

    let owner_balance = app.wrap().query_balance(owner(), ATOM_DENOM).unwrap();
    assert_eq!(owner_balance, coin(5, ATOM_DENOM));

    // retract funds
    contract.retract(&mut app, bob(), None).unwrap();

    let bob_balance = app.wrap().query_balance(bob(), ATOM_DENOM).unwrap();
    assert_eq!(bob_balance, coin(10, ATOM_DENOM));

    let err = contract.retract(&mut app, alice(), None).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // TODO handle commission
}
