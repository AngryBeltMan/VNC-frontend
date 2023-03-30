use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages;
#[derive(Debug,Clone,PartialEq,Routable)]
#[non_exhaustive]
pub enum Route{
    #[at("/")]
    Join,
    #[at("/client/:code")]
    Client {code:String},
}

pub fn switch(route:&Route) -> Html {
    match route {
        Route::Join => return html! {<pages::join::Join/>},
        Route::Client {code: _} => return html! {<pages::client::Client/>},
        #[allow(unreachable_patterns)]
        _ => html!(<h1>{"not found"}</h1>)
    }
}
#[derive(Properties,PartialEq,Clone)]
pub struct Path {
    pub path:String
}
