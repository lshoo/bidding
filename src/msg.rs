use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
}

impl InstantiateMsg {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cw_serde]
pub enum ExecuteMsg {}

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
    highest: Coin,
    bidder: Addr,
}

#[cw_serde]
pub struct WinnerResp {
    winner: Addr,
}
