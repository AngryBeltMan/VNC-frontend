use yew::prelude::*;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement,EventTarget};
use crate::routers::Route;
#[function_component(Join)]
pub fn join() -> Html {
    let join_code = use_state(|| String::new());
    let join_code = Arc::new(join_code);
    let j = Arc::clone(&join_code);
    let nav = use_navigator().unwrap();
    let input = move |e:Event| {
        let join_code = &j;
        let target:Option<EventTarget> = e.target();
        let html_input:HtmlInputElement = target
            .expect("error getting input")
            .dyn_into()
            .unwrap();
        join_code.set(html_input.value());
    };
    let j = Arc::clone(&join_code);
    let join = move |_| {
        let j = &**j;
        if (j.len() >= 5) | (j.len() <= 10) {
            let nav = &nav;
            nav.push(&Route::Client { code: j.to_string() });
        }
    };
    html! {
        <div>
            <input type="text" onchange={input}/>
            <button onclick={join}>{"join"}</button>
        </div>
    }
}
