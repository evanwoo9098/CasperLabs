#![no_std]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String};

use contract_ffi::{
    contract_api::{runtime, storage, ContractRef},
    unwrap_or_revert::UnwrapOrRevert,
    uref::URef,
    value::CLValue,
};

#[no_mangle]
pub extern "C" fn hello_ext() {
    let test_string = String::from("Hello, world!");
    let test_uref: URef = storage::new_turef(test_string).into();
    let return_value = CLValue::from_t(test_uref).unwrap_or_revert();
    runtime::ret(return_value)
}

#[no_mangle]
pub extern "C" fn call() {
    let named_keys = BTreeMap::new();
    let contract_pointer: ContractRef = storage::store_function_at_hash("hello_ext", named_keys);
    runtime::put_key("hello_ext", contract_pointer.into());
}
