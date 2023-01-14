use std::{io::Cursor, rc::Rc};

use sitemap_xml::writer::SitemapWriter;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

enum Action {
    Add(url::Url),
}

#[derive(Debug, Default)]
struct State {
    urls: Vec<url::Url>,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Add(url) => {
                let mut urls = self.urls.clone();
                urls.push(url);
                State { urls }.into()
            }
        }
    }
}

fn build_preview(urls: &[url::Url]) -> anyhow::Result<String> {
    let mut writer = SitemapWriter::start_with_indent(Cursor::new(Vec::new()))?;
    for url in urls.iter() {
        writer.write(sitemap_xml::writer::Url::loc(url.clone())?)?;
    }
    writer.end()?;
    Ok(String::from_utf8(writer.into_inner().into_inner())?)
}

#[function_component]
fn App() -> Html {
    let state = use_reducer(State::default);
    let onkeypress = {
        let state = state.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();

                input.set_value("");
                let u = url::Url::parse(&value).unwrap_throw();
                state.dispatch(Action::Add(u))
            }
        })
    };

    let url_list = state
        .urls
        .iter()
        .map(|url| html! { <li>{ url.to_string() }</li> })
        .collect::<Vec<Html>>();
    let preview = build_preview(&state.urls).unwrap_throw();

    html! {
        <div style="">
            <h1>{"Online Sitemap Builder"}</h1>
            <div style="display: flex; width: 100%; justify-content: center; ">
                <div class="editor" style="width: 50%">
                    <input {onkeypress} style="display: block; margin: 0 auto; padding: 8px; width: 80%;" />
                    <ul>{url_list}</ul>
                </div>
                <div class="preview" style="width: 50%">
                    <pre style="background-color: #333333; border: 1px solid #f4f7f2; color: #f4f7f2; margin: 0; padding: 16px; white-space: pre-wrap;">{preview}</pre>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
