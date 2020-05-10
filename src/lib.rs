#![feature(track_caller)]
use seed::prelude::*;
use seed::*;
use seed_hooks::*;
use seed_style::measures::{pc, px, rem};
use seed_style::*;
mod button_styling;
mod introduction;
mod layout_composition;
mod theme;
use theme::*;
mod thousandtest;
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

#[derive(Clone)]
enum Page {
    Introduction,
    LayoutComposition,
    ButtonStyling,
    LoadTest,
}

pub struct Model {
    // storing page inside a StateAccess allows us to modify it from within a view!
    page: StateAccess<Page>,

    show_drawer: StateAccess<bool>,

    // Yeah a bit hacky.. tbc
    themes: StateAccess<Option<Vec<Theme>>>,
}

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
    // Global style resets above normalize.css
    // Mostly box-sizing and global font.
    GlobalStyle::default()
        .style("button", s().align_self_center().outline_style_none())
        .style("html", s().box_sizing_border_box())
        .style("img", s().box_sizing_content_box())
        .style("*, *:before, *:after", s().box_sizing("inherit"))
        .activate_init_styles();

    orders.stream(streams::window_event(Ev::Resize, |_| Msg::WindowResized));
    Model {
        page: use_state(|| Page::Introduction),
        show_drawer: use_state(|| false),
        themes: use_state(|| Some(vec![my_theme(), style_presets()])),
    }
}

// Default app start...
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum AppAreas {
    Header,
    MainContent,
}
impl LayoutArea for AppAreas {}

#[topo::nested]
pub fn view(model: &Model) -> Node<Msg> {
    // ensure our app is themed.
    use_themes_mut(model.themes, || themed_view(model))
}

pub fn themed_view(model: &Model) -> Node<Msg> {
    use AppAreas::*;
    use Breakpoint::*;

    Composition::with_layout(
        ExtraSmall,
        Layout::areas(&[&[Header], &[MainContent]]).style(s().grid_template_rows("auto 1fr")),
    )
    .set_content(Header, |model| header(model))
    .set_content(MainContent, |model| main_content(model))
    .add_styles(&[
        s().style_child("div")
            .display_flex()
            .flex_direction_column(),
        s().style_descendant("p")
            .line_height(CssLineHeight::Number(1.6))
            .letter_spacing(rem(-0.00278))
            .mt(3),
        s().style_descendant("p code")
            .color(Color::MainText)
            .font_size(0)
            .m(0)
            .radius(px(3))
            .bg_color(Color::MutedSecondary),
        s().style_descendant("h3")
            .font_weight_v900()
            .b_style_solid()
            .bb_width(1)
            .b_color(Color::Primary)
            .display_block()
            .py(3)
            .mt(3)
            .mb(4)
            .font_size(px(24)),
        s().style_descendant("h4")
            .font_style_italic()
            .font_weight_v700()
            .display_block()
            .mt(1)
            .mb(2)
            .font_size(px(18)),
        s().style_descendant("pre")
            .color(Color::MainText)
            .display_flex()
            .flex_direction_row()
            .justify_content_center()
            .width(pc(100))
            .overflow_x_auto()
            .box_sizing_border_box()
            .px(4)
            .py(3)
            .my(2),
        s().style_descendant("pre > code")
            .display_inline_block()
            .bg_color(Color::MutedSecondary)
            .b_style_dotted()
            .overflow_x_auto()
            .display_inline_block()
            .b_width(1)
            .b_color(Color::Secondary)
            .box_sizing_border_box()
            .px(4)
            .py(3)
            .my(2)
            .radius(px(5))
            .font_size(&[px(14), px(18)]),
    ])
    .render(model)
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  (At present `Theme` is not Clone therefore need to pass in as owned Vec)  to be improved in future.
//
//  ---------------
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

fn header(model: &Model) -> Node<Msg> {
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
            Layout::areas(&[&[Home, Empty, Theme]])
                .style(
                    s().h(px(64))
                        .grid_template_columns("auto 1fr auto")
                        .align_items_center()
                        .padding_x(3),
                )
                .area_style(Home, s().justify_self_left())
                .area_style(HeaderOptions, s().justify_self_right()),
        ),
    ])
    .set_content(Home, |model: &Model| {
        h2![
            s().font_weight_v700().letter_spacing(rem(0.02)),
            s().cursor_pointer().hover(),
            "Seed Style",
            model.page.on_click(|p| *p = Page::Introduction)
        ]
    })
    .set_content(HeaderOptions, |m: &Model| {
        div![
            button![
                s().px(4)
                    .py(2)
                    .radius(px(2))
                    .bg_color(Color::MutedPrimary)
                    .b_width(0)
                    .b_style_solid()
                    .b_color(Color::Primary),
                "Theme"
            ],
            button![
                s().px(4)
                    .py(2)
                    .radius(px(2))
                    .bg_color(Color::MutedPrimary)
                    .b_width(0)
                    .b_style_solid()
                    .b_color(Color::Primary),
                "Hamburger",
                m.show_drawer.on_click(|v| *v = !*v)
            ]
        ]
    })
    .set_content(Theme, |m| {
        div![button![
            s().px(4)
                .py(2)
                .radius(px(2))
                .bg_color(Color::MutedPrimary)
                .b_width(0)
                .b_style_solid()
                .b_color(Color::Primary),
            "Theme",
            // hacky ... to be improved
            m.themes.on_click(|vt| {
                let use_dark_theme = use_state(|| true);
                if use_dark_theme.get() {
                    *vt = Some(vec![dark_theme(), style_presets()])
                } else {
                    *vt = Some(vec![my_theme(), style_presets()])
                }
                use_dark_theme.set(!use_dark_theme.get());
            })
        ]]
    })
    .render(model)
}

fn main_content(model: &Model) -> Node<Msg> {
    match model.page.get() {
        Page::LayoutComposition => layout_composition::view(model),
        Page::ButtonStyling => button_styling::view(model),
        Page::LoadTest => thousandtest::view(model),
        Page::Introduction => introduction::view(model),
    }
}
