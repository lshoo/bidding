use cosmwasm_std::{coin, DepsMut, Env, MessageInfo, Response};

use crate::{
    msg::InstantiateMsg,
    state::{State, STATE},
    ContractError, ATOM_DENOM,
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

    let state = State::new(
        info.sender,
        msg.name,
        coin(msg.tick, ATOM_DENOM),
        coin(msg.commission, ATOM_DENOM),
    );

    STATE.save(deps.storage, &state)?;

    Ok(Response::new())
}

pub mod exec {
    use cosmwasm_std::{coin, Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response};

    use crate::{
        helper::{add_coin, collect_coins},
        msg::ExecuteMsg::{self, *},
        state::{Bid, BidStatus, State, BIDDINGS, STATE},
        ContractError, ATOM_DENOM,
    };

    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            Bidding {} => bid(deps, info),
            Close {} => close(deps, env, info),
            Retract { receiver } => retract(deps, env, info, receiver),
        }
    }

    pub fn bid(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let sender = &info.sender;

        let funds = &info.funds;
        validiate_denom(funds, ATOM_DENOM)?;

        let mut state = STATE.load(deps.storage)?;
        can_bid(sender, &state.owner)?;

        // Update the state if the bidding is valid
        let bid = BIDDINGS.may_load(deps.storage, sender.clone())?;

        let spread = collect_coins(funds, ATOM_DENOM)?;

        validiate_bid(&state, &spread)?;

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
        let sender = info.sender;
        let mut state = STATE.load(deps.storage)?;

        validiate_owner(&sender, &state.owner)?;

        let highest = state.highest.as_ref();

        state.status = BidStatus::Closed {};
        state.winner = highest.map(|bid| bid.bidder.clone());

        let highest_coin: Vec<_> = highest.map(|bid| bid.bid.clone()).into_iter().collect();

        STATE.save(deps.storage, &state)?;

        let contract_balances = deps.querier.query_all_balances(env.contract.address)?;

        validiate_balances(&contract_balances, &highest_coin)?;

        // transfer funds to owner
        let bank_msg = BankMsg::Send {
            to_address: sender.to_string(),
            amount: highest_coin,
        };

        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "close")
            .add_attribute("sender", sender);

        Ok(resp)
    }

    pub fn retract(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        receiver: Option<String>,
    ) -> Result<Response, ContractError> {
        let sender = info.sender;

        let state = STATE.load(deps.storage)?;

        can_retract(&state, &sender)?;

        let owner = state.owner;

        can_bid(&sender, &owner)?;

        let receiver = &receiver
            .as_ref()
            .map(Addr::unchecked)
            .unwrap_or(sender.clone());
        let bid = BIDDINGS.may_load(deps.storage, receiver.clone())?;

        let resp = if let Some(bid) = bid {
            if bid.amount.is_zero() {
                return Err(ContractError::Unauthorized {});
            }

            let bids = vec![coin(
                bid.amount
                    .checked_sub(state.commission.amount)
                    .unwrap()
                    .u128(),
                ATOM_DENOM,
            )];

            let contract_balances = deps.querier.query_all_balances(env.contract.address)?;
            validiate_balances(&contract_balances, &bids)?;

            let bank_msg = BankMsg::Send {
                to_address: receiver.to_string(),
                amount: bids,
            };

            Response::new().add_message(bank_msg)
        } else {
            Response::new()
        }
        .add_attribute("action", "retract")
        .add_attribute("sender", sender)
        .add_attribute("receiver", receiver);

        Ok(resp)
    }

    pub fn validiate_bid(state: &State, spread: &Coin) -> Result<(), ContractError> {
        if spread.amount < state.tick.amount || spread.amount < state.commission.amount {
            return Err(ContractError::InvalidBidErr {
                total_bid: spread.clone(),
            });
        }

        Ok(())
    }

    pub fn validiate_denom(denom: &[Coin], atom_denom: &str) -> Result<(), ContractError> {
        if denom.iter().any(|c| c.denom != atom_denom) {
            return Err(ContractError::CoinSupportedOnlyErr {
                denom: atom_denom.into(),
            });
        }

        Ok(())
    }

    pub fn can_bid(sender: &Addr, owner: &Addr) -> Result<(), ContractError> {
        if is_owner(sender, owner) {
            return Err(ContractError::Unauthorized {});
        }

        Ok(())
    }

    pub fn validiate_owner(sender: &Addr, owner: &Addr) -> Result<(), ContractError> {
        if !is_owner(sender, owner) {
            return Err(ContractError::Unauthorized {});
        }

        Ok(())
    }

    pub fn is_owner(sender: &Addr, owner: &Addr) -> bool {
        owner == sender
    }

    pub fn validiate_balances(
        contract_balances: &[Coin],
        highest_coin: &[Coin],
    ) -> Result<(), ContractError> {
        let contract_total = collect_coins(contract_balances, ATOM_DENOM)?;
        let highest_total = collect_coins(highest_coin, ATOM_DENOM)?;

        if contract_total.amount >= highest_total.amount {
            Ok(())
        } else {
            Err(ContractError::ContractBalanceInvalidErr {
                amount: contract_total,
            })
        }
    }

    // Owner and winner can't retract
    pub fn can_retract(state: &State, sender: &Addr) -> Result<(), ContractError> {
        if state.owner == sender
            || state.winner == Some(sender.clone())
            || !state.status.is_closed()
        {
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
        let highest_amount = state
            .highest
            .as_ref()
            .map(|b| b.bid.amount)
            .unwrap_or_default();

        if current_amount > highest_amount {
            let highest = Bid {
                bid: current_bid.clone(),
                bidder: sender.clone(),
            };

            state.highest = Some(highest);

            Ok(current_bid)
        } else {
            Err(ContractError::BidTooLowErr {
                less_than: current_bid,
            })
        }
    }
}

pub mod query {
    use cosmwasm_std::{coin, to_binary, Addr, Binary, Deps, Env, StdResult};

    use crate::{
        msg::{HighestOfBidResp, QueryMsg, TotalBidResp, WinnerResp},
        state::{BIDDINGS, STATE},
        ATOM_DENOM,
    };
    use QueryMsg::*;

    pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            TotalBid { addr } => query_total_bid(deps, &addr).and_then(|tb| to_binary(&tb)),

            HighestOfBid {} => query_highest_of_bid(deps).and_then(|hb| to_binary(&hb)),

            Winner {} => query_winner(deps).and_then(|w| to_binary(&w)),
        }
    }

    pub fn query_total_bid(deps: Deps, sender: &str) -> StdResult<TotalBidResp> {
        let bid = BIDDINGS.may_load(deps.storage, Addr::unchecked(sender))?;

        if let Some(bid) = bid {
            let state = STATE.load(deps.storage)?;

            Ok(TotalBidResp {
                total: coin(
                    bid.amount.checked_sub(state.commission.amount)?.u128(),
                    ATOM_DENOM,
                ),
            })
        } else {
            Ok(TotalBidResp {
                total: coin(0, ATOM_DENOM),
            })
        }
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
