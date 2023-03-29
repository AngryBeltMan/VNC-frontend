use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages;
#[derive(Debug,Clone,PartialEq,Routable)]
pub enum Route{
    #[at("/")]
    Join,
    #[at("/client/:code")]
    Client {code:String},
}

pub fn switch(route:&Route) -> Html {
    match route {
        Route::Join => return html! {<pages::join::Join/>},
        Route::Client { code } => return html! {<pages::client::Client/>},
    }
}
#[derive(Properties,PartialEq,Clone)]
pub struct Path {
    pub path:String
}
