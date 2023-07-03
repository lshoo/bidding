use cosmwasm_std::{coin, Coin};

use crate::{ContractError, ATOM_DENOM};

pub fn collect_coins(coins: &[Coin], denom: &str) -> Result<Coin, ContractError> {
    validiate_denom(coins, denom)?;

    Ok(coins
        .iter()
        .fold(coin(0, denom), |acc, coin| add_coin(&acc, coin).unwrap()))
}

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

pub fn validiate_denom(coins: &[Coin], denom: &str) -> Result<(), ContractError> {
    if coins.iter().any(|c| c.denom != denom) {
        return Err(ContractError::CoinSupportedOnlyErr {
            denom: denom.into(),
        });
    }

    Ok(())
}

pub fn ten_atom() -> Coin {
    Coin::new(10, ATOM_DENOM)
}

pub fn zero_atom() -> Coin {
    Coin::new(0, ATOM_DENOM)
}
