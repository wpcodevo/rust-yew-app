use yew::prelude::*;
use yewdux::prelude::*;

use super::feedback_item::FeedbackItem;
use crate::store::Store;

#[function_component]
pub fn FeedbackList() -> Html {
    let (store, _) = use_store::<Store>();
    let feedback_list = store.feedbacks.clone();

    html! {
        <div>
            {
                feedback_list.into_iter().map(|feedback|{
                    let key = feedback.id.to_string();
                    html!{<FeedbackItem {key} feedback={feedback.clone()} />}
                }).collect::<Html>()
            }
        </div>
    }
}
