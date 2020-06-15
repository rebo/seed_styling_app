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

    div![div![s().h(vh(100)),
        Composition::with_layout(
            Layout::areas(&[&[HeroHeader], &[MainContent]])
                .style(s().grid_template_rows("auto 1fr auto").min_height(pc(100.))),
        )
        .set_content(HeroHeader, |model| hero_header_ss(model))
        .set_content(MainContent, main_content_ss)
        .render(model)
        ]
        ,

    ]

  
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

fn hero_header_ss(model: &Model) -> Node<Msg> {
    use HeroHeaderArea::*;
    div![
        s().h(px(300)).background_image("linear-gradient(darkgreen, #aaf006)")
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
            "Supercharge Seed"
        ])
        .set_content(Subtitle, |_| div![
            s().color(seed_colors::Base::White)
                .font_size(px(32))
                .font_weight_v300()
                .font_style_italic(),
            "With Seed Style & Seed Hooks"
        ])
        .render(model)
    ]
}


fn main_content_ss(model: &Model) -> Node<Msg> {
    Composition::with_layout(
    Layout::grid(s().grid_template_columns("1fr minmax(0px,1000px) 1fr")),
    )
    .add_child(|model: &Model| {
        div![
            s().grid_column_start("2")
                .grid_column_end("3")
                .padding_x(3)
                .padding_y(2),
                
            Composition::with_layout(
                Layout::grid(s().grid_template_columns("repeat(auto-fit,minmax(0px,450px))").grid_auto_flow_row().justify_content_center())
            ).add_child(|model: &Model|
                
                div![
                    s().py(px(8)).px(px(12))
                    .overflow_auto(),
                    h2!["Seed Style"],
                    p!["Quickly and easily set responsive styles to Seed elements.",
                        pre![s().display_flex().flex_direction_row().justify_content_center(),code![s().font_size(&[px(12),px(14)]),
r#"p![
  s().font_size(&[px(18), px(24)]),
  "Size 18 font on small screens",
  "and 24 on medium size screens"
]                       
"#
                        ]]
                    
                    ],
                    Row![Item![a![attrs![At::Href=>"/style_home"],"Let's go"], align = RowAlign::Right]]
                ]                
            ).add_child(|model: &Model|

            div![
                s().py(px(8)).px(px(12))
                .overflow_auto(),
                h2!["Seed Hooks"],
                p!["Use per-component state to simplify interactivity.",
                pre![s().display_flex().flex_direction_row().justify_content_center(),code![s().font_size(&[px(12),px(14)]),
r#"let votes = use_state(||0);
button![
  votes, " people voted",
  votes.on_click(|v| *v+=1)
]                     
"#
                ]]
                
                ],
                Row![Item![a![attrs![At::Href=>"/hooks_home"],"Let's go"  ] ,align = RowAlign::Right]],
            ]
            )
            .render(model)
        ]
        
    })
    .render(model)
}

