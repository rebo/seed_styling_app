use crate::app_styling::theme::*;
use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::*;
use seed_style::{px, vh};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        s().only_and_above(SeedBreakpoint::Small).width(px(256)).position_sticky().top(px(0)),
    div![
        only_and_above(SeedBreakpoint::Small, || { model.show_drawer.set(false); empty![]}),
        s()
            .position_fixed()
            .left(px(0))
            .top(px(64))
            .bottom(px(0))
            .z_index("1")
            .min_width(px(0))
            .width(px(256))
            .max_height(vh(100))
            .overflow_x_visible()
            .overflow_y_auto()
            .padding_bottom(px(32))
            .padding_right(px(8))
            .padding_left(px(4))
            .transition("transform 0.2s ease-out 0s")
            .background_color(Color::Background),
        if model.show_drawer.get() {
            s().transform("translateX(0px)")
                .box_shadow(Shadow::RightEdge)
        } else {
            s().transform("translateX(-100%)").box_shadow("none")
        },
            s().only_and_above(Breakpoint::Small)
                .box_shadow("none")
                .top(px(0))
                .position_sticky()
                .bottom_auto()
                .transform("none")
        ,
        div![s().pt(px(30)).font_size(px(20)).font_weight_v900(),"Seed Style"]
        ,
        ul![
            s().pl(px(12)).pr(px(38)).pt(px(12)),
            s().style_descendant("a").text_decoration_none(),
            s().style_child("li")
                .font_size(px(16))
                .font_weight_v700()
                .py(px(8))
                .my(px(4))
                .text_decoration_none(),
            s().style_child("li").list_style_type_none(),
            li![a![
                attrs! {At::Href => "/style_home"},
                "Style Home",
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
                attrs! {At::Href => "/simple_layout"},
                "Simple Layout Primitives",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/layout"},
                "Layout Example",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/extending_seed"},
                "Extending Seed",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/load_test"},
                "Load Test",
                model.show_drawer.on_click(|v| *v = false)
            ]],

        ],
        div![s().pt(px(30)).font_size(px(20)).font_weight_v900(),"Seed Hooks"]
        ,
        ul![
            s().pl(px(12)).pr(px(38)).pt(px(12)),
            s().style_descendant("a").text_decoration_none(),
            s().style_child("li")
                .font_size(px(16))
                .font_weight_v700()
                .py(px(8))
                .my(px(4))
                .text_decoration_none(),
            s().style_child("li").list_style_type_none(),
            li![a![
                attrs! {At::Href => "/hooks_home"},
                "Hooks Home",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/hooks_getting_started"},
                "Hooks Getting Started",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/hooks_api"},
                "Hooks Api",
                model.show_drawer.on_click(|v| *v = false)
            ]],
            li![a![
                attrs! {At::Href => "/hooks_tutorial"},
                "Hooks Tutorial",
                model.show_drawer.on_click(|v| *v = false)
            ]],
        ]
    ]
    ]
}
