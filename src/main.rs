#![feature(nll)]


extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;


mod network_agent;
mod app;


use app::Model as AppModel;
use yew::prelude::*;


fn main() {
    let app = App::<AppModel>::new();

    yew::initialize();

    app.mount_to_body();

    yew::run_loop();
}
