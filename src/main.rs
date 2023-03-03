mod components;
mod store;

use components::{
    feedback_form::FeedbackForm, feedback_list::FeedbackList, feedback_stats::FeedbackStats,
};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <main class="md:container mt-24 px-5">
            <FeedbackForm />
            <FeedbackStats />
            <FeedbackList />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
