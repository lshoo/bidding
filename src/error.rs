use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Bid already Closed")]
    BidAlreadyClosed {},

    #[error("Bid is opening")]
    BidIsOpening {},

    #[error("Bid too low {bid}")]
    BidTooLow { bid: u64 },

    #[error("No bidder")]
    NoBidder {},
}
