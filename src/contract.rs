#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::error::ContractError;
use crate::msg::{EmployeeInfoResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{store, store_query, Employee};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let employee = Employee {
        id: "0".to_string(),
        name: msg.name,
        career: msg.career,
        domain: msg.domain,
        group: msg.group,
        number_of_course: msg.number_of_course,
        department: msg.department,
        position: msg.position,
    };
    let key = employee.id.as_bytes();
    store(deps.storage).save(key, &employee)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddNew {
            id,
            name,
            career,
            domain,
            group,
            number_of_course,
            department,
            position,
        } => add_new(
            deps,
            id,
            name,
            career,
            domain,
            group,
            number_of_course,
            department,
            position,
        ),
    }
}

pub fn add_new(
    deps: DepsMut,
    id: String,
    name: String,
    career: String,
    domain: String,
    group: String,
    number_of_course: i32,
    department: String,
    position: String,
) -> Result<Response, ContractError> {
    let employee = Employee {
        id,
        name,
        career,
        domain,
        group,
        number_of_course,
        department,
        position,
    };
    let key = employee.id.as_bytes();
    if (store(deps.storage).may_load(key)?).is_some() {
        // id is already taken
        return Err(ContractError::IdTaken { id: employee.id });
    }
    store(deps.storage).save(key, &employee)?;
    Ok(Response::new()
        .add_attribute("method", "add_new")
        .add_attribute("id", employee.id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEmployee { id } => query_employee(deps, id),
    }
}

fn query_employee(deps: Deps, id: String) -> StdResult<Binary> {
    let key = id.as_bytes();
    let employee = match store_query(deps.storage).may_load(key)? {
        Some(employee) => Some(employee),
        None => return Err(StdError::generic_err("Employee does not exist")),
    };

    let resp = EmployeeInfoResponse { employee };
    to_binary(&resp)
}
