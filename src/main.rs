use serde::{Deserialize,Serialize};
use gloo_storage::{LocalStorage,Storage};
use yew::prelude::*;
use yew_router::prelude::*;

mod routers;
mod pages;
mod macros;

pub const URL:&str = "vnc-shuttle.shuttleapp.rs";

struct App { }

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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"VNC"}</h1>
                <BrowserRouter>
                    <Switch<routers::Route> render={|e| routers::switch(&e)}/>
                </BrowserRouter>
            </div>
        }
    }
}
#[derive(Deserialize,Serialize,Properties,PartialEq,Clone)]
pub struct Resolution {
    resolution:(u32,u32)
}
fn main() {
    yew::Renderer::<App>::new().render();
}
