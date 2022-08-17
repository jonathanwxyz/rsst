#[derive(Debug, Clone, PartialEq)]
pub struct Feed {
    pub title: String,
    pub articles: Vec<Article>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub is_full_article: bool,
    pub link: String
}
