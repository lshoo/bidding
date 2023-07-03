use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    msg::InstantiateMsg,
    state::{State, STATE},
    ContractError,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State::new(info.sender, msg.name, msg.tick);

    STATE.save(deps.storage, &state)?;

    Ok(Response::new())
}

pub mod exec {
    use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response};

    use crate::{
        helper::add_coin,
        msg::ExecuteMsg::{self, *},
        state::{Bid, State, BIDDINGS, STATE},
        ContractError, ATOM_DENOM,
    };

    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            Bidding { spread } => bid(deps, info, spread),
            Close {} => todo!(),
            Retract { receiver } => todo!(),
        }
    }

    pub fn bid(deps: DepsMut, info: MessageInfo, spread: Coin) -> Result<Response, ContractError> {
        let sender = &info.sender;

        validiate_denom(spread.denom.as_str(), ATOM_DENOM)?;

        let mut state = STATE.load(deps.storage)?;
        validiate_sender(sender, &state.owner)?;

        // Update the state if the bidding is valid
        let bid = BIDDINGS.may_load(deps.storage, sender.clone())?;

        let current_bid = update_state(&mut state, sender, bid, &spread)?;

        // save the state and bids
        STATE.save(deps.storage, &state)?;
        BIDDINGS.save(deps.storage, sender.clone(), &current_bid)?;

        let resp = Response::new()
            .add_attribute("action", "bid")
            .add_attribute("sender", sender)
            .add_attribute("spread", spread.amount.to_string());

        Ok(resp)
    }

    pub fn close(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        todo!()
    }

    pub fn retract(
        deps: DepsMut,
        info: MessageInfo,
        receiver: Option<String>,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    pub fn validiate_denom(denom: &str, atom_denom: &str) -> Result<(), ContractError> {
        if denom != atom_denom {
            return Err(ContractError::CoinUnsupportedErr {
                denom: denom.to_string(),
            });
        }

        Ok(())
    }

    pub fn validiate_sender(sender: &Addr, owner: &Addr) -> Result<(), ContractError> {
        if owner == sender {
            return Err(ContractError::Unauthorized {});
        }

        Ok(())
    }

    pub fn update_state(
        state: &mut State,
        sender: &Addr,
        bid: Option<Coin>,
        spread: &Coin,
    ) -> Result<Coin, ContractError> {
        let current_bid = add_coin(&bid.unwrap_or_else(|| Coin::new(0, ATOM_DENOM)), spread)?;

        let current_amount = current_bid.amount;
        let highest_amount = state.highest.as_ref().map(|b| b.bid.amount).unwrap();

        if current_amount > highest_amount {
            let highest = Bid {
                bid: current_bid.clone(),
                bidder: sender.clone(),
            };
            state.highest = Some(highest);

            let total = add_coin(&state.total, spread)?;
            state.total = total;

            Ok(current_bid)
        } else {
            Err(ContractError::BidTooLow {
                less_than: current_bid,
            })
        }
    }
}

pub mod query {
    use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

    use crate::{
        msg::{HighestOfBidResp, QueryMsg, TotalBidResp, WinnerResp},
        state::STATE,
    };
    use QueryMsg::*;

    pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            TotalBid {} => query_total_bid(deps).and_then(|tb| to_binary(&tb)),

            HighestOfBid {} => query_highest_of_bid(deps).and_then(|hb| to_binary(&hb)),

            Winner {} => query_winner(deps).and_then(|w| to_binary(&w)),
        }
    }

    pub fn query_total_bid(deps: Deps) -> StdResult<TotalBidResp> {
        let state = STATE.load(deps.storage)?;
        Ok(TotalBidResp { total: state.total })
    }

    pub fn query_highest_of_bid(deps: Deps) -> StdResult<HighestOfBidResp> {
        let state = STATE.load(deps.storage)?;

        Ok(HighestOfBidResp { bid: state.highest })
    }

    pub fn query_winner(deps: Deps) -> StdResult<WinnerResp> {
        let state = STATE.load(deps.storage)?;

        Ok(WinnerResp {
            winner: state.winner,
        })
    }
}
