use futures_util::{StreamExt, SinkExt};
use std::pin::pin;
use serde::{Deserialize,Serialize};
use wasm_bindgen::JsCast;
use gloo_utils::errors::JsError;
use gloo_storage::{LocalStorage,Storage};
use gloo_events::EventListener;
use wasm_bindgen_futures::spawn_local;
use async_mutex::Mutex;
use yew_hooks::use_async;
use yew::prelude::*;
use gloo_net::websocket::{self,Message};
use gloo_console::log;
use base64::{engine::general_purpose, Engine as _};
use std::sync::{Arc};
use gloo_net::websocket::futures::WebSocket;
use ws_stream_wasm:: *;

mod routers;

struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        if LocalStorage::get::<Resolution>("resolution").is_err() { // this likely means resolution has
            LocalStorage::set("resolution",Resolution {
                resolution: (960,540)
            }).unwrap();
        }
        Self { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"VNC"}</h1>
                <Frame />
                <button>{"hello"}</button>
            </div>
        }
    }
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render { return; }
        spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let frame = loop {
                if let Some(o) = document.get_element_by_id("frame") {
                    log!("found image");
                    break o;
                } else {
                    log!("continue");
                }
            };
            let ws = WsMeta::connect( "ws://127.0.0.1:6969/ws/keyboard/hello", None ).await.unwrap().1;
            let ws = Arc::new(Mutex::new(ws));
            let res = LocalStorage::get::<Resolution>("resolution").unwrap().resolution; // okay to unwrap because
            let mouse_ws = Arc::clone(&ws);
            let key_ws_up = Arc::clone(&ws);
            let key_ws_down = Arc::clone(&ws);
            let mouse_ws_press = Arc::clone(&ws);
            let mouse_ws_release = Arc::clone(&ws);
            EventListener::new(&window,"keyup", move |event:&Event| {
                if let Ok(o) = event.clone().dyn_into::<web_sys::KeyboardEvent>() {
                    let key_ws_up = Arc::clone(&key_ws_up);
                    let key = o.key();
                    spawn_local(async move {
                        let key = format!("KEYUP,{}",key);
                        log!(&key);
                        pin!(key_ws_up.lock().await).send(WsMessage::Text(key)).await.unwrap();
                    });
                } else {
                    log!("error");
                }
            }).forget();
            EventListener::new(&frame,"mousemove", move |event:&Event| {
                let mouse_event = event.clone().dyn_into::<MouseEvent>();
                if let Ok(mouse) =   mouse_event {
                    let cords = (mouse.x(),mouse.y());
                    let mouse_ws = Arc::clone(&mouse_ws);
                    spawn_local(async move {
                        let mouse_ws = Arc::clone(&mouse_ws);
                        let mouse = format!("MOUSEMOVE,{},{},{},{}",cords.0,cords.1,res.0,res.1);
                        log!(&mouse);
                        pin!(mouse_ws.lock().await).send(WsMessage::Text(mouse)).await.unwrap();
                    });
                }
            }).forget();
            EventListener::new(&window,"keydown", move |event:&Event| {
                if let Ok(o) = event.clone().dyn_into::<web_sys::KeyboardEvent>() {
                    let key_ws_down = Arc::clone(&key_ws_down);
                    let key = o.key();
                    let key = format!("KEYDOWN,{}",key);
                    log!(&key);
                    spawn_local(async move {
                        pin!(key_ws_down.lock().await).send(WsMessage::Text(key)).await.unwrap();
                    });
                }
            }).forget();
            EventListener::new(&window,"mousedown", move |event:&Event| {
                if let Ok(o) = event.clone().dyn_into::<MouseEvent>() {
                    match o.button() {
                        0 => {
                            let mouse_ws_press = Arc::clone(&mouse_ws_press);
                            let mouse = format!("MOUSEDOWN,LEFT");
                            spawn_local(async move {
                                pin!(mouse_ws_press.lock().await).send(WsMessage::Text(mouse)).await.unwrap();
                            });
                        },
                        1 => {
                            let mouse_ws_press = Arc::clone(&mouse_ws_press);
                            let mouse = format!("MOUSEDOWN,MIDDLE");
                            spawn_local(async move {
                                pin!(mouse_ws_press.lock().await).send(WsMessage::Text(mouse)).await.unwrap();
                            });
                        },
                        2 => {
                            let mouse_ws_press = Arc::clone(&mouse_ws_press);
                            let mouse = format!("MOUSEDOWN,RIGHT");
                            spawn_local(async move {
                                pin!(mouse_ws_press.lock().await).send(WsMessage::Text(mouse)).await.unwrap();
                            });
                        },
                        _ => {}
                    }
                }
            }).forget();
            EventListener::new(&window,"mouseup", move |event:&Event| {
                if let Ok(o) = event.clone().dyn_into::<MouseEvent>() {
                    match o.button() {
                        0 => {
                            let mouse_ws_press = Arc::clone(&mouse_ws_release);
                            let mouse = format!("MOUSEUP,LEFT");
                            spawn_local(async move {
                                pin!(mouse_ws_press.lock().await).send(WsMessage::Text(mouse)).await.unwrap();
                            });
                        },
                        1 => {
                            let mouse_ws_press = Arc::clone(&mouse_ws_release);
                            let mouse = format!("MOUSEUP,MIDDLE");
                            spawn_local(async move {
                                pin!(mouse_ws_press.lock().await).send(WsMessage::Text(mouse)).await.unwrap();
                            });
                        },
                        2 => {
                            let mouse_ws_press = Arc::clone(&mouse_ws_release);
                            let mouse = format!("MOUSEUP,RIGHT");
                            spawn_local(async move {
                                pin!(mouse_ws_press.lock().await).send(WsMessage::Text(mouse)).await.unwrap();
                            });
                        },
                        _ => {}
                    }
                }
            }).forget();

        });
   }
}
#[derive(Deserialize,Serialize)]
pub struct Resolution {
    resolution:(u32,u32)
}
pub fn connect_with_ws() -> Result<WebSocket,JsError> {
    log!("opening");
    gloo_net::websocket::futures::WebSocket::open("ws://127.0.0.1:6969/ws/frames/hello")

}
#[function_component(Frame)]
pub fn frame() -> Html {
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

fn main() {
    yew::Renderer::<App>::new().render();
}
