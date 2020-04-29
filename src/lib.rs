#![feature(track_caller)]
use seed::{prelude::*, *};
use seed_style::measures::{pc, px};
use seed_style::*;

//  Model, Msg, Update, init(), and start()
//  ---------------------------------------
struct Model {}

// We just need one Msg in order to handle event handlers bound to the window/
// In this case a msg everytime the window is resized.
#[derive(Clone)]
pub enum Msg {
    WindowResized,
}

// No need for update to do anything the WindowResized Msg doesn't need to be
// handled, its enough for a re-render to be triggered
fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

// We subscribe to a window resize event in the init.
fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.stream(streams::window_event(Ev::Resize, |_| Msg::WindowResized));
    Model {}
}

// Default app start...
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

// Theme Definition Here
// ---------------------
//
// A Theme Object is where all css related values and presets can be stored
// and then accessed at any point in the view.
//
// The Theme Object is broadly consistent with the Theme specification that is used in
// several css librarys: https://theme-ui.com/theme-spec/
//
// A Theme object is made up of named css values called aliases
// as well as unnamed scales
//
// Having a scale is useful for things like sizes and spacing
// because you can have consistent layout throughout your app.
//
// Having named aliases for things like colors is useful because it means
// swapping out colors, or having a dark/light theme can be done in a central location.
//
// In order to use cssvalue aliases we need to identify the names by using an enum
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
#[derive(Hash, PartialEq, Eq, Clone)]
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
fn my_theme() -> Theme {
    use Breakpoint::*;
    use Color::*;

    // I generally set the aliases seperately from the theme scales:
    let mut theme = Theme::new()
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
        .font_size_scale(&[px(12), px(14), px(16), px(36)])
        .breakpoint_scale([600, 960, 1280, 1920]) // standard-material-ui breakpoints
}

// Main Layout
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Area {
    Header,
    MainContent,
    Nav,
    SideBar,
    Footer,
    None,
}
impl CssArea for Area {} // Allow Area to act as a CssArea for layout purposes.

//
//  View Entry Here, Sets up theme access, two themes are allowed access
//
//  The first is the app defined theme, the second provides access to seed style presets.
//  (At present `Theme` is not Clone therefore need to pass in as owned Vec)
//
//  ---------------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    use_themes(|| vec![my_theme(), style_presets()], || themed_view(model))
}

//
// We now define some layouts to be used in the app.  The first a larger layout.
//
fn large_layout(_model: &Model) -> SeedLayout<Area, Model, Msg> {
    use Area::*;

    #[cfg_attr(rustfmt, rustfmt_skip)] // ensure the layout array formatting matches the layout
    let mut layout = SeedLayout::areas(&[
        &[ Header , Header     , Header     ],
        &[ SideBar, MainContent, MainContent],
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
    );

    // In theory up to this point the layout is content agnostic.
    // Infact this "layout" could be stored and reused in other applications.

    // We assign the Header area to the `header()` view.
    layout.set(Header, |model| header(model));
    layout
}

// Small layout for devices such as phones, column layout.
// Note here we have `Nav` conditionally rendered instead of `SideBar`,
fn small_layout(_model: &Model) -> SeedLayout<Area, Model, Msg> {
    use Area::*;

    #[cfg_attr(rustfmt, rustfmt_skip)] // ensure the layout array formatting matches the layout
    let mut layout = SeedLayout::areas(
        &[
            &[Header     ], 
            &[Nav        ], 
            &[MainContent], 
            &[Footer     ],
    ]);

    layout.style(
        S.grid_template_rows("auto auto 1fr auto")
            .min_height(pc(100.)),
    );

    // Again if we wanted we could re-use the above single column layout.

    layout.set(Header, |model| header(model));
    layout
}

/////////// End of layout definiitons

fn themed_view(model: &Model) -> Node<Msg> {
    use Breakpoint::*;
    //  To enable differerent layouts on breakpoints we add layouts toa  componsition and render
    //  If we were using only one layout we could render it directly.
    //
    //  If no breakpoints match, the componsition will choose the smallest one.
    //
    let mut comp = Composition::default();
    comp.add(Small, large_layout(model)); // renders large_layout on medium and above breakpoints
    comp.add(ExtraSmall, small_layout(model)); // renders small_layout on small and above breakpoints
    comp.render(model)
}

// The above code has setup our themes and conditionally renders a traditional header/sidebar/footer/main layour, or a column layout

// The Header Layout
//
// we want to mock up a website that displays holiday destinations.
// a specific destination can be selected in the Sidebar / nav bar
// cards of that holiday designation will appear in the main content
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum HeaderAreas {
    Title,
    Actions,
    Hamburger,
    Logo,
}

impl CssArea for HeaderAreas {}

fn header(model: &Model) -> Node<Msg> {
    use HeaderAreas::*;

    let mut layout = SeedLayout::areas(&[&[Logo, Title, Title, Title, Actions]]);

    layout.style(S.grid_template_columns("auto 1fr 1fr 1fr auto"));

    let mut small_layout = SeedLayout::areas(&[&[Title, Hamburger]]);

    small_layout.style(S.grid_template_columns("1fr auto"));

    let mut comp = Composition::default();
    comp.add(Breakpoint::ExtraSmall, small_layout);
    comp.add(Breakpoint::Small, layout);
    comp.render(model)
}

// The Sidebar layout
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum SideBarAreas {
    Title,
    ListOfDestinations,
}
impl CssArea for SideBarAreas {}

fn sidebar(model: &Model) -> Node<Msg> {
    use SideBarAreas::*;
    let mut layout = SeedLayout::areas(&[&[Title], &[ListOfDestinations]]);
    layout.style(S.grid_template_rows("auto 1fr").grid_gap(px(2)));
    layout.render(model)
}

// The Main Content Layout
fn main_content(model: &Model) -> Node<Msg> {
    let mut layout: SeedLayout<NoArea, _, _> = SeedLayout::<NoArea, _, _>::grid();

    layout.style(
        S.grid_template_columns("repeat(auto-fill, minmax(250px, 1fr))")
            .grid_auto_flow_row()
            .grid_template_rows("auto 1fr")
            .grid_gap(px(4)),
    );

    layout.mock_children("Photo", 15, px(300), px(200));

    layout.render(model)
}

//
//
//
//
//
//
//
//
//
//
//
//

//
//
//
//
//
//

//
//
//
//
//
//

//
//
//
//
//
//

//
//
//
//
//
//

use std::collections::HashMap;
use std::hash::Hash;

trait SeedLayoutTraitObject<Mdl, Ms> {
    fn render_layout(&self, model: &Mdl) -> Node<Ms>;
}

impl<A, Mdl, Ms> SeedLayoutTraitObject<Mdl, Ms> for SeedLayout<A, Mdl, Ms>
where
    A: CssArea,
{
    fn render_layout(&self, model: &Mdl) -> Node<Ms> {
        self.render(model)
    }
}

struct Composition<T, Mdl, Ms>
where
    T: BreakpointTheme,
{
    layouts: Vec<Box<dyn SeedLayoutTraitObject<Mdl, Ms>>>,
    default_idx: Option<usize>,
    layouts_hm: HashMap<T, usize>,
}

impl<T, Mdl, Ms> Default for Composition<T, Mdl, Ms>
where
    T: BreakpointTheme,
{
    fn default() -> Self {
        Self {
            layouts: vec![],
            default_idx: None,
            layouts_hm: HashMap::new(),
        }
    }
}

impl<T, Mdl, Ms> Composition<T, Mdl, Ms>
where
    T: BreakpointTheme + 'static,
    Ms: 'static,
    Mdl: 'static,
{
    fn add<A>(&mut self, bp: T, layout: SeedLayout<A, Mdl, Ms>)
    where
        A: CssArea,
    {
        self.layouts.push(Box::new(layout));
        let idx = self.layouts.len() - 1;
        // for bp in bps {
        self.layouts_hm.insert(bp.clone(), idx);
        // }
    }

    fn set_default<A>(&mut self, layout: SeedLayout<A, Mdl, Ms>)
    where
        A: CssArea,
    {
        self.layouts.push(Box::new(layout));
        self.default_idx = Some(self.layouts.len() - 1);
    }

    fn render(&self, model: &Mdl) -> Node<Ms> {
        // sorted breakpoints

        let mut sorted_bps = self.layouts_hm.keys().cloned().collect::<Vec<T>>();
        sorted_bps.sort_unstable_by_key(|bp_key| {
            let bp_pair = with_themes(|borrowed_themes| {
                borrowed_themes
                    .iter()
                    .find_map(|theme| theme.get::<T, (u32, Option<u32>)>(bp_key.clone().clone()))
                    .unwrap()
            });
            -(bp_pair.0 as i32)
        });

        // We find the biggest breakpoint that fits...

        // find the first layout which
        let opt_layout = sorted_bps
            .iter()
            .map(|bp_key| {
                (
                    with_themes(|borrowed_themes| {
                        borrowed_themes
                            .iter()
                            .find_map(|theme| {
                                theme.get::<T, (u32, Option<u32>)>(bp_key.clone().clone())
                            })
                            .unwrap()
                    }),
                    self.layouts_hm.get(bp_key),
                )
            })
            .find(|(bp_pair, layout)| match bp_pair {
                (lower, _) => window()
                    .match_media(&format!("(min-width: {}px)", lower))
                    .unwrap()
                    .unwrap()
                    .matches(),
            });

        if let Some((_bp_pair, Some(idx))) = opt_layout {
            self.layouts[*idx].render_layout(model)
        } else {
            if let Some(idx) = self.default_idx {
                self.layouts[idx].render_layout(model)
            } else {
                let smallest_bp_key = sorted_bps.last().unwrap();
                let idx_of_smallest_layout = self.layouts_hm.get(smallest_bp_key).unwrap();
                self.layouts[*idx_of_smallest_layout].render_layout(model)
            }
        }

        // let opt_layout_idx = self.layouts_hm.iter().find(move |(bp_key, layout)| {
        //     let bp_pair = with_themes(|borrowed_themes| {
        //         borrowed_themes
        //             .iter()
        //             .find_map(|theme| theme.get::<T, (u32, Option<u32>)>(bp_key.clone().clone()))
        //             .unwrap()
        //     });

        //     match bp_pair {
        //         (lower, Some(higher)) => window()
        //             .match_media(&format!(
        //                 "(min-width: {}px) and (max-width: {}px)",
        //                 lower, higher
        //             ))
        //             .unwrap()
        //             .unwrap()
        //             .matches(),
        //         (lower, None) => window()
        //             .match_media(&format!("(min-width: {}px)", lower))
        //             .unwrap()
        //             .unwrap()
        //             .matches(),
        //     }
        // });

        // if let Some((_, idx)) = opt_layout_idx {
        //     self.layouts[*idx].render_layout(model)
        // } else {
        //     self.layouts[self.default_idx].render_layout(model)
        // }
    }
}

use std::marker::PhantomData;

#[derive(Default)]
pub struct SeedLayout<A, Mdl, Ms>
where
    A: CssArea,
{
    areas: Vec<A>,
    mocked_children: Option<(String, u32, ExactLength, ExactLength)>,
    children: Vec<Box<dyn Fn(&Mdl) -> Node<Ms> + 'static>>,
    layout: Vec<Vec<A>>,
    areas_hm: HashMap<A, Box<dyn Fn(&Mdl) -> Node<Ms> + 'static>>,
    container_styles: Option<seed_style::Style>,
    area_styles: HashMap<A, seed_style::Style>,
    _phantom_data: PhantomData<A>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum NoArea {}
impl CssArea for NoArea {}

impl<A, Mdl, Ms> SeedLayout<A, Mdl, Ms>
where
    A: CssArea,
{
    pub fn set<F: Fn(&Mdl) -> Node<Ms> + 'static>(&mut self, area: A, area_view: F) -> &mut Self {
        let boxed_area_view = Box::new(area_view);
        self.areas_hm.insert(area, boxed_area_view);
        self
    }

    pub fn style(&mut self, style: seed_style::Style) {
        self.container_styles = Some(style);
    }

    pub fn area_style(&mut self, area: A, style: seed_style::Style) {
        self.area_styles.insert(area, style);
    }

    pub fn areas(layout_array: &[&[A]]) -> Self {
        let mut areas = vec![];
        let mut layout = vec![];
        for row in layout_array {
            let mut inner_vec_layout = vec![];
            for area in row.iter().cloned() {
                if !areas.contains(&area) {
                    if !area.is_empty() {
                        areas.push(area.clone());
                    }
                }
                inner_vec_layout.push(area.clone());
            }
            layout.push(inner_vec_layout);
        }

        SeedLayout {
            areas,
            mocked_children: None,
            layout,
            children: vec![],
            areas_hm: HashMap::new(),
            container_styles: None,
            area_styles: HashMap::new(),
            _phantom_data: PhantomData,
        }
    }

    pub fn grid() -> SeedLayout<NoArea, Mdl, Ms> {
        SeedLayout::<NoArea, Mdl, Ms> {
            areas: vec![],
            layout: vec![],
            mocked_children: None,
            children: vec![],
            areas_hm: HashMap::new(),
            container_styles: None,
            area_styles: HashMap::new(),
            _phantom_data: PhantomData::<NoArea>,
        }
    }

    pub fn mock(&self, area: A) -> Node<Ms> {
        div![
            S.box_sizing_border_box()
                .font_size(px(24))
                .text_align_center()
                .p(px(30))
                .height(pc(100.))
                .border_style_dashed()
                .border_width(px(2))
                .display_flex()
                .align_items_center()
                .justify_content_center()
                .border_color(seed_colors::Gray::No7)
                .bg_color(seed_colors::Gray::No5),
            h1![format!("{:?} ", area).replace("::", "__")]
        ]
    }

    pub fn mock_children(
        &mut self,
        name: &str,
        count: u32,
        width: ExactLength,
        height: ExactLength,
    ) {
        self.mocked_children = Some((name.to_string(), count, width, height));
    }

    pub fn render(&self, model: &Mdl) -> Node<Ms> {
        // Some calculations ..
        // Some calculations ..
        if self.layout.len() > 0 {
            self.render_areas(model)
        } else {
            self.render_grid(model)
        }
    }

    pub fn add<F: Fn(&Mdl) -> Node<Ms> + 'static>(&mut self, child_view: F) -> &mut Self {
        let boxed_child_view = Box::new(child_view);
        self.children.push(boxed_child_view);
        self
    }

    pub fn render_grid(&self, model: &Mdl) -> Node<Ms> {
        div![
            S.grid_template_columns("1 fr ")
                .box_sizing("border_box")
                .display_grid(),
            if let Some(styles) = &self.container_styles {
                styles.clone()
            } else {
                seed_style::Style::default()
            },
            if let Some(mock) = &self.mocked_children {
                let (name, count, width, height) = mock;
                (0..*count)
                    .into_iter()
                    .map(|i| {
                        div![
                            S.box_sizing_border_box()
                                .w(width.clone())
                                .h(height.clone())
                                .font_size(px(24))
                                .text_align_center()
                                .p(px(30))
                                .height(pc(100.))
                                .border_style_dashed()
                                .border_width(px(2))
                                .display_flex()
                                .align_items_center()
                                .justify_content_center()
                                .border_color(seed_colors::Gray::No7)
                                .bg_color(seed_colors::Red::No4),
                            h1![format!("Mocked {} No.{} ", name, i)]
                        ]
                    })
                    .collect::<Vec<Node<Ms>>>()
            } else {
                self.children
                    .iter()
                    .map(|child| child(model))
                    .collect::<Vec<Node<Ms>>>()
            }
        ]
    }

    pub fn render_areas(&self, model: &Mdl) -> Node<Ms> {
        let number_of_columns = self.layout.iter().map(|v| v.len()).max().unwrap();
        let number_of_rows = self.layout.len();
        let one_frs = std::iter::repeat("1fr ");
        let grid_template_columns = one_frs.take(number_of_columns).collect::<String>();

        let mut grid_template_areas = String::new();

        for row in &self.layout {
            let mut grid_template_areas_row = String::from("\"");

            for area in row {
                if area.is_empty() {
                    grid_template_areas_row.push_str(" . ");
                } else {
                    grid_template_areas_row.push_str(&format!("{:?} ", area).replace("::", "__"));
                }
            }

            grid_template_areas_row.push_str("\"");
            grid_template_areas.push_str(&grid_template_areas_row);
        }
        div![
            S.grid_template_columns(grid_template_columns.as_str())
                .box_sizing("border_box")
                .display_grid()
                .grid_template_areas(grid_template_areas.as_str()),
            if let Some(styles) = &self.container_styles {
                styles.clone()
            } else {
                seed_style::Style::default()
            },
            self.areas.iter().map(|area| {
                div![
                    if let Some(styles) = self.area_styles.get(area) {
                        styles.clone()
                    } else {
                        seed_style::Style::default()
                    }
                    .grid_area(format!("{:?} ", area).replace("::", "__").as_str())
                    .name(format!("{:?}_wrapper", area).as_str()),
                    if let Some(view) = self.areas_hm.get(area) {
                        view(model)
                    } else {
                        self.mock(area.clone())
                    },
                ]
            })
        ]
    }
}

fn themed_button() -> Node<Msg> {
    div![
        p![
            S.is_direct_child_of("div").color(Color::Primary),
            "Is this the child of a div?"
        ],
        button![
            S.background_color(Color::Primary)
                .color(seed_colors::Red::No6)
                .border_radius(px(3))
                .letter_spacing_normal()
                .pr(px(5))
                .border_color(Color::Highlight)
                .border_width(px(2))
                .outline_style_none(),
            S.except(Breakpoint::Large).color(hsl(200, 85, 60)),
            S.hover()
                .background_color(Color::DarkPrimary)
                .color(Color::DarkSecondary)
                .border_width(px(4))
                .outline_none(),
            "Clicked ",
            " times",
        ]
    ]
}

fn shorter_styles_button() -> Node<Msg> {
    div![
        button![
            S.background_color(&[
                hsl(10, 80, 20),
                hsl(10, 80, 50),
                hsl(10, 80, 80),
                hsl(10, 80, 90),
            ])
            .color(Color::Secondary)
            .radius(px(3))
            // .px(Space::Large)
            // .py(Space::Medium)
            .border_color(Color::Highlight)
            .border_width(px(2))
            .outline_none(),
            S.hover().b_width(px(4)).outline_none(),
            "Clicked ",
            " times",
        ],
        only(Breakpoint::Large, || div![
            S.w(px(80)).bg_color(Color::Primary),
            "Rendered only on large Screens"
        ])
    ]
}
pub trait CssArea: Hash + PartialEq + Eq + std::fmt::Debug + Clone + 'static {
    fn is_empty(&self) -> bool {
        false
    }
}
