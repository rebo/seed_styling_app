use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::px;
use seed_style::*;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AppArea {
    Main,
    Nav,
}

impl LayoutArea for AppArea {}

#[topo::nested]
pub fn main_with_sidebar(on_intro_page: bool) -> Composition<AppArea, Model, Msg, SeedBreakpoint> {
    use AppArea::*;
    Composition::with_layout(
        Layout::areas(&[&[Nav, Main]])
            .area_style(Main, if !on_intro_page { s().px(px(12)) } else { s() })
            .style(s().grid_template_columns("auto minmax(0px,1fr)")),
    )
}

pub fn render_centred_article<F: Fn(&Model) -> Node<Msg> + 'static>(
    model: &Model,
    content: F,
) -> Node<Msg> {
    Composition::with_layout(Layout::grid(
        s().grid_template_columns("1fr minmax(0px,1000px) 1fr"),
    ))
    .add_style(s().style_descendant("button").align_self_center())
    .add_child(move |model: &Model| {
        div![
            only_and_above(SeedBreakpoint::Small, || {log!("hiding_drawer");model.show_drawer.set(false); empty![]}),
            s().grid_column_start("2")
                .grid_column_end("3")
                .padding_x(3)
                .padding_y(2),
            content(model)
        ]
    })
    .render(model)
}
