use crate::lib::Article;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ArticleCardProps {
    pub article: Article
}

#[function_component(ArticleCard)]
pub fn article_card(ArticleCardProps { article }: &ArticleCardProps) -> Html {
    html! {
        <div>
            <h3>{&article.title}</h3>
            <p>{&article.description}</p>
        </div>
    }
}
