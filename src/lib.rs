#![feature(use_extern_macros)]

extern crate ndarray;
extern crate protobuf;
extern crate serde;
extern crate serde_cbor;
#[macro_use]
extern crate serde_derive;

extern crate rusty_machine;

extern crate ekiden_core_common;
extern crate ekiden_core_trusted;

extern crate poll_api;

mod poll;

use rusty_machine::learning::lin_reg::LinRegressor;
use rusty_machine::prelude::*;

use ekiden_core_common::{Error, Result};
use ekiden_core_common::contract::{Address, Contract};
use ekiden_core_trusted::db::Db;
use ekiden_core_trusted::rpc::create_enclave_rpc;

use utils::{pack_proto, unpack_feature_matrix, unpack_feature_vector};

use contract::DexContract;

// Create enclave RPC handlers.
with_api! {
    create_enclave_rpc!(api);
}

fn vote(request: &VoteRequest) -> Result<VoteResponse> {
    let mut response = VoteResponse::new();
    let contract = PollContract::from_state(&Db::instance().get("state")?);
    let value = contract.get_total();
    response.set_total(value);

    Ok(response)
}

