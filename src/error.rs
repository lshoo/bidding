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
    BidTooLow { less_than: Coin },

    #[error("No bidder")]
    NoBidder {},

    #[error("Coin not same: {first} = {second}")]
    CoinOperationErr { first: String, second: String },

    #[error("Coin not supported: {denom}")]
    CoinSupportedOnlyErr { denom: String },
}
