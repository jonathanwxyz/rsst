use regex::Regex;
use crate::utils::Article;
use crate::atoms::safe_html::SafeHtml;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ArticleCardProps {
    pub article: Article,
    pub on_click: Callback<Article>
}

#[function_component(ArticleCard)]
pub fn article_card(ArticleCardProps { article, on_click }: &ArticleCardProps) -> Html {
    
    fn concise_description(html: &str) -> String {
        const LEN:usize = 300;
        let mat = Regex::new(r"<p>.*</p>").unwrap().find(html);
        if let Some(m) = mat {
            let m = m.as_str();
            if m.len() > LEN {
              m[3..LEN].to_string() + "..."
            } else { m.to_owned() }
        } else {
            String::from("Unable to parse description... Sorry!")
        }
    }

    fn displayed_desc(article: &Article) -> Html {
        let desc = &article.description;
        if article.is_full_article {
            let concise = concise_description(desc);
            html! {
                <SafeHtml html={concise.clone()}/>
            }
        } else {
            html! {
                <p>{desc}</p>
            }
        }
    }

    let on_article_select = {
        let on_click = on_click.clone();
        let article = article.clone();
        Callback::from(move |_| {
            on_click.emit(article.clone())
        })
    };

    html! {
        <div onclick={on_article_select} class="article-card">
            <h3>{&article.title}</h3>
            {displayed_desc(article)}
        </div>
    }
}
