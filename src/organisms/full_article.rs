use crate::atoms::safe_html::SafeHtml;
use yew::prelude::*;

use crate::utils::Article;

#[derive(PartialEq, Properties)]
pub struct FullArticleProps {
    pub article: Article,
    pub on_close: Callback<i32>
}

#[function_component(FullArticle)]
pub fn full_article(FullArticleProps {article, on_close}: &FullArticleProps) -> Html {

    let close = {
        let on_close = on_close.clone();
        Callback::from(move |_| {
            on_close.emit(42)
        })
    };

    let body = {
        if article.is_full_article {
            article.description.clone()
        } else {
            String::from("yo")
        }
    };

    html! {
        <div class="full-article">
            <div onclick={close} class="close">{"X"}</div>
            <h1>{article.title.clone()}</h1>
            <SafeHtml html={body}/>
        </div>
    }
}
