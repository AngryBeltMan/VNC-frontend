use web_sys::MouseEvent;
pub trait MButton {
    fn button_type(&self) -> String;
}
impl MButton for MouseEvent {
    fn button_type(&self) -> String {
        match self.button() {
            0 => {
                return format!("LEFT");
            }
            1 => {
                return format!("MIDDLE");
            }
            2 => {
                return format!("RIGHT");
            }
            _ => return format!("UNKOWN"),
        }
    }
}
#[macro_export]
macro_rules! eventlistener {
    ($win:expr,$kind:expr,$ws:expr,$format:expr,$type:ty,$method:ident) => {{
        use gloo_console::log;
        use gloo_events::EventListener;
        use std::pin::pin;
        use std::sync::Arc;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::spawn_local;
        use ws_stream_wasm::*;
        use yew::prelude::*;
        EventListener::new($win, $kind, move |event: &Event| {
            if let Ok(o) = event.clone().dyn_into::<$type>() {
                let key = o.$method();
                let ws = Arc::clone(&$ws);
                spawn_local(async move {
                    let key = format!("{},{key}", $format);
                    log!(&key);
                    pin!(ws.lock().await)
                        .send(WsMessage::Text(key))
                        .await
                        .unwrap();
                });
            } else {
                log!("error");
            }
        })
        .forget();
    }};
}
#[macro_export]
macro_rules! input {
    ($state:expr) => {{
        use yew::prelude::*;
        let change = move |event:Event| {
            let state = &$state;
            let target:Option<web_sys::EventTarget> = event.target();
            let html_input:web_sys::HtmlInputElement = target
                .expect("error getting input")
                .dyn_into()
                .unwrap();
            state.set(html_input.value());
        };
        change
    }};
}
