use yew::prelude::*;
use serde::{Deserialize,Serialize};
use stylist::Style;
use futures_util::{StreamExt, SinkExt};
use std::pin::pin;
use wasm_bindgen::JsCast;
use gloo_storage::{LocalStorage,Storage};
use gloo_events::EventListener;
use wasm_bindgen_futures::spawn_local;
use async_mutex::Mutex;
use std::sync::Arc;
use ws_stream_wasm:: *;
use gloo_net;
use gloo_console::log;
use yew_hooks::{use_async, use_location};
use base64::{engine::general_purpose, Engine as _};
use gloo_utils::errors::JsError;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use crate::{URL,Resolution,eventlistener, macros::MButton};

const SCREEN:&'static str = include_str!("screen.css");

pub struct Client {
    resolution:Resolution,
}

impl Component for Client {
    type Message = ();
    type Properties = ();
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Frame/>
            </div>
        }
    }
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            resolution:Resolution { resolution: (100,100) },

        }
    }
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render { return; }
        let window = web_sys::window().unwrap();
        let path = window.location().to_string().as_string().unwrap();
        let path = path.split("/").last().unwrap().to_string();
        let document = window.document().unwrap();

        let frame = loop {
            if let Some(o) = document.get_element_by_id("frame") {
                log!("found image");
                break o;
            } else {
                log!("continue");
            }
        };
        let dom = frame.get_bounding_client_rect();
        let locx = dom.x().floor() as i32;
        let locy = dom.y().floor() as i32;

        spawn_local(async move {
            let ws = WsMeta::connect(&format!("wss://{URL}/ws/keyboard/{path}"), None ).await.unwrap().1;
            let ws = Arc::new(Mutex::new(ws));
            let res = LocalStorage::get::<Resolution>("resolution").unwrap().resolution; // okay to unwrap because
            let mouse_ws = Arc::clone(&ws);
            let key_ws_up = Arc::clone(&ws);
            let key_ws_down = Arc::clone(&ws);
            let mouse_ws_press = Arc::clone(&ws);
            let mouse_ws_release = Arc::clone(&ws);

            // checks if keys have been pressed up
            eventlistener!(&window,"keyup",Arc::clone(&key_ws_up),"KEYUP",web_sys::KeyboardEvent,key);

            // checks if keys have been pressed down
            eventlistener!(&window,"keydown",Arc::clone(&key_ws_down),"KEYDOWN",web_sys::KeyboardEvent,key);

            // checks if mouse button have been pressed down
            eventlistener!(&window,"mousedown",Arc::clone(&mouse_ws_press),"MOUSEDOWN",web_sys::MouseEvent,button_type);

            // checks if mouse button have been pressed up
            eventlistener!(&window,"mouseup",Arc::clone(&mouse_ws_release),"MOUSEUP",web_sys::MouseEvent,button_type);

            EventListener::new(&frame,"mousemove", move |event:&Event| {
                let mouse_event = event.clone().dyn_into::<MouseEvent>();
                if let Ok(mouse) =   mouse_event {
                    let cords = ( mouse.x() - locx ,mouse.y() - locy );
                    let mouse_ws = Arc::clone(&mouse_ws);
                    spawn_local(async move {
                        let mouse_ws = Arc::clone(&mouse_ws);
                        let mouse = format!("MOUSEMOVE,{},{},{},{}",cords.0,cords.1,res.0,res.1);
                        log!(&mouse);
                        pin!(mouse_ws.lock().await).send(WsMessage::Text(mouse)).await.unwrap();
                    });
                }
            }).forget();
});
    }
}
#[function_component(Frame)]
pub fn frame() -> Html {
    let loc = use_location();
    let ws = use_state(|| {
        Arc::new(Mutex::new(connect_with_ws(&loc.pathname.replace("/client/", "")).unwrap()))
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
                let style = Style::new(SCREEN).unwrap();
                html!{
                    <div>
                        <img width=960 height=540 class={style} id="frame"/>
                    </div>
                }
            }
        }
    }
}
pub async fn get_frames(read:Arc<Mutex<WebSocket>>) -> Result<Html,()> {
    log!("connecting ...");
    if let Some(Ok(o)) = read.lock().await.next().await {
        let style = Style::new(SCREEN).unwrap();
        match o {
            Message::Bytes(b) => {
                let base64 = general_purpose::STANDARD.encode(&b);
                return Ok(html!{
                    <div>
                        <img width=960 height=540 id="frame" class={style} src={format!("data:image/jpeg;base64,{base64}")}/>
                    </div>
                });
            },
            Message::Text(_) => { }
        }
    }
    Err(())
}

pub fn connect_with_ws(code:&str) -> Result<WebSocket,JsError> {
    log!("opening");
    gloo_net::websocket::futures::WebSocket::open(&format!("wss://{URL}/ws/frames/{code}"))
}
