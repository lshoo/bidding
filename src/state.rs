
/// Define Bidding contract state and storage item

use cosmwasm_std::{Coin, Addr};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct State {
    // contract owner
    pub owner: Addr, 
    // bidding name       
    pub name: String,
    // total bid
    pub total: Coin,
    // highest bid
    pub highest: Coin,
    // bid status, Opening or Closed, default is Opening
    pub status: BidStatus,
    // winner of bid when the status is Closed
    pub winner: Option<Addr>,
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

