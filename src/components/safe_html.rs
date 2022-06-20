use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let p = gloo_utils::document().create_element("p").unwrap();
    p.set_inner_html(&props.html.clone());

    Html::VRef(p.into())
}
