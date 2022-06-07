use crate::lib::Feed;
use crate::components::article_card::ArticleCard;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ArticleCardProps {
    pub feed: Feed
}

#[function_component(FeedCard)]
pub fn feed_card(ArticleCardProps {feed}: &ArticleCardProps) -> Html {
    let articles = feed.articles.iter().map(|a| html! {
        <ArticleCard article={a.clone()}/>
    }).collect::<Html>();

    html! {
        <div>
            <h2>{&feed.title}</h2>
            {articles}
        </div>
    }
}
