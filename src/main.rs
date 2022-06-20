mod lib;
use lib::*;

mod components;
use crate::components::feed_card::FeedCard;

use yew::prelude::*;
use reqwasm::http::Request;
use minidom::Element;

#[function_component(App)]
fn app() -> Html {
    let feeds = use_state(|| vec![]);
    {
        let feeds = feeds.clone();
        use_effect_with_deps(move |_| {
            let urls = vec!["https://jonathanw.xyz/rss.xml", "https://frame.work/blog.rss"];
            let feeds = feeds.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut collect_fields = vec![];
                for url in urls {
                    let mut fetched_feed = Request::get(url)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    add_namespace(&mut fetched_feed);

                    let root: Element = fetched_feed.parse().unwrap();

                    const NS: &str = "";
                    let mut articles: Vec<Article> = vec![];
                    let mut title: String = "Feed".to_string();
                    for channel in root.children().filter(|e| e.is("channel", NS)) {
                        match channel.get_child("title", NS) {
                            Some(i) => title = i.text(),
                            _ => ()
                        };
                        for item in channel.children().filter(|e| e.is("item", NS)) {
                            articles.push(Article {
                                title: parse_fields(item, "title"),
                                description: parse_fields(item, "description"),
                                link: parse_fields(item, "link"),
                            })
                        }
                    }
                    collect_fields.push(Feed { title, articles });
                }
                feeds.set(collect_fields);
            });
            || ()
        }, ());
    }

    html! {
        <>
            <h1>{"RSST!!!"}</h1>
            if feeds.len() > 0 {
                {feeds.iter().map(|f| html! {
                    <FeedCard feed={f.clone()}/>
                }).collect::<Html>()}
            }
        </>

    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
