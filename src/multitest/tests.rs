use cw_multi_test::App;

use crate::{
    multitest::{sender, zero_atom},
    ContractError,
};

use super::BiddingContract;

#[test]
fn query_total_bid_should_works() {
    let mut app = App::default();

    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(&mut app, code_id, sender(), "bidding", 2).unwrap();

    let resp = contract.query_total_bid(&app).unwrap();

    assert_eq!(resp.total, zero_atom());
}

#[test]
fn query_winner_highest_not_closed_should_works() {
    let mut app = App::default();

    let code_id = BiddingContract::store_code(&mut app);

    let contract = BiddingContract::instantiate(&mut app, code_id, sender(), "bidding", 2).unwrap();

    let resp = contract.query_winner(&app).unwrap();

    assert_eq!(resp.winner, None);

    let resp = contract.query_highest_of_bid(&app).unwrap();
    assert_eq!(resp.bid, None);
}
