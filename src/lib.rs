use seed::{*, prelude::*};

struct Model {}


#[derive(Clone)]
enum Msg {
}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
}


fn view(_model: &Model) -> impl IntoNodes<Msg> {
    div![]
}


fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}


#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

