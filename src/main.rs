mod utils;
use crate::utils::*;

mod organisms;
mod molecules;
mod atoms;
// use crate::organisms::home::Home;
// use crate::organisms::article::Article;
use crate::molecules::feed_card::FeedCard;
use crate::atoms::add_feed::AddFeed;
use crate::organisms::full_article::FullArticle;

use yew::prelude::*;
use reqwasm::http::Request;
use minidom::Element;
//use rayon::prelude::*;
//use tokio::runtime::Runtime;



#[function_component(App)]
fn app() -> Html {

    let feeds = use_state(|| vec![]);
    {
        let feeds = feeds.clone();
        // runs on first load
        use_effect_with_deps(move |_| {
            let urls = vec!["https://jonathanw.xyz/rss.xml",
                            "https://frame.work/blog.rss"];
            let feeds = feeds.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut collect_fields = vec![];
                for url in urls {
                    let mut fetched_feed = {
                        Request::get(url)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap()
                    };


                    let mut articles: Vec<Article> = vec![];
                    let mut title: String = "Feed".to_string();

                    /// Adds a namespace to the xml which is required for
                    /// parsing by minidom.
                    fn add_namespace(xml: &mut String) -> Result<(), String> {
                        let rss_idx = xml.match_indices("rss").next()
                            .ok_or("Unable to find supported RSS XML")?.0;
                        xml.insert_str(rss_idx + 3, r#" xmlns="""#);
                        Ok(())
                    }

                    if let Err(e) = add_namespace(&mut fetched_feed) {
                        articles.push(Article {
                            title: url.to_string(),
                            description: e,
                            is_full_article: true,
                            link: url.to_string()
                        })
                    } else {

                        pub fn parse_fields(item: &Element, tag: &str) -> String {
                            match item.get_child(tag, "") {
                                Some(i) => i.text(),
                                None => "".to_string()
                            }
                        }

                        let root: Element = fetched_feed.parse().unwrap();

                        const NS: &str = "";
                        for channel in root.children().filter(|e| e.is("channel", NS)) {
                            match channel.get_child("title", NS) {
                                Some(i) => title = i.text(),
                                _ => ()
                            };
                            for item in channel.children().filter(|e| e.is("item", NS)) {
                                let is_full_article;
                                if parse_fields(item, "description").starts_with('<') {
                                    is_full_article = true;
                                } else { is_full_article = false; }
                                articles.push(Article {
                                    title: parse_fields(item, "title"),
                                    description: parse_fields(item, "description"),
                                    is_full_article,
                                    link: parse_fields(item, "link"),
                                })
                            }
                        }
                    };
                    collect_fields.push(Feed { title, articles });
                };
                feeds.set(collect_fields);
            });
            || ()
        }, ());
    }

    let selected_article = use_state(|| None);

    let on_article_select = {
        let selected_article = selected_article.clone();
        log::info!("{:#?}", selected_article);
        Callback::from(move |article: Article| {
            selected_article.set(Some(article))
        })
    };

    let on_close = {
        let selected_article = selected_article.clone();
        // _i is just to make the compiler happy :P
        Callback::from(move |_i: i32| {
            selected_article.set(None);
        })
    };

    let full_article = selected_article.as_ref().map(|article| html! {
        <FullArticle article={article.clone()} on_close={on_close.clone()} />
    });
    html! {
        <>
            if *selected_article == None {
                <h1>{"RSST!!!"}</h1>
                <AddFeed />
                if feeds.len() > 0 {
                    {feeds.iter().map(|f| html! {
                        <FeedCard feed={f.clone()} on_article_select={on_article_select.clone()}/>
                    }).collect::<Html>()}
                }
            } else {
                { for full_article }
            }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
