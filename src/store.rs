use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Feedback {
    pub id: uuid::Uuid,
    pub text: String,
    pub rating: u8,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub feedbacks: Vec<Feedback>,
    pub loading: bool,
}

pub fn set_feedback(feedback: Feedback, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.feedbacks.insert(0, feedback);
    })
}

pub fn delete_feedback(id: uuid::Uuid, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.feedbacks.retain(|f| f.id != id);
    })
}

pub fn set_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.loading = loading;
    })
}
