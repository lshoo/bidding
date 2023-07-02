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

    let state = State::new(info.sender, msg.name);

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
        unimplemented!()
    }
}

pub mod query {
    use cosmwasm_std::{Binary, Deps, Env, StdResult};

    use crate::msg::QueryMsg;

    pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
        unimplemented!()
    }
}
