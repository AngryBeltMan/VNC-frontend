use yew::prelude::*;
use futures_util::StreamExt;
use std::sync::Arc;
use async_mutex::Mutex;
use gloo_net;
use gloo_console::log;
use yew_hooks::use_async;
use base64::{engine::general_purpose, Engine as _};
use gloo_utils::errors::JsError;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use crate::URL;
pub fn connect_with_ws() -> Result<WebSocket,JsError> {
    log!("opening");
    gloo_net::websocket::futures::WebSocket::open(&format!("ws://{URL}/ws/frames/hello"))

}
#[function_component(Client)]
pub fn client() -> Html {
    // log!("sending");
    let ws = use_state(|| {
        Arc::new(Mutex::new(connect_with_ws().unwrap()))
    });
    let websock = Arc::clone(&ws);
    let frames = use_async(async move {
        get_frames(Arc::clone(&websock)).await
    });
    if !frames.loading {
        frames.run();
    }
    html!{
        {
            if let Some(o) = &frames.data {
                o.clone()
            } else {
                html!{
                    <div>
                        <img width=960 height=540 id="frame"/>
                    </div>
                }
            }
        }
    }
}
pub async fn get_frames(read:Arc<Mutex<WebSocket>>) -> Result<Html,()> {
    log!("connecting ...");
    if let Some(Ok(o)) = read.lock().await.next().await {
        match o {
            Message::Bytes(b) => {
                let base64 = general_purpose::STANDARD.encode(&b);
                return Ok(html!{
                    <div>
                        <img width=960 height=540 id="frame" src={format!("data:image/jpeg;base64,{base64}")}/>
                    </div>
                });
            },
            Message::Text(_) => {
            }
        }
    }
    Err(())
}

