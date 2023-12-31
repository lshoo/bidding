use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::state::Bid;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub tick: u128,
    pub commission: u128,
}

impl InstantiateMsg {
    pub fn new(name: String, tick: u128, commission: u128) -> Self {
        Self {
            name,
            tick,
            commission,
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    Bidding {},
    Close {},
    Retract { receiver: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TotalBidResp)]
    TotalBid { addr: String },
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
