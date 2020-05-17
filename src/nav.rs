use crate::app_styling::theme::*;
use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::*;
use seed_style::{px, vh};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        s().position_fixed()
            .left(px(0))
            .top(px(64))
            .bottom(px(0))
            .z_index("1")
            .min_width(px(0))
            .width(px(256))
            .max_height(vh(100))
            .overflow_x_visible()
            .overflow_y_auto()
            .background_color(Color::Background)
            .padding_bottom(px(32))
            .padding_right(px(8))
            .transition("transform 0.2s ease-out 0s"),
        if model.show_drawer.get() {
            s().transform("translateX(0px)")
                .box_shadow(Shadow::RightEdge)
        } else {
            s().transform("translateX(-100%)").box_shadow("none")
        },
        if model.page.get() == crate::Page::Home {
            s().only_and_above(Breakpoint::Small)
                .transform("translateX(-100%)")
                .box_shadow("none")
        } else {
            s().only_and_above(Breakpoint::Small)
                .box_shadow("none")
                .top(px(0))
                .position_sticky()
                .bottom_auto()
                .transform("none")
        },
        ul![
            s().pl(px(12)).pr(px(38)).pt(px(24)),
            s().style_descendant("a").text_decoration_none(),
            s().style_child("li")
                .font_size(px(16))
                .font_weight_v700()
                .py(px(4))
                .my(px(2))
                .text_decoration_none(),
            li![a![
                attrs! {At::Href => "/home"},
                "Home",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/getting_started"},
                "Getting Started",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/buttons"},
                "Button Styling",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/theming"},
                "Theming Support",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/responsive_styling"},
                "Responsive Styling",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/layout"},
                "Layout Example",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/load_test"},
                "Load Test",
                model.show_drawer.on_click(|v| *v = false)
            ]],
        ]
    ]
}
