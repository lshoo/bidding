use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Bid already Closed")]
    BidAlreadyClosed {},

    #[error("Bid is opening")]
    BidIsOpening {},

    #[error("The bid is {less_than} lower than the highest price")]
    BidTooLowErr { less_than: Coin },

    #[error("Coin not same: {first} = {second}")]
    CoinOperationErr { first: String, second: String },

    #[error("Coin not supported: {denom}")]
    CoinSupportedOnlyErr { denom: String },

    #[error("Contract balance invalid: {amount}")]
    ContractBalanceInvalidErr { amount: Coin },

    #[error("Bid amount must be greater thant tick and commission")]
    InvalidBidErr { total_bid: Coin },
}
