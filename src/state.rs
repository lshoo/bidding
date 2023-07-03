use cosmwasm_schema::cw_serde;
/// Define Bidding contract state and storage item
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct State {
    // contract owner
    pub owner: Addr,
    // bidding name
    pub name: String,
    // bid tick
    pub tick: Coin,
    // commission
    pub commission: Coin,
    // bid status, Opening or Closed, default is Opening
    pub status: BidStatus,
    // highest bid
    pub highest: Option<Bid>,
    // winner of bid when the status is Closed
    pub winner: Option<Addr>,
}

impl State {
    pub fn new(owner: Addr, name: String, tick: Coin, commission: Coin) -> Self {
        Self {
            owner,
            name,
            tick,
            commission,
            status: BidStatus::default(),
            highest: None,
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

impl BidStatus {
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed {})
    }
}

// Define the state storage
pub const STATE: Item<State> = Item::new("state");
pub const BIDDINGS: Map<Addr, Coin> = Map::new("bids");
