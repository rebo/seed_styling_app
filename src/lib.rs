#![feature(track_caller)]
use seed::{*, prelude::*};

use seed_hooks::*;
use seed_hooks::style::*;

//  Model, Msg, Update, init(), and start()
//  ---------------------------------------
struct Model {}


#[derive(Clone)]
enum Msg {
}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
}

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
        styles_on_args_counter(S.bg_indigo_600().font_size("20px")),
    ]
}

fn unstyled_counter() -> Node<Msg> {

    let counter = use_state(||0);

    div![
        h3!["Unstyled counter"],
        p![
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}


fn basic_styled_counter() -> Node<Msg> {

    let counter = use_state(||0);
    
    div![
        h3!["Basic Styled Counter"],
        // simple 'inline' defined styles with long property methods
        S.padding("4px").margin("2px").display("inline-block"),

        p![
            S.margin_top("4px"),
            "You have clicked ",
            counter,
            " times",
        ],

        button![
            S.padding_x("24px").padding_y("8px").margin("2px").background_color("teal").color("white").display("inline-block"),
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn css_styled_counter() -> Node<Msg> {

    let counter = use_state(||0);

    let div_style = S.css(r#"
        font-family: verdana;
        font-size: 20px;
        text-align: center;
        padding: 4px;
        margin: 2px;
    "#);

    let p_style = S.css(r#"
        margin-top: 4px;
        display: inline-block;
    "#);

    let button_style = S.css(r#"
        border-radius: 4px;
        background-color: #DB3080;
        color: white;
        padding: 14px;
        margin: 4px;
    "#);
    
    div![
        div_style,
        h3!["CSS Styled Counter"],

        p![
            p_style,
            "You have clicked ",
            counter,
            " times",
        ],

        button![
            button_style,
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}


fn conveinience_styled_counter() -> Node<Msg> {

    let counter = use_state(||0);

    let div_style = S.p("4px").m("2px");

    let p_style = S.mt("4px").display("inline-block");

    let button_style = S.radius("4px").
        bg_color("#DB3080").color("white")
        .px("20px").py("8px").m("4px");
    
    
    div![
        div_style,
        h3!["CSS Styled Counter"],

        p![
            p_style,
            "You have clicked ",
            counter,
            " times",
        ],

        button![
            button_style,
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}


fn tw_styled_counter() -> Node<Msg> {

    let counter = use_state(||0);
    
    div![
        S.p_1().m_1(),
        h3!["TW Styled Counter"],

        p![
            S.mt_1().inline_block(),
            "You have clicked ",
            counter,
            " times",
        ],

        button![
            S.bg_red_600().text_white().px_10().py_4().m_4(),
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}


fn hover_counter() -> Node<Msg> {

    let counter = use_state(||0);
    
    let button_style = S.bg_red_600().text_white().px_10().py_4().m_4();

    let hover_style = S.hover().bg_green_400();

    div![
        S.p_1().m_1(),
        h3!["Hover Styled Counter"],

        p![
            S.mt_1().inline_block(),
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

    let counter = use_state(||0);
    
    let button_style = S.bg_red_600().text_white().px_10().py_4().m_4();

    let media_query = S.media("@media only screen and (max-width: 600px)").bg_green_400();

    div![
        S.p_1().m_1(),
        h3!["Media Query Styled Counter - shrink width to less than 600px"],

        p![
            S.mt_1().inline_block(),
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

    let danger = use_state(|| false );
    let counter = use_state(|| 0);
    let base_button_style = S.bg_blue_600().text_white().px_10().py_4().m_4();

    let danger_style = base_button_style.bg_red_700();
    let ok_style = base_button_style.bg_green_500();

    div![
        S.p_1().m_1(),
        h3!["Variants of Styles"],

        p![
            S.mt_1().inline_block(),
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
            if danger.get() { danger_style} else {ok_style }, // group styles that need the same class
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}


fn themed_counter() -> Node<Msg> {

    let counter = use_state(||0);
    
    let pink_theme = Theme::default()
        .color("primary","#DB3080")
        .color("primary_light", "palevioletred")
        .color("secondary", "gray")
        .space_scale(["10px","20px","30px"])
        .radius("medium", "3px")
        .border_width_scale(["1px","2px","3px"]);

    div![
        use_theme(pink_theme, || 
            div![
                S.p_1().m_1(),
                h3!["With Theme"],

                p![
                    S.mt_1().inline_block(),
                    "You have clicked ",
                    counter,
                    " times",
                ],

                button![
                    S.p((3,"10px"))  // Padding will take the 3rd space theme value, or 10px if not present
                        .m((2, "8px"))  // Margin will take the 2nd space theme value, or 8px if not present
                        .bg_color(("primary","red"))  // The primary color value, or "red" if not present
                        .color("white")
                        .radius("2px")
                        .inline_block()
                    ,
                    "Increment Counter",
                    counter.on_click(|v| *v += 1)
                ]
            ]
        )
        ,
        div![
            S.p_1().m_1(),
            h3!["Without Theme"],

            p![
                S.mt_1().inline_block(),
                "You have clicked ",
                counter,
                " times",
            ],

            button![
                S.p((3,"10px"))  // Padding will take the 3rd space theme value, or 10px if not present
                    .m((2, "8px"))  // Margin will take the 2nd space theme value, or 8px if not present
                    .bg_color(("primary","red"))  // The primary color value, or "red" if not present
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

    let counter = use_state(||0);
    
    let button_style = S.bg_red_600().text_white().px_10().py_4().m_4();

    div![
        S.p_1().m_1(),
        h3!["Style From Arguments"],

        p![
            S.mt_1().inline_block(),
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








