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
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

    use crate::{msg::ExecuteMsg, ContractError};

    pub fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Ok(Response::new())
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
