use cosmwasm_schema::cw_serde;
/// Define Bidding contract state and storage item
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

use crate::ATOM_DENOM;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct State {
    // contract owner
    pub owner: Addr,
    // bidding name
    pub name: String,
    // bid tick
    pub tick: u64,
    // total bid
    pub total: Coin,
    // bid status, Opening or Closed, default is Opening
    pub status: BidStatus,
    // highest bid
    pub highest: Option<Bid>,
    // winner of bid when the status is Closed
    pub winner: Option<Addr>,
}

impl State {
    pub fn new(owner: Addr, name: String, tick: u64) -> Self {
        Self {
            owner,
            name,
            tick,
            total: Coin::new(0, ATOM_DENOM),
            highest: None,
            status: BidStatus::default(),
            winner: None,
        }
    }
}

#[cw_serde]
pub struct Bid {
    pub bid: Coin,
    pub bidder: Addr,
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
pub const BIDDINGS: Map<Addr, Coin> = Map::new("bids");
