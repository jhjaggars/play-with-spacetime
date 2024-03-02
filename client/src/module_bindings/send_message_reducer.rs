// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SendMessageArgs {
    pub text: String,
}

impl Reducer for SendMessageArgs {
    const REDUCER_NAME: &'static str = "send_message";
}

#[allow(unused)]
pub fn send_message(text: String) {
    SendMessageArgs { text }.invoke();
}

#[allow(unused)]
pub fn on_send_message(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<SendMessageArgs> {
    SendMessageArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let SendMessageArgs { text } = __args;
        __callback(__identity, __addr, __status, text);
    })
}

#[allow(unused)]
pub fn once_on_send_message(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<SendMessageArgs> {
    SendMessageArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let SendMessageArgs { text } = __args;
        __callback(__identity, __addr, __status, text);
    })
}

#[allow(unused)]
pub fn remove_on_send_message(id: ReducerCallbackId<SendMessageArgs>) {
    SendMessageArgs::remove_on_reducer(id);
}