#![feature(track_caller)]
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::*;
use seed_style::vw;

mod button_styling;
mod compositions;
mod extending_seed;
mod getting_started;
mod header;
mod home;
mod home_style;
mod simple_layout;
mod layout_composition;
mod nav;
mod responsive_styling;
mod theming;
mod thousandtest;
// mod hooks_api_ref;
mod hooks_home;
mod hooks_getting_started;
mod hooks_api;
mod hooks_tutorial;


mod app_styling;
use app_styling::global_styles::{init_styles,themed_global_styles};
use app_styling::theme::*;

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

#[derive(Clone, PartialEq)]
enum Page {
    Home,
    StyleHome,
    SimpleLayout,
    LayoutComposition,
    Theming,
    ResponsiveStyling,
    ButtonStyling,
    LoadTest,
    GettingStarted,
    ExtendingSeed,
    HooksHome,
    HooksGettingStarted,
    HooksApi,
    HooksTutorial,
}

pub struct Model {
    // storing page inside a StateAccess allows us to modify it from within a view eventHandler callback!
    page: StateAccess<Page>,
    show_drawer: StateAccess<bool>,
}

// In aps that make use of conditional rendering on breakpoints we We just need one Msg
// in order to handle a WindowResized event.
// Currently need a NoOp for stream/subscribe re-rendering, this is do to be fixed.
#[derive(Clone)]
pub enum Msg {
    SubmitMarkdownHtml(String),
    WindowResized,
    NoOp,
}

// Update optionally handles WindowResized. For performance reasons we don't want to
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
        Msg::NoOp => {},
        Msg::SubmitMarkdownHtml(html) => log!(html),
    }
}

// init sets up simple routing, global CSS styles for css resets,
// and window resizing callback
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    
    load_app_themes(&[default_colors_theme, default_breakpoint_theme, my_theme]);
    // setup a page state accessor, which is modified as part of a Url Changed subscription
    let page = use_state(|| Page::Home);

    orders
        .subscribe(move |subs::UrlChanged(mut url)| {
            let new_page = match url.remaining_path_parts().as_slice() {
                ["style_home"] => Page::StyleHome,
                ["home"] => Page::Home,
                ["buttons"] => Page::ButtonStyling,
                ["theming"] => Page::Theming,
                ["responsive_styling"] => Page::ResponsiveStyling,
                ["load_test"] => Page::LoadTest,
                ["simple_layout"] => Page::SimpleLayout,
                ["layout"] => Page::LayoutComposition,
                ["getting_started"] => Page::GettingStarted,
                ["extending_seed"] => Page::ExtendingSeed,
                ["hooks_home"] => Page::HooksHome,
                ["hooks_api"] => Page::HooksApi,
                ["hooks_tutorial"] => Page::HooksTutorial,
                ["hooks_getting_started"] => Page::HooksGettingStarted,
                _ =>  Page::Home,
            };

            if page.get() != new_page {
                window().scroll_to_with_x_and_y(0., 0.);
                page.set(new_page);
            }
            Msg::NoOp
            }
            // 
        )
        .notify(subs::UrlChanged(url));

    // Global style resets above normalize.css
    // Mostly box-sizing and global font.
    init_styles();

    // We subscribe to a window resize event in the init in order to handle window resizing
    orders.stream(streams::window_event(Ev::Resize, |_| Msg::WindowResized));
    themed_global_styles().get();
    // Our model just needs the state accessors for page, show_drawer, and themes
    Model {
        page,
        show_drawer: use_state(|| false),
    }
}

// Default app start...
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

// We setup some areas to use with Seed Styles layout system
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum AppAreas {
    Header,
    MainContent,
}
impl LayoutArea for AppAreas {}

#[topo::nested]
pub fn view(model: &Model) -> Node<Msg> {
    themed_view(model)
}

pub fn themed_view(model: &Model) -> Node<Msg> {
    use AppAreas::*;
    use Breakpoint::*;

    Composition::with_layout(
        Layout::areas(&[&[Header], &[MainContent]])
            .area_style(
                Header,
                s().only_and_below(ExtraSmall)
                    .z_index("2")
                    .position_fixed()
                    .bg_color(Color::Background)
                    .width(vw(100))
                    .top("0px")
                    .right("0px")
                    .left("0px"),
            )
            .style(s().grid_template_rows("auto 1fr")),
    )
    .set_content(Header, |model| header::view(model))
    .set_content(MainContent, |model| main_layout(model))
    .render(model)
}

//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  (At present `Theme` is not Clone therefore need to pass in as owned Vec)  to be improved in future.
//
//  ---------------
#[topo::nested]
fn main_layout(model: &Model) -> Node<Msg> {
    use compositions::AppArea::*;

    compositions::main_with_sidebar(
        model.page.get() == Page::Home || model.page.get() == Page::LayoutComposition || model.page.get() == Page::HooksHome,
    )
    .set_content(Main, main_view)
    .set_content(Nav, nav::view)
    .render(model)
    // }
}

#[topo::nested]
fn main_view(model: &Model) -> Node<Msg> {
    match model.page.get() {
        Page::Home => home::view(model),
        Page::StyleHome => home_style::view(model),
        Page::ButtonStyling => button_styling::view(model),
        Page::SimpleLayout => simple_layout::view(model),
        Page::LayoutComposition => layout_composition::view(model),
        Page::Theming => theming::view(model),
        Page::ResponsiveStyling => responsive_styling::view(model),
        Page::LoadTest => thousandtest::view(model),
        Page::GettingStarted => getting_started::view(model),
        Page::ExtendingSeed => extending_seed::view(model),
        Page::HooksHome => hooks_home::view(model),
        Page::HooksGettingStarted => hooks_getting_started::view(model),
        Page::HooksApi => hooks_api::view(model),
        Page::HooksTutorial => hooks_tutorial::view(model),

    }
}
