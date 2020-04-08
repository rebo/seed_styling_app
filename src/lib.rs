#![feature(track_caller)]
use seed::{*, prelude::*};
use seed_hooks::*;


//  Model, Msg, Update, init(), and start()
//  ---------------------------------------
struct Model {}


#[derive(Clone)]
enum Msg {
}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}


#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

//  View code here!
//  ---------------
#[topo::nested]
fn view(_model: &Model) -> impl IntoNodes<Msg> {
    let counter = use_state(||0);
    button![
     "Clicked ", counter, " times",
     counter.mouse_ev(Ev::Click, |v,_| *v += 1)
    ]
}




