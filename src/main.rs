//use yew::{function_component, html, Html};
use yew::prelude::*;
use serde::{Deserialize, ser::Error};
//use quick_xml::de::from_str;
use reqwasm::http::Request;
//use web_sys::{Request, RequestInit, RequestMode, Response};
use minidom::{Element, error};

#[derive(Debug)]
struct Feed {
    title: String,
    articles: Vec<Article>
}

#[derive(Debug)]
struct Article {
    title: String,
    description: String,
    link: String
}

/*
#[derive(Clone, PartialEq, Deserialize)]
struct Rss {
    channel: Channel
}

#[derive(Clone, PartialEq, Deserialize)]
struct Channel {
    title: String,
    link: String,
    description: String,
    item: Item,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Item {
    title: String,
    link: String,
    guid: String,
    pubDate: String,
    description: String
}
*/
#[function_component(App)]
fn app() -> Html {
    let feeds: UseStateHandle<Vec<Feed>> = use_state(|| Vec::new());
    {
        let feeds = feeds.clone();
        use_effect_with_deps(move |_| {
            let feeds = feeds.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut fetched_feed = Request::get("https://jonathanw.xyz/rss.xml")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                fetched_feed = add_namespace(fetched_feed);

                log::info!("{fetched_feed}");
                let root: Element = fetched_feed.parse().unwrap();
                log::info!("{:#?}", root);

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
                log::info!("{:?}", articles);
                let f = vec![Feed { title, articles }];
                //log::info!("{:?}", *feeds);
            });
            || ()
        }, ());
    }

    html! {
        <h1>{"Hello, World!"}</h1>
    }
}

fn parse_fields(item: &Element, tag: &str) -> String {
    match item.get_child(tag, "") {
        Some(i) => i.text(),
        None => "".to_string()
    }
}

fn add_namespace(mut xml: String) -> String {
    let rss_idx = match xml.match_indices("rss").next() {
        Some(ridx) => ridx.0,
        None => panic!("Failed: unable to find compatible rss xml")
    };

    xml.insert_str(rss_idx + 3, r#" xmlns="""#);
    xml
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
