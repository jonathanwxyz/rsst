use yew::prelude::*;

#[function_component(AddFeed)]
pub fn add_feed() -> Html {
    html! {
        <>
            <div class="new-feed">
                <label for="add-feed" id="add-feed-label">{"https://"}</label>
                <input type="text" id="add-feed-text" placeholder="jonathanw.xyz/rss.xml" />
                <div class="button">{"Add Feed"}</div>
            </div>
        </>
    }
}
