use alloc::{string::String, vec};

use casper_erc20::Address;

use casper_types::{
    URef, U512, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

pub const PROTOCOL_FEE: &str = "protocol_fee";
pub const PROTOCOL_LOCK_PERIOD: &str = "protocol_lock_period";
pub const ARG_TMP_PURSE: &str = "tmp_purse";
pub const ARG_AMOUNT: &str = "amount";

pub const ENTRY_POINT_INIT: &str = "initialize_contract";
const ENTRY_POINT_BALANCE_OF: &str = "balance_of";
const ENTRY_POINT_GET_STREAM: &str = "get_stream";
const ENTRY_POINT_CREATE_STREAM: &str = "create_stream";
const ENTRY_POINT_WITHDRAW_FROM_STREAM: &str = "withdraw_from_stream";
const ENTRY_POINT_CANCEL_STREAM: &str = "cancel_stream";

pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_BALANCE_OF),
        vec![
            Parameter::new(ARG_TMP_PURSE, URef::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn get_stream() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_GET_STREAM),
        vec![
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn create_stream() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_CREATE_STREAM),
        vec![
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn withdraw_from_stream() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_WITHDRAW_FROM_STREAM),
        vec![
            Parameter::new(PROTOCOL_LOCK_PERIOD, U512::cl_type()),
            ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn cancel_stream() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_CANCEL_STREAM),
        vec![
            Parameter::new(PROTOCOL_FEE, U512::cl_type()),
            ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn initialize_contract() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_INIT),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn hub_contract_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(get_stream());
    entry_points.add_entry_point(create_stream());
    entry_points.add_entry_point(withdraw_from_stream());
    entry_points.add_entry_point(cancel_stream());
    entry_points.add_entry_point(initialize_contract());
    entry_points
}