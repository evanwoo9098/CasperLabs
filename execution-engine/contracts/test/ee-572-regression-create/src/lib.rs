#![no_std]

use core::convert::Into;

use contract_ffi::{
    contract_api::{runtime, storage, TURef},
    key::Key,
    unwrap_or_revert::UnwrapOrRevert,
    uref::{AccessRights, URef},
    value::CLValue,
};

const DATA: &str = "data";
const CONTRACT_NAME: &str = "create";

#[no_mangle]
pub extern "C" fn create() {
    let reference: TURef<&str> = storage::new_turef(&DATA);

    let read_only_reference: URef = {
        let mut ret: TURef<&str> = reference;
        ret.set_access_rights(AccessRights::READ);
        ret.into()
    };
    let return_value = CLValue::from_t(read_only_reference).unwrap_or_revert();
    runtime::ret(return_value)
}

#[no_mangle]
pub extern "C" fn call() {
    let contract: Key = storage::store_function_at_hash(CONTRACT_NAME, Default::default()).into();
    runtime::put_key(CONTRACT_NAME, contract)
}
