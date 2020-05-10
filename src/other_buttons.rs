#![feature(track_caller)]
use seed::{prelude::*, *};

use seed_hooks::style::measures::px;
use seed_hooks::style::*;
use seed_hooks::*;

//  Model, Msg, Update, init(), and start()
//  ---------------------------------------
struct Model {}

#[derive(Clone)]
enum Msg {}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

//  View code here!
//  ---------------

#[topo::nested]
fn view(_model: &Model) -> impl IntoNodes<Msg> {
    p![
        unstyled_counter(),
        basic_styled_counter(),
        css_styled_counter(),
        conveinience_styled_counter(),
        tw_styled_counter(),
        hover_counter(),
        media_query_counter(),
        variant_counter(),
        themed_counter(),
        styles_on_args_counter(s().bg_indigo_600().font_size(
            seed_hooks::style::CssFontSize::StringValue("32px".to_string())
        )),
    ]
}

fn unstyled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    div![
        h3!["Unstyled counter"],
        p!["You have clicked ", counter, " times",],
        button!["Increment Counter", counter.on_click(|v| *v += 1)]
    ]
}

fn basic_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    div![
        h3!["Basic Styled Counter"],
        // simple 'inline' defined styles with long property methods
        s().padding("4px")
            .margin("2px")
            .display(CssDisplay::InlineBlock),
        p![
            s().margin_top("4px"),
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            s().padding_x("24px")
                .padding_y("8px")
                .margin("2px")
                .background_color("teal")
                .color("white")
                .display(CssDisplay::InlineBlock),
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn css_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let div_style = s().css(
        r#"
        font-family: verdana;
        font-size: 20px;
        text-align: center;
        padding: 4px;
        margin: 2px;
    "#,
    );

    let p_style = s().css(
        r#"
        margin-top: 4px;
        display: inline-block;
    "#,
    );

    let button_style = s().css(
        r#"
        border-radius: 4px;
        background-color: #DB3080;
        color: white;
        padding: 14px;
        margin: 4px;
    "#,
    );

    div![
        div_style,
        h3!["CSS Styled Counter"],
        p![p_style, "You have clicked ", counter, " times",],
        button![
            button_style,
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn conveinience_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let div_style = s().p("4px").m("2px");

    let p_style = s().mt("4px").display(CssDisplay::InlineBlock);

    let button_style = S
        .border_radius(CssBorderRadius::Length(px(2)))
        .bg_color("#DB3080")
        .color("white")
        .px("20px")
        .py("8px")
        .m("4px");

    div![
        div_style,
        h3!["Styled with shorter conveinience methods Counter"],
        p![p_style, "You have clicked ", counter, " times",],
        button![
            button_style,
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn tw_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    div![
        s().p_1().m_1(),
        h3!["TW Styled Counter"],
        p![
            s().mt_1().inline_block(),
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            s().bg_red_600().text_white().px_10().py_4().m_4(),
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn hover_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = s().bg_red_600().text_white().px_10().py_4().m_4();

    let hover_style = s().hover().bg_green_400();

    div![
        s().p_1().m_1(),
        h3!["Hover Styled Counter"],
        p![
            s().mt_1().inline_block(),
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            [button_style, hover_style], // group styles that need the same class
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn media_query_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = s().bg_red_600().text_white().px_10().py_4().m_4();

    let media_query = S
        .media("@media only screen and (max-width: 600px)")
        .bg_green_400();

    div![
        s().p_1().m_1(),
        h3!["Media Query Styled Counter - shrink width to less than 600px"],
        p![
            s().mt_1().inline_block(),
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            [button_style, media_query], // group styles that need the same class
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn variant_counter() -> Node<Msg> {
    let danger = use_state(|| false);
    let counter = use_state(|| 0);
    let base_button_style = s().bg_blue_600().text_white().px_10().py_4().m_4();

    let danger_style = base_button_style.bg_red_700();
    let ok_style = base_button_style.bg_green_500();

    div![
        s().p_1().m_1(),
        h3!["Variants of Styles"],
        p![
            s().mt_1().inline_block(),
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            base_button_style, // group styles that need the same class
            "Swap variants of other button",
            danger.on_click(|v| *v = !*v)
        ],
        button![
            if danger.get() { danger_style } else { ok_style }, // group styles that need the same class
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum Color {
    Primary,
    Secondary,
    Accent,
    Muted,
}

impl ColorTheme for Color {}

fn themed_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let pink_theme = Theme::default()
        .set(Color::Primary, rgba(255.0, 255.0, 10.0, 0.3))
        .set(Color::Secondary, rgba(30.0, 40.0, 50.0, 0.2));

    div![
        use_theme(pink_theme, || div![
            s().p_1().m_1(),
            h3!["With Theme"],
            p![
                s().mt_1().inline_block(),
                "You have clicked ",
                counter,
                " times",
            ],
            button![
                s().color((Color::Secondary, rgba(0.0, 0.0, 0.0, 0.0))) // Padding will take the 3rd space theme value, or 10px if not present
                    .m("8px") // Margin will take the 2nd space theme value, or 8px if not present
                    .bg_color((Color::Primary, rgba(0.0, 0.0, 0.0, 0.0))) // The primary color value, or "red" if not present
                    .color("white")
                    .radius(px(2))
                    .inline_block(),
                "Increment Counter",
                counter.on_click(|v| *v += 1)
            ]
        ]),
        div![
            s().p_1().m_1(),
            h3!["Without Theme"],
            p![
                s().mt_1().inline_block(),
                "You have clicked ",
                counter,
                " times",
            ],
            button![
                s().p("10px") // Padding will take the 3rd space theme value, or 10px if not present
                    .m("8px") // Margin will take the 2nd space theme value, or 8px if not present
                    .bg_color("primary") // The primary color value, or "red" if not present
                    .color("white")
                    .radius("2px")
                    .inline_block(),
                "Increment Counter",
                counter.on_click(|v| *v += 1)
            ]
        ]
    ]
}

fn styles_on_args_counter(user_style: seed_hooks::style::Style) -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = s().bg_red_600().text_white().px_10().py_4().m_4();

    div![
        s().p_1().m_1(),
        h3!["Style From Arguments"],
        p![
            s().mt_1().inline_block(),
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            [button_style, user_style], // group styles that need the same class
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}
