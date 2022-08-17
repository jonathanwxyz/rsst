use yew::prelude::*;

use crate::utils;

#[derive(PartialEq, Properties)]
pub struct FullArticleProps {
    pub article: utils::Article
}

#[function_component(FullArticle)]
pub fn full_article(FullArticleProps {article}: &FullArticleProps) -> Html {
    html! {
        <>
            <p>{"hi"}</p>
        </>
    }
}
