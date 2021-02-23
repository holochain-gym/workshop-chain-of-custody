use hdk3::prelude::*;

pub fn err(reason: &str) -> WasmError {
    WasmError::Zome(String::from(reason))
}

entry_defs![];
