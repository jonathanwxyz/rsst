use crate::lib::Article;
use crate::components::safe_html::SafeHtml;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ArticleCardProps {
    pub article: Article
}

#[function_component(ArticleCard)]
pub fn article_card(ArticleCardProps { article }: &ArticleCardProps) -> Html {
    
    let desc = &article.description;
    let displayed_desc: Html;
    if desc.starts_with('<') {
        displayed_desc = html! {
            <SafeHtml html={desc.clone()}/>
        }
    } else {
        displayed_desc = html! {
            <p>{desc}</p>
        }
    }

    html! {
        <div>
            <h3>{&article.title}</h3>
            {displayed_desc}
        </div>
    }
}
