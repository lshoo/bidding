mod tests;

use cosmwasm_std::{Addr, Coin, StdResult};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use crate::{
    contract::instantiate,
    execute,
    msg::{ExecuteMsg, HighestOfBidResp, InstantiateMsg, QueryMsg, TotalBidResp, WinnerResp},
    query, ContractError, CONTRACT_LABEL,
};

pub struct BiddingContract(Addr);

impl BiddingContract {
    pub fn new(owner: Addr) -> Self {
        Self(owner)
    }

    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: u64,
        sender: Addr,
        name: &str,
        tick: u128,
        commission: u128,
    ) -> StdResult<BiddingContract> {
        app.instantiate_contract(
            code_id,
            sender,
            &InstantiateMsg {
                name: name.into(),
                tick,
                commission,
            },
            &[],
            CONTRACT_LABEL,
            None,
        )
        .map_err(|e| e.downcast().unwrap())
        .map(BiddingContract)
    }

    #[track_caller]
    pub fn bid(
        &self,
        app: &mut App,
        sender: Addr,
        send_funds: &[Coin],
    ) -> Result<AppResponse, ContractError> {
        app.execute_contract(sender, self.addr(), &ExecuteMsg::Bidding {}, send_funds)
            .map_err(|e| e.downcast().unwrap())
    }

    #[track_caller]
    pub fn close(&self, app: &mut App, sender: Addr) -> Result<AppResponse, ContractError> {
        app.execute_contract(sender, self.addr(), &ExecuteMsg::Close {}, &[])
            .map_err(|e| e.downcast().unwrap())
    }

    #[track_caller]
    pub fn retract(
        &self,
        app: &mut App,
        sender: Addr,
        receiver: Option<String>,
    ) -> Result<AppResponse, ContractError> {
        app.execute_contract(sender, self.addr(), &ExecuteMsg::Retract { receiver }, &[])
            .map_err(|e| e.downcast().unwrap())
    }

    pub fn query_total_bid(&self, app: &App, addr: String) -> Result<TotalBidResp, ContractError> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::TotalBid { addr })
            .map_err(ContractError::Std)
    }

    pub fn query_winner(&self, app: &App) -> StdResult<WinnerResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Winner {})
    }

    pub fn query_highest_of_bid(&self, app: &App) -> StdResult<HighestOfBidResp> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::HighestOfBid {})
    }

    pub fn query_balance(&self, app: &App, denom: impl Into<String>) -> StdResult<Coin> {
        app.wrap().query_balance(self.addr(), denom)
    }
}

pub fn alice() -> Addr {
    Addr::unchecked("sei18rszd3tmgpjvjwq2qajtmn5jqvtscd2yuygl4z")
}

pub fn bob() -> Addr {
    Addr::unchecked("sei1aan9kqywf4rf274cal0hj6eyly6wu0uv7edxy2")
}

pub fn owner() -> Addr {
    Addr::unchecked("sei1zj6fjsc2gkce878ukzg6g9wy8cl8p554dlggxd")
}

pub fn parent() -> Addr {
    Addr::unchecked("inj1g9v8suckezwx93zypckd4xg03r26h6ejlmsptz")
}
