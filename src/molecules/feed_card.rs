use crate::utils::{Feed, Article};
use crate::atoms::article_card::ArticleCard;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FeedCardProps {
    pub feed: Feed,
    pub on_article_select: Callback<Article>
}

#[function_component(FeedCard)]
pub fn feed_card(FeedCardProps {feed, on_article_select}: &FeedCardProps) -> Html {
    let articles = feed.articles.iter().map(|a| {
        html! {
            <ArticleCard on_click={on_article_select} article={a.clone()} />
    }}).collect::<Html>();

    html! {
        <div class="feed-card">
            <h2>{&feed.title}</h2>
            <img src="assets/trash.svg" class="trash" />
            {articles}
        </div>
    }
}
