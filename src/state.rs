/// Define Bidding contract state and storage item
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

use crate::DENOM_ATOM;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct State {
    // contract owner
    pub owner: Addr,
    // bidding name
    pub name: String,
    // total bid
    pub total: Coin,
    // highest bid
    pub highest: Option<Bid>,
    // bid status, Opening or Closed, default is Opening
    pub status: BidStatus,
    // winner of bid when the status is Closed
    pub winner: Option<Addr>,
}

impl State {
    pub fn new(owner: Addr, name: String) -> Self {
        Self {
            owner,
            name,
            total: Coin::new(0, DENOM_ATOM),
            highest: None,
            status: BidStatus::default(),
            winner: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Bid {
    bid: Coin,
    bidder: Addr,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum BidStatus {
    Opening {},
    Closed {},
}

impl Default for BidStatus {
    fn default() -> Self {
        Self::Opening {}
    }
}

// Define the state storage
pub const STATE: Item<State> = Item::new("state");
pub const INDIVIDUAL_BIDDING: Map<Addr, u64> = Map::new("individual_bidding");
