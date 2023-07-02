use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Bid is Closed")]
    BidIsClosed {},

    #[error("Bid too low {bid}")]
    BidTooLow { bid: u64 },
}
