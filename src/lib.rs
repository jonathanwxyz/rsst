use minidom::Element;

#[derive(Debug, Clone)]
pub struct Feed {
    pub title: String,
    pub articles: Vec<Article>
}

#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub link: String
}

pub fn parse_fields(item: &Element, tag: &str) -> String {
    match item.get_child(tag, "") {
        Some(i) => i.text(),
        None => "".to_string()
    }
}

pub fn add_namespace(mut xml: String) -> String {
    let rss_idx = match xml.match_indices("rss").next() {
        Some(ridx) => ridx.0,
        None => panic!("Failed: unable to find compatible rss xml")
    };

    xml.insert_str(rss_idx + 3, r#" xmlns="""#);
    xml
}
