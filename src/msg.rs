use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::state::Bid;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub tick: u64,
}

impl InstantiateMsg {
    pub fn new(name: String, tick: u64) -> Self {
        Self { name, tick }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    Bidding { spread: Coin },
    Close {},
    Retract { receiver: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TotalBidResp)]
    TotalBid {},
    #[returns(HighestOfBidResp)]
    HighestOfBid {},
    #[returns(WinnerResp)]
    Winner {},
}

#[cw_serde]
pub struct TotalBidResp {
    pub total: Coin,
}

#[cw_serde]
pub struct HighestOfBidResp {
    pub bid: Option<Bid>,
}

#[cw_serde]
pub struct WinnerResp {
    pub winner: Option<Addr>,
}
