use crate::app_styling::theme::*;
use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::*;
use seed_style::{px, rem};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Area {
    Home,
    Theme,
    HeaderOptions,
    Empty,
}

impl LayoutArea for Area {
    fn is_empty(&self) -> bool {
        *self == Area::Empty
    }
}
pub fn view(model: &Model) -> Node<Msg> {
    use Area::*;
    use Breakpoint::*;
    Composition::with_layouts(&[
        (
            Small,
            Layout::areas(&[&[Home, Empty, Theme]])
                .style(
                    s().h(px(64))
                        .grid_template_columns("auto 1fr auto")
                        .align_items_center()
                        .padding_x(3),
                )
                .area_style(Home, s().justify_self_left())
                .area_style(Theme, s().justify_self_right()),
        ),
        (
            ExtraSmall,
            Layout::areas(&[&[Home, Empty, HeaderOptions, Theme]])
                .style(
                    s().h(px(64))
                        .grid_template_columns("auto 1fr auto auto")
                        .align_items_center()
                        .padding_x(3),
                )
                .area_style(Home, s().justify_self_left())
                .area_style(HeaderOptions, s().justify_self_right()),
        ),
    ])
    .set_content(Home, |_model: &Model| {
        a![
            attrs! {At::Href => "/home"},
            div![
                s().font_weight_v700().letter_spacing(rem(0.02)),
                s().cursor_pointer().hover(),
                "Seed Style",
            ]
        ]
    })
    .set_content(HeaderOptions, |m: &Model| {
        div![button![
            s().px(4)
                .py(2)
                .radius(px(2))
                .color(Color::Primary)
                .bg_color(Color::MutedPrimary)
                .mr(px(12))
                .b_width(0)
                .b_color(Color::Primary)
                .b_style_solid(),
            "☰",
            m.show_drawer.on_click(|v| *v = !*v)
        ]]
    })
    .set_content(Theme, |_m| {
        let using_dark_theme = use_state(|| false);
        div![button![
            s().px(4)
                .py(2)
                .radius(px(2))
                .bg_color(Color::MutedPrimary)
                .b_width(0)
                .b_style_solid()
                .b_color(Color::Primary),
            if using_dark_theme.get() {
                "Dark Theme"
            } else {
                "Light Theme"
            },
            using_dark_theme.on_click(|t| {
                if *t {
                    change_theme_with_name("dark_theme", my_theme())
                } else {
                    change_theme_with_name("light_theme", dark_theme());
                }
                *t = !*t;
            })
        ]]
    })
    .render(model)
}
