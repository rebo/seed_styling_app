#![feature(track_caller)]
use seed::{prelude::*};
use seed_style::measures::{pc, px};
use seed_style::*;

// Before we set up our app, we define things like our themees and layout
// These can actually be completely reused in different apps because they are content agnostic
// 
// Normally they would be defined in separate files, they could even be in their own crates.


// Theme Definition 
// -----------------
//
// A Theme Object is where all css related values and presets can be stored
// and then accessed at any point in the view.
//
// The Theme Object is broadly consistent with the Theme specification that is used in
// several css librarys: https://theme-ui.com/theme-spec/
//
// A Theme object is made up of named css values called aliases
// as well as scales for css values/
//
// Having a scale is useful for things like sizes and spacing
// because you can have consistent layout throughout your app.  For instance pixel gaps 
// at 4px increments.
//
// Having named aliases for things like colors is useful because it means
// swapping out colors, or having a dark/light theme can be defined in a central location.
//
// In order to use cssvalue aliases we use an enum.
//
// // Main Color Theme Keys
#[derive(Hash, PartialEq, Eq, Clone)]
enum Color {
    Primary,
    DarkPrimary,
    Secondary,
    DarkSecondary,
    Highlight,
}
impl ColorTheme for Color {} // Allows you to use a `Color` variant as a CssColor key in the theme.

// Named Breakpoints Keys allow you to refer to a named breakpoint in layout helpers and css media queries.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Breakpoint {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}
impl BreakpointTheme for Breakpoint {} // Enable `Breakpoint` as a key type.

// The good thing about declaring the keys ahread of time is that it is easy for
// A user of the app / component to know what can be set to configure the style of the app

// WIth the keys declared, we can now actually define a theme that we want to use:
// We can have multiple themes for a single Theme definition if needed.

// The following value classes are themeable with named aliases:
//
//  BorderTheme,
//  BorderWidthTheme,
//  BorderStyleTheme,
//  SpaceTheme,
//  LineHeightTheme,
//  LetterSpacingTheme,
//  BorderRadiusTheme,
//  FontTheme,
//  FontSizeTheme,
//  SizeTheme,
//  TransitionTheme,
//  ZIndexTheme,
//  DisplayTheme,
//  ColorTheme,
//  ShadowTheme,
//  StyleTheme,
//  BreakpointTheme,


// We now write a function to provide an instance of a theme.
// A different function could provide a completely different theme
// For instance a dark mode theme.
fn my_theme() -> Theme {
    use Breakpoint::*;
    

    // I generally set the named aliases seperately from the theme scales:
    let theme = Theme::new()
        .set_color(Color::Primary, CssColor::Hsl(200.0, 70.0, 80.0))
        .set_color(Color::Secondary, hsl(300, 60, 50)) // or use the hsl shortcut
        .set_color(Color::Highlight, hsl(310, 70, 85))
        .set_color(Color::DarkPrimary, hsl(200, 70, 35))
        .set_color(Color::DarkSecondary, hsl(300, 60, 20))
        .set_breakpoint(ExtraSmall, (0, Some(600))) // Breakpoints are upper bound exclusive lower bound inclusive.
        .set_breakpoint(Small, (600, Some(960)))
        .set_breakpoint(Medium, (960, Some(1280)))
        .set_breakpoint(Large, (1280, Some(1920)))
        .set_breakpoint(ExtraLarge, (1920, None));

    //scales
    // https://styled-system.com/guides/array-scales/
    theme
        .space_scale(&[px(2), px(4), px(8), px(16), px(32)])
        .font_size_scale(&[px(14), px(18), px(20), px(36)])
        .breakpoint_scale([600, 960, 1280, 1920]) // standard-material-ui breakpoints
}
////////////////////////////////////////////////////
// Layout definition
// ------------------------------------------------
// The App will be laid out as follows.
//
// The main page will be a traditional header sidebar content & footer at larger screens.
// And a linear Header, collapsable nav, content and footer at smaller screens.
//
// The header is split into logo, title and actions at larger screens
// And title and hambuger button at smaller screens
//
// The Sidebar has a title plus repeatable elements
//
// The main content has repeatable content arranged in a in a grid.

// We first define all the possible areas that can be arranged on the main app page
// They will not all be rendered, this will determined on the specific breakpoint
//
// Layout makes use of css grid
// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum AppArea {
    Header,
    MainContent,
    Nav,
    Sidebar,
    Footer,
    None,
}
impl GridArea for AppArea {} // Allow Area to act as a GridArea for layout purposes.

// We now define some layouts to be used in the app.  The first a larger layout.
fn app_layout_large() -> Layout<AppArea> {
    use AppArea::*;
    
    #[cfg_attr(rustfmt, rustfmt_skip)] // ensure the layout array formatting matches the layout
    let layout = Layout::areas(&[
        &[ Header , Header     , Header     ],
        &[ Sidebar, MainContent, MainContent],
        &[ Footer , Footer     , Footer     ],
    ]);

    // `layout.style` applies a style to the grid container element
    // grid-template-rows set to `auto 1fr auto` to make the header and footer auto sized
    //  - the middle content row the largest possible size.
    // min-height 100% on the container to fill the app's container
    //
    layout.style(
        S.grid_template_rows("auto 1fr auto")
            .min_height(pc(100.))
            .grid_gap(px(8)),
    )
    // This layout definition is completely content agnostic.
    // Infact it can be used in any seed app that needs a similar arrangement of areas.
}

// Small layout for devices such as phones, column layout.
// Note here we have `Nav` conditionally rendered instead of `Sidebar`,
fn app_layout_small() -> Layout<AppArea> {
    use AppArea::*;

    #[cfg_attr(rustfmt, rustfmt_skip)] // ensure the layout array formatting matches the layout
     Layout::areas(
        &[
            &[Header     ], 
            &[Nav        ], 
            &[MainContent], 
            &[Footer     ],
    ])
        .style(
            S.grid_template_rows("auto auto 1fr auto").min_height(pc(100.))
        )
}

// We have just defined the layouts for the main page, however we can also declare
// our layouts for other parts of the page.
// All layouts are completely content agnostic.

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum HeaderArea {
    Title,
    Actions,
    Hamburger,
    Logo,
}
impl GridArea for HeaderArea {}

// We can actually use the build pattern to remove much cruft.
fn header_layout_large() -> Layout<HeaderArea> {
    use HeaderArea::*;

    Layout::areas(&[&[Logo, Title, Title, Title, Actions]]) // Horizontal layout of logo title and actions
        .style(S.grid_template_columns("auto 1fr 1fr 1fr auto"))
        .area_style(Actions, S.justify_self_right()) // ensure actions are justified to the right
        .area_style(Logo, S.justify_self_left()) // ensure the logo is justified to the left
        
}

fn header_layout_small() -> Layout<HeaderArea> {
    use HeaderArea::*;
    Layout::areas(&[&[Title, Hamburger]])
        .style(S.grid_template_columns("1fr auto"))
        .area_style(Hamburger, S.justify_self_right())
        
}


// The Sidebar layout - consists of a title and then filled with a list of travel destinations
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Sidebar {
    Title,
}

impl GridArea for Sidebar {}

fn sidebar_layout() -> Layout<Sidebar> {
    use Sidebar::*;
    
    #[cfg_attr(rustfmt, rustfmt_skip)] 
    Layout::areas(&[&[Title]]) // a spanning title followed by repeating unamed list of travel destinations
        .style( 
            S.grid_template_rows("auto 1fr")
                .grid_gap(px(16))
                .justify_items_center(),
        )
        
}

// The Main Content Layout  notice this has no named areas
// Just a repeating grid, therefore we use the 'NoArea' Area type.
fn main_content_layout() -> Layout<NoArea> {
    
    Layout::<NoArea>::grid()
        .style(
        S.grid_template_columns("repeat(auto-fit, minmax(250px, 1fr))")
            .grid_auto_flow_row()
            .justify_items_center()
            .grid_template_rows("auto 1fr")
            .grid_gap(px(16))
            .w(pc(100.)),
        )
        
}


// ------------------ End of Layout Definition ---------------------
//
// Reviewing the code above you will see that there is zero content described.
// In theory the above layouts in any app with similar layouts / Themes.
// These could be cut & pasted out freely, or be stored in their own crate.




// Now the Actual Seed app: 
//
//
//  Model, Msg, Update, init(), and start()
//  ---------------------------------------
struct Model {}

// We just need one Msg in order to handle event handlers bound to the window
// In this case a msg everytime the window is resized. This enables automatic
// conditional rendering on breakpoint changes
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
            // it is bassed in a block, because we only need it on first assignment.
            conditionally_skip_rendering::<Breakpoint,_,_,_>(|| my_theme(), orders)
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

fn view(model: &Model) -> impl IntoNodes<Msg> {
    use_themes(|| vec![my_theme(), style_presets()], || app_composiiton(model))
}

// The view configures and renders a `Composition` using the layouts defined above.
// each set content could acutally point to an indiviudal function.
fn app_composiiton(model: &Model) -> Node<Msg> {
    use AppArea::*;

    use Breakpoint::*;

    //  To enable differerent layouts on different breakpoints we add layouts to a composition and render
    //
    //  If no breakpoints match, the composition will choose the smallest one.
    let mut comp = Composition::with_layouts(&[
        (Small, app_layout_large()),
        (ExtraSmall, app_layout_small()),
    ]);

    // We set content views to the various named areas, the could be a composition or normal seed view
    comp.set_content(Header, |model| {
        Composition::with_layouts(&[
            (Small, header_layout_large()),
            (ExtraSmall, header_layout_small()),
        ])
        .render(model)
    });
    comp.set_content(Sidebar, |model| {
        let mut c = Composition::with_layout(ExtraSmall, sidebar_layout());

        // composition includes a mock_children helper which helps render mulltiple children divs
        // this is useful when planning out the layout
        //
        // once you are ready to create the child you would use the `add_child` method to add a child node directly
        c.mock_children("Dest.", 20, px(400), px(100));

        c.render(model)
    });
    comp.set_content(MainContent, |model| {
        let mut c = Composition::with_layout(ExtraSmall, main_content_layout());
        c.mock_children("Photo", 20, px(250), px(250));
        c.render(model)
    });
    // notice we have not hooked up Footer with anything yet...

    // Also notice we have not actually really created any content, we have just worked on the generate layout.
    comp.render(model)
}




//

//
//
//
//
//
//