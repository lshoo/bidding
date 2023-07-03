use cosmwasm_std::Coin;

use crate::ContractError;

pub fn add_coin(first: &Coin, second: &Coin) -> Result<Coin, ContractError> {
    if first.denom == second.denom {
        Ok(Coin {
            amount: first.amount + second.amount,
            denom: first.denom.clone(),
        })
    } else {
        Err(ContractError::CoinOperationErr {
            first: first.denom.clone(),
            second: second.denom.clone(),
        })
    }
}
