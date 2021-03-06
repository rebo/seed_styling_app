use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_style::{pc, px,vh};
use seed_style::*;
use crate::Page;
use seed_hooks::*;
use crate::app_styling::theme::*;
////////////////////////////////////////

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum IntroAreas {
    HeroHeader,
    MainContent,
}
impl LayoutArea for IntroAreas {}


#[view_macro]
fn fancy_button_view<Ms>(mut root: Node<Ms>, children: Vec<Node<Ms>> )-> Node<Ms>{
    as_tag![
        button,
        root,
        s().background_color(seed_colors::Indigo::No6)
        .color(seed_colors::Base::White)
        .padding_x(&[px(24), px(32)])
        .padding_y(&[px(8), px(12)])
        .margin(px(8))
        .margin_top(px(18))
        .font_size(&[px(18), px(24)])
        .border_radius(px(3))
        .outline_none(),
        s().hover()
        .background_color(seed_colors::Blue::No5)
        .cursor_pointer(),
        children
    ]
}

#[view_macro]
fn center_view<Ms>( root: Node<Ms>, children: Vec<Node<Ms>>) -> Node<Ms> {
    root![
        s()
        .display_flex()
        .flex_direction_row()
        .justify_content_center()
        .align_items_center()
        ,
        children,
    ]
}


pub fn view(model: &Model) -> Node<Msg> {
    use IntroAreas::*;

    div![
        Composition::with_layout(
            Layout::areas(&[&[HeroHeader], &[MainContent]])
                .style(s().grid_template_rows("auto 1fr auto").min_height(pc(100.))),
        )
        .set_content(HeroHeader, |model| hero_header(model))
        .set_content(MainContent, main_content)
        .render(model)
    ]
}

fn main_content(model: &Model) -> Node<Msg> {
    Composition::with_layout(
    
        Layout::grid(s().grid_template_columns("1fr minmax(0px,1000px) 1fr")),
    )
    .add_child(|model: &Model| {
        div![
            s().grid_column_start("2")
                .grid_column_end("3")
                .padding_x(3)
                .padding_y(2),
            s().style_child("pre").justify_content_center(),
            p![
                "Use the full power of typed CSS directly within any Seed component. 
            Theme, context and media query aware mean your styles are effortlessly 
             reactive and in harmony with your design system."
            ],
            div![
                s().display_flex()
                    .flex_direction_row()
                    .justify_content_center(),
                button![
                    s().background_color(seed_colors::Blue::No6)
                        .color(seed_colors::Base::White)
                        .padding_x(&[px(24), px(32)])
                        .padding_y(&[px(8), px(12)])
                        .margin(px(8))
                        .margin_top(px(18))
                        .font_size(&[px(18), px(24)])
                        .border_radius(px(3))
                        .outline_none(),
                    s().hover()
                        .background_color(seed_colors::Blue::No5)
                        .cursor_pointer(),
                    s().active()
                        .background_color(seed_colors::Blue::No7)
                        .cursor_pointer(),
                    "Seed Rocks!"
                ]
            ],
            md![r#"

```
button![
    s().background_color(seed_colors::Blue::No6)
        .color(seed_colors::Base::White)
        .padding_x(&[px(24), px(32)])
        .padding_y(&[px(8), px(12)])
        .margin(px(8))
        .font_size(&[px(18), px(24)])
        .border_radius(px(3))
        .outline_none(),
    s().hover()
        .background_color(seed_colors::Blue::No5)
        .cursor_pointer(),
    s().active()
        .background_color(seed_colors::Blue::No7)
        .cursor_pointer(),
    "Seed Rocks!"
]
```
"#],h2!["Getting Started"],
md![r#"The best way to get started is to download the Seed Style quickstart,
 this targets current Seed master and includes `seed_style_preview`
 
 ```
 git clone https://github.com/rebo/seed-style-quickstart-basic
 cd seed-styling-quickstart
 cargo make start
```
 "#]
 ,h2!["Features"],
            Composition::with_layout(
                
                Layout::grid(
                    s().grid_template_columns("repeat(auto-fit, minmax(300px, 1fr))")
                        .grid_auto_flow_row()
                        .justify_items_center()
                        .grid_template_rows("auto 1fr")
                        .grid_gap(px(16))
                        .w(pc(100.)),
                )
            )
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Use styles scoped to individual components."])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Optional typing, lets you use traditional CSS or fully typed properties and values."])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Fully integrated themes let you swap styles at the press of a button."])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Responsive styles mean precise control over styles at set breakpoints"])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Customizable scales for a perfectly consistent design system"])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Atomic in nature, styles can be freely composed in sophisticated ways"])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Integrated full grid layout system, layouts and compositions are first class citizens"])
            .render(model),
            h2!["Examples"],
            div![s().display_flex().flex_direction_row(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::ButtonStyling )
                ],
                div![     
                    a![attrs!{At::Href => "/getting_started"}, h4!["Getting Started"]],
                    p!["Let's explore some basics of Seed Style by using the quickstart app"]
                ]
            ],
            div![s().display_flex().flex_direction_row(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::ButtonStyling )
                ],
                div![     
                    a![attrs!{At::Href => "/buttons"}, h4!["Button Styling Examples"]],
                    p!["See how to setup styles for buttons in a variety of contexts and use cases."],
                ]
            ],
            div![
                s().display_flex().flex_direction_row(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::LoadTest )
                ],
                div![     
                    a![attrs!{At::Href => "/theming"}, h4!["Theming Support"]],
                    p!["Seed Style provides first class theme support that is consistent with The Theme Specification. Find out how to define, create and use themes"]
                ]
            ],      
            div![
                s().display_flex().flex_direction_row(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::LoadTest )
                ],
                div![     
                    a![attrs!{At::Href => "/responsive_styling"}, h4!["Responsive Styling"]],
                    p!["Use the power of Seed Style to effortlessly style elements and components that need to be responsive to the device they are rendered on.  Mobile first development is straightforward and comprehensive."]
                ]
            ],

            div![s().display_flex(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::SimpleLayout )
                ],
                div![     
                    a![attrs!{At::Href => "/simple_layout"}, h4!["Simple Layout Primitives"]],
                    p!["Seed Style includes Row and Column layout primitives to make simple layout a breeze"],
                ]
            ],

            div![s().display_flex(),
            div![
                s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                model.page.on_click(|p| *p = Page::ExtendingSeed )
            ],
            div![     
                a![attrs!{At::Href => "/extending_seed"}, h4!["Extending Seed"]],
                p!["Create expressive UI DSLs by extending seed with styles and hooks."],
            ]
            ],
            div![s().display_flex(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::LayoutComposition )
                ],
                div![     
                    a![attrs!{At::Href => "/layout"}, h4!["Fully Integrated Layout"]],
                    p!["Seed Style includes full layout capabilities, this example demonstrates setting up a typical header-sidebar-content-footer layout. Rendered with inbuilt mock content."],
                ]
            ],
   
            div![
                s().display_flex().flex_direction_row(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::LoadTest )
                ],
                div![     
                    a![attrs!{At::Href => "/load_test"}, h4!["Load Test"]],
                    p!["Whilst extensive performance optimization is yet to be done Seed Style is reasonably performant. This page renders 1000 random new styles in sub 16ms in release mode."]
                ]
            ],       
        ]
    })
    .render(model)
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum HeroHeaderArea {
    
    Title,
    Subtitle,
    Empty,
}

impl LayoutArea for HeroHeaderArea {
    fn is_empty(&self) -> bool {
        self == &HeroHeaderArea::Empty
    }
}

fn hero_header(model: &Model) -> Node<Msg> {
    use HeroHeaderArea::*;
    
    div![
        a![attrs![At::Name=>"style"]],
        s().h(px(300)).background_image("linear-gradient(red, #f0aa06)")
        ,
        Composition::with_layout(
            Layout::areas(&[
                &[Empty, Empty, Empty],
                &[Empty, Title, Empty],
            &[Empty, Subtitle, Empty],
            ])
            .style(s().grid_template_columns("minmax(0,1fr) auto minmax(0,1fr)"))
            .area_style(Title, s().align_self_flex_end())
            .area_style(Subtitle, s().align_self_flex_start())
        )
        
        .set_content(Title, |_|
            div![
            s().color(seed_colors::Base::White)
                .font_weight_v900()
                .font_size(px(48)),
            "Seed Style"
        ])
        .set_content(Subtitle, |_| div![
            s().color(seed_colors::Base::White)
                .font_size(px(32))
                .font_weight_v300()
                .font_style_italic(),
            "Easy, powerful, styling for Seed"
        ])
        .render(model)
    ]
}
