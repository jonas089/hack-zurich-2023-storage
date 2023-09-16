#![no_std]
#![no_main]

extern crate alloc;
use casper_types::{
    contracts::NamedKeys, ApiError, CLType, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, URef
};

use casper_contract::{
    contract_api::{
        runtime,
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use alloc::{
    string::{String, ToString},
    vec,
};

#[no_mangle]
pub extern "C" fn submit(){
    let key: String = runtime::get_named_arg("key");
    let value: String = runtime::get_named_arg("value");
    let watches_uref: URef = match runtime::get_key("watches"){
        Some(key) => key,
        None => runtime::revert(ApiError::MissingKey)
    }.into_uref().unwrap_or_revert();
    storage::dictionary_put(watches_uref, &key, value);
}

#[no_mangle]
pub extern "C" fn call(){
    let mut entry_points: EntryPoints = EntryPoints::new();
    let submit: EntryPoint = EntryPoint::new(
        "submit",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );
    entry_points.add_entry_point(submit);

    let mut named_keys = NamedKeys::new();
    let watches_dict = storage::new_dictionary("watches").unwrap_or_revert();
    named_keys.insert("watches".to_string(), watches_dict.into());
    storage::new_contract(
        entry_points, 
        Some(named_keys), 
        Some("hack_zurich_package".to_string()), 
        Some("hack_zurich_hash".to_string())
    );
}