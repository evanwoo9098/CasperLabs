use alloc::{collections::BTreeMap, string::String, vec::Vec};
use core::{
    convert::{From, TryFrom, TryInto},
    mem::MaybeUninit,
    u8,
};

use super::{
    alloc_bytes, runtime::read_host_buffer_count, str_ref_to_ptr, to_ptr, ContractRef, TURef,
};
use crate::{
    bytesrepr::{self, deserialize, ToBytes},
    contract_api::{error, runtime, Error},
    ext_ffi,
    key::{Key, UREF_SIZE},
    unwrap_or_revert::UnwrapOrRevert,
    uref::AccessRights,
    value::{Contract, Value},
};

pub(crate) fn read_untyped(key: &Key) -> Result<Option<Value>, bytesrepr::Error> {
    // Note: _bytes is necessary to keep the Vec<u8> in scope. If _bytes is
    //      dropped then key_ptr becomes invalid.

    let (key_ptr, key_size, _bytes) = to_ptr(key);
    let output_size = {
        let mut output_size = MaybeUninit::uninit();
        let ret = unsafe { ext_ffi::read_value(key_ptr, key_size, output_size.as_mut_ptr()) };
        match error::result_from(ret) {
            Ok(_) => unsafe { output_size.assume_init() },
            Err(Error::ValueNotFound) => return Ok(None),
            Err(e) => runtime::revert(e),
        }
    };
    let value_bytes = read_host_buffer_count(output_size).unwrap_or_revert();
    let value: Value = deserialize(&value_bytes)?;
    Ok(Some(value))
}

fn try_into<T>(maybe_value: Option<Value>) -> Result<Option<T>, bytesrepr::Error>
where
    T: TryFrom<Value>,
{
    match maybe_value {
        None => Ok(None),
        Some(value) => {
            let ret = value.try_into();
            let ret = ret.map_err(|_| Error::ValueConversion).unwrap_or_revert();
            Ok(Some(ret))
        }
    }
}

/// Read value under the key in the global state
pub fn read<T>(turef: TURef<T>) -> Result<Option<T>, bytesrepr::Error>
where
    T: Into<Value> + TryFrom<Value>,
{
    let key: Key = turef.into();
    let maybe_value = read_untyped(&key)?;
    try_into(maybe_value)
}

/// Reads the value at the given key in the context-local partition of global
/// state
pub fn read_local<K, V>(key: K) -> Result<Option<V>, bytesrepr::Error>
where
    K: ToBytes,
    V: TryFrom<Value>,
{
    let key_bytes = key.to_bytes()?;
    let maybe_value = read_untyped_local(&key_bytes)?;
    try_into(maybe_value)
}

fn read_untyped_local(key_bytes: &[u8]) -> Result<Option<Value>, bytesrepr::Error> {
    let key_bytes_ptr = key_bytes.as_ptr();
    let key_bytes_size = key_bytes.len();
    let output_size = {
        let mut output_size = MaybeUninit::uninit();
        let ret = unsafe {
            ext_ffi::read_value_local(key_bytes_ptr, key_bytes_size, output_size.as_mut_ptr())
        };
        match error::result_from(ret) {
            Ok(_) => unsafe { output_size.assume_init() },
            Err(Error::ValueNotFound) => return Ok(None),
            Err(e) => runtime::revert(e),
        }
    };
    let value_bytes = read_host_buffer_count(output_size).unwrap_or_revert();
    let value: Value = deserialize(&value_bytes)?;
    Ok(Some(value))
}

/// Write the value under the key in the global state
pub fn write<T: Into<Value>>(turef: TURef<T>, t: T) {
    let key = turef.into();
    let value = t.into();
    write_untyped(&key, &value)
}

fn write_untyped(key: &Key, value: &Value) {
    let (key_ptr, key_size, _bytes) = to_ptr(key);
    let (value_ptr, value_size, _bytes2) = to_ptr(value);
    unsafe {
        ext_ffi::write(key_ptr, key_size, value_ptr, value_size);
    }
}

/// Writes the given value at the given key in the context-local partition of
/// global state
pub fn write_local<K, V>(key: K, value: V)
where
    K: ToBytes,
    V: Into<Value>,
{
    let key_bytes = key.to_bytes().unwrap_or_revert();
    write_untyped_local(&key_bytes, &value.into());
}

fn write_untyped_local(key_bytes: &[u8], value: &Value) {
    let key_bytes_ptr = key_bytes.as_ptr();
    let key_bytes_size = key_bytes.len();
    let (value_ptr, value_size, _bytes2) = to_ptr(value);
    unsafe {
        ext_ffi::write_local(key_bytes_ptr, key_bytes_size, value_ptr, value_size);
    }
}

/// Add the given value to the one currently under the key in the global state
pub fn add<T>(turef: TURef<T>, t: T)
where
    Value: From<T>,
{
    let key = turef.into();
    let value = t.into();
    add_untyped(&key, &value)
}

fn add_untyped(key: &Key, value: &Value) {
    let (key_ptr, key_size, _bytes) = to_ptr(key);
    let (value_ptr, value_size, _bytes2) = to_ptr(value);
    unsafe {
        // Could panic if the value under the key cannot be added to
        // the given value in memory
        ext_ffi::add(key_ptr, key_size, value_ptr, value_size);
    }
}

/// Stores the serialized bytes of an exported function under a URef generated by the host.
pub fn store_function(name: &str, named_keys: BTreeMap<String, Key>) -> ContractRef {
    let (fn_ptr, fn_size, _bytes1) = str_ref_to_ptr(name);
    let (keys_ptr, keys_size, _bytes2) = to_ptr(&named_keys);
    let mut addr = [0u8; 32];
    unsafe {
        ext_ffi::store_function(fn_ptr, fn_size, keys_ptr, keys_size, addr.as_mut_ptr());
    }
    ContractRef::TURef(TURef::<Contract>::new(addr, AccessRights::READ_ADD_WRITE))
}

/// Stores the serialized bytes of an exported function at an immutable address generated by the
/// host.
pub fn store_function_at_hash(name: &str, named_keys: BTreeMap<String, Key>) -> ContractRef {
    let (fn_ptr, fn_size, _bytes1) = str_ref_to_ptr(name);
    let (keys_ptr, keys_size, _bytes2) = to_ptr(&named_keys);
    let mut addr = [0u8; 32];
    unsafe {
        ext_ffi::store_function_at_hash(fn_ptr, fn_size, keys_ptr, keys_size, addr.as_mut_ptr());
    }
    ContractRef::Hash(addr)
}

/// Returns a new unforgable pointer, where value is initialized to `init`
pub fn new_turef<T: Into<Value>>(init: T) -> TURef<T> {
    let key_ptr = alloc_bytes(UREF_SIZE);
    let value: Value = init.into();
    let (value_ptr, value_size, _bytes2) = to_ptr(&value);
    let bytes = unsafe {
        ext_ffi::new_uref(key_ptr, value_ptr, value_size); // new_uref creates a URef with ReadWrite access writes
        Vec::from_raw_parts(key_ptr, UREF_SIZE, UREF_SIZE)
    };
    let key: Key = deserialize(&bytes).unwrap_or_revert();
    if let Key::URef(uref) = key {
        TURef::from_uref(uref).unwrap_or_revert()
    } else {
        runtime::revert(Error::UnexpectedKeyVariant);
    }
}
