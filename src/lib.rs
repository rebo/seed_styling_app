#![feature(track_caller)]
use seed::prelude::*;
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::measures::{pc, px};
use seed_style::*;
mod button_styling;
mod layout_composition;
mod theme;
use theme::*;
// This app shows how to use most features of a proposed styling system for Seed.
//
// It includes :
//
// Component Style - Define CSS styles for html elements.
// Theming Support -Define themes for your app so common styles can set centrally
// Layout - Declare Application layout ahead of time in a context agnostic way.

// Not all of these need to be used, all features are optional.
//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------
pub struct Model {}

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
#[derive(Clone)]
pub enum Msg {
    WindowResized,
}

// Update optionally hamdles WindowResized. For performance reasons we dont want to
// re-render the app on every window resize, only if the resize takes the window into new breakpoint
// this step could be completely left off and just added in at the end of a design once all breakpoints have been
// firmly decided.
fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WindowResized => {
            // We just need to provide a copy of the theme that is providing the breakpoints.
            // it is passed in a block because we only need it on first assignment.
            conditionally_skip_rendering::<Breakpoint, _, _, _>(|| my_theme(), orders)
        }
    }
}

// We subscribe to a window resize event in the init in order to handle window resizing
fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.stream(streams::window_event(Ev::Resize, |_| Msg::WindowResized));
    Model {}
}

// Default app start...
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  (At present `Theme` is not Clone therefore need to pass in as owned Vec)  to be improved in future.
//
//  ---------------
#[derive(Clone)]
enum Example {
    LayoutComposition,
    ButtonStyling,
    Theming,
    StyleAliases,
    PerBreakpointStyling,
}

#[topo::nested]
fn view(model: &Model) -> impl IntoNodes<Msg> {
    let example = use_state(|| Example::LayoutComposition);

    use_themes(
        || vec![theme::my_theme(), style_presets()],
        || {
            div![
                S.display_flex().flex_direction_column(),
                div![
                    S.if_nested_style("button")
                        .bg_color(seed_colors::Gray::No5)
                        .color(seed_colors::Base::White)
                        .my(px(5))
                        .mx(px(3))
                        .radius(px(3))
                        .py(px(3))
                        .px(px(6))
                        .outline_style_none(),
                    S.if_nested_style("button")
                        .hover()
                        .cursor_pointer()
                        .bg_color(seed_colors::Gray::No3)
                        .color("#000"),
                    S.display_flex()
                        .flex_direction_row()
                        .bg_color(seed_colors::Gray::No7)
                        .color(seed_colors::Base::White)
                        .p(px(5)),
                    h1![S.display_inline().font_size(px(24)), "Examples"],
                    button![
                        "Layout Composition",
                        example.on_click(|e| *e = Example::LayoutComposition)
                    ],
                    button![
                        "Button Styling",
                        example.on_click(|e| *e = Example::ButtonStyling)
                    ],
                    button!["Theming", example.on_click(|e| *e = Example::Theming)],
                    button![
                        "Style Aliases",
                        example.on_click(|e| *e = Example::StyleAliases)
                    ],
                    button![
                        "PerBreakpointStyling",
                        example.on_click(|e| *e = Example::PerBreakpointStyling)
                    ],
                ],
                match example.get() {
                    Example::LayoutComposition => layout_composition::view(model),
                    Example::ButtonStyling => button_styling::view(model),
                    Example::Theming => layout_composition::view(model),
                    Example::StyleAliases => layout_composition::view(model),
                    Example::PerBreakpointStyling => layout_composition::view(model),
                }
            ]
        },
    )
}
