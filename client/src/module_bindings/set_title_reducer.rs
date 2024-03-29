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
pub struct SetTitleArgs {
    pub title: String,
}

impl Reducer for SetTitleArgs {
    const REDUCER_NAME: &'static str = "set_title";
}

#[allow(unused)]
pub fn set_title(title: String) {
    SetTitleArgs { title }.invoke();
}

#[allow(unused)]
pub fn on_set_title(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<SetTitleArgs> {
    SetTitleArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let SetTitleArgs { title } = __args;
        __callback(__identity, __addr, __status, title);
    })
}

#[allow(unused)]
pub fn once_on_set_title(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<SetTitleArgs> {
    SetTitleArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let SetTitleArgs { title } = __args;
        __callback(__identity, __addr, __status, title);
    })
}

#[allow(unused)]
pub fn remove_on_set_title(id: ReducerCallbackId<SetTitleArgs>) {
    SetTitleArgs::remove_on_reducer(id);
}
