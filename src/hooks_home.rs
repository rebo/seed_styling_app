use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_style::{pc, px};
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

pub fn view(model: &Model) -> Node<Msg> {
    use IntroAreas::*;

    Composition::with_layout(
        Layout::areas(&[&[HeroHeader], &[MainContent]])
            .style(s().grid_template_rows("auto 1fr auto").min_height(pc(100.))),
    )
    .set_content(HeroHeader, |model| hero_header(model))
    .set_content(MainContent, main_content)
    .render(model)
}

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
fn center_view<Ms>(mut root: Node<Ms>, children: Vec<Node<Ms>>) -> Node<Ms> {
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

fn main_content(model: &Model) -> Node<Msg> {
    Composition::with_layout(
    
        Layout::grid(s().grid_template_columns("1fr minmax(400px,1000px) 1fr")),
    )
    .add_child(|model: &Model| {
        div![
            s().grid_column_start("2")
                .grid_column_end("3")
                .padding_x(3)
                .padding_y(2),
            p![
                "Easily implement per component state in your Seed apps to create re-usable interactive views."
            ],
            center![
                {
                    let counter = use_state(||1337);
                    
                    fancy_button![
                        "Seed Rocks ", counter, " times!",
                        counter.on_click(|c| *c+=1),
                    ]
        }              
            ],
            md![r#"

```
let counter = use_state(||1337);

fancy_button![
    "Seed Rocks ", counter, " times!",
    counter.on_click(|c| *c+=1),
]
```

"#],h2!["Getting Started"],
md![r#"The best way to get started is to download the Seed Hooks quickstart,
 this targets current Seed master and includes `seed_hooks`
 
 ```
 git clone https://github.com/rebo/seed-hooks-quickstart.git
 cd seed-hooks-quickstart
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
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Use per state component."])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Easily cache and memoize complex values"])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Trigger effects a single time, for instance on render"])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Use patterns such as React's `useReducer`"])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "easily update state from EventHandlers"])
            .add_child(|_| div![s().w(px(300)).font_weight_v700(), "Atomic in nature, rich interactive components can be build from self contained building blocks"])
            .render(model),
            h2!["Guides and Examples"],
            div![s().display_flex().flex_direction_row(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::HooksGettingStarted )
                ],
                div![     
                    a![attrs!{At::Href => "/hooks_getting_started"}, h4!["Getting Started"]],
                    p!["Let's explore some basics of Seed Hooks by using the quickstart app"]
                ]
            ],
            div![s().display_flex().flex_direction_row(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::ButtonStyling )
                ],
                div![     
                    a![attrs!{At::Href => "/hooks_api"}, h4!["Hooks Api Notes"]],
                    p!["See what functions and methods are available to interact with Seed Hooks"],
                ]
            ],
            div![
                s().display_flex().flex_direction_row(),
                div![
                    s().bg_color(Color::Primary).w(px(80)).flex_none().h(px(80)).m(px(12)).mt(px(0)).radius(pc(80)),
                    model.page.on_click(|p| *p = Page::LoadTest )
                ],
                div![     
                    a![attrs!{At::Href => "/hooks_tutorial"}, h4!["Hooks Tutoral"]],
                    p!["We create a live markdown preview component using seed hooks"]
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
        s().h(px(300)).background_image("linear-gradient(purple, royalblue)")
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
        .set_content(Title, |_| div![
            s().color(seed_colors::Base::White)
                .font_weight_v900()
                .font_size(px(48)),
            "Seed Hooks"
        ])
        .set_content(Subtitle, |_| div![
            s().color(seed_colors::Base::White)
                .font_size(px(32))
                .font_weight_v300()
                .font_style_italic(),
            "Easy State for Seed Components"
        ])
        .render(model)
    ]
}
