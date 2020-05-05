use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::measures::{px, rem};
use seed_style::*;

#[topo::nested]
pub fn view(_model: &Model) -> Node<Msg> {
    div![
        S.line_height(CssLineHeight::Number(1.6))
            .letter_spacing(rem(-0.00278)),
        S.if_nested_style("p code")
            .font_size(px(14))
            .my(px(2))
            .display_inline_block()
            .radius(px(3))
            .bg_color(seed_colors::Gray::No3)
            .py(px(2))
            .px(px(4)),
        S.if_direct_child_style("div")
            .display_flex()
            .flex_direction_column(),
        S.if_nested_style("button")
            .align_self_center()
            .outline_style_none(),
        S.if_nested_style("h3")
            .box_sizing_border_box()
            .font_family("roboto, Arial, sans-serif")
            .font_weight_v700()
            .b_style_solid()
            .bb_width(px(3))
            .b_color(seed_colors::Gray::No4)
            .display_block()
            .px(px(12))
            .py(px(8))
            .mb(px(4))
            .font_size(px(24)),
        S.if_nested_style("p")
            .font_family("Arial,x-locale-body,sans-serif")
            .webkit_font_smoothing_antialiased()
            .box_sizing_border_box()
            .display_block()
            .px(px(12))
            .py(px(8))
            .mx(px(8))
            .mb(px(4))
            .font_size(px(16)),
        S.if_nested_style("pre")
            .bg_color(seed_colors::Orange::No1)
            .b_style_dotted()
            .overflow_x_auto()
            .b_width(px(4))
            .b_color(seed_colors::Orange::No4)
            .box_sizing_border_box()
            .display_block()
            .px(px(16))
            .py(px(12))
            .mx(px(16))
            .my(px(8))
            .radius(px(5))
            .font_size(px(18)),
        unstyled_counter(),
        // (0..500).map(|_| basic_styled_counter()),
        basic_styled_counter(),
        css_styled_counter(),
        conveinience_styled_counter(),
        hover_counter(),
        media_query_counter(),
        variant_counter(),
        themed_counter(),
        styles_on_args_counter(S.bg_color(seed_colors::Indigo::No5).font_size(px(24))),
    ]
}

fn unstyled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    div![
        h3!["Unstyled counter"],
        md![
            r#"This is a basic unstyled button, it uses seed hooks to store state with `use_state(|| 0)`.
            The counter is incremented with `counter.on_click(|v| *v += 1)` which creates an `Ev::OnClick` EventHandler.
        "#
        ],
        p!["You have clicked ", counter, " times",],
        button!["Increment Counter", counter.on_click(|v| *v += 1)]
    ]
}

fn basic_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    div![
        h3!["Basic Styled Counter"],
        md![
            r#"This is a basic styled button, the button is styled with the following

```
S.padding_left(px(24))
    .padding_right(px(24))
    .padding_top(px(8))
    .padding_bottom("8px")
    .border_radius(px(4))
    .margin("2px")
    .background_color("teal")
    .color("white")
    .outline_style(CssOutlineStyle::None)
    .display(CssDisplay::InlineBlock)
```
Notice how the optional value typing works : `px(2)` is an `ExactLength` it can be used 
whenever a pixel measurement is needed. Other units include `pc()` `rem()` and `em()`.

Strings can be used for properties (e.g. `.padding_bottom("8px").  You can also use the specific Css value type directly. 
These types are enums, for instance demonstrated here with a `CssDisplay::InlineBlock` passed to the `.display()` method 
or the `CssOutlineStyle::None` passed to the `outline_style()` method.

The reason for prefering typed arguments is that typos are detected at compile time. For instance writing
this page I accidently typed `Outine` instead of `Outline`. If this was pure css the error would only be 
spotted if I had thorouly checked the resultant page and css for a property that is often easy to miss.

See `css_values.rs` for a complete list of properties and values available.
       "#
        ],
        S.padding("4px")
            .margin("2px")
            .display(CssDisplay::InlineBlock),
        p![S.margin_top("4px"), "You have clicked ", counter, " times",],
        button![
            S.padding_left(px(24))
                .padding_right(px(24))
                .padding_top(px(8))
                .padding_bottom(px(8))
                .border_radius(px(4))
                .margin(px(2))
                .background_color("teal")
                .color("white")
                .outline_style(CssOutlineStyle::None)
                .display(CssDisplay::InlineBlock),
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn css_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let div_style = S.raw(
        r#"
        padding: 4px;
        margin: 2px;
    "#,
    );

    let p_style = S.raw(
        r#"
        margin-top: 4px;
        display: inline-block;
    "#,
    );

    let button_style = S.raw(
        r#"
        font-family: verdana;
        font-size: 20px;
        display: inline-block;
        text-align: center;
        border-radius: 4px;
        background-color: #DB3080;
        color: white;
        padding: 16px;
        margin: 6px;"#,
    );

    div![
        div_style,
        h3!["Raw CSS Styled Counter"],
        md![
            r##"This is a button styled with css direcrly using the `.raw()` method.  We can include any css we want. In this case we have used:

```
S.raw(
    r#"
    font-family: verdana;
    font-size: 20px;
    display: inline-block;
    text-align: center;
    border-radius: 4px;
    background-color: #DB3080;
    color: white;
    padding: 16px;
    margin: 6px;
    "#
)   
```
Whilst this is conveinient please note that this css is not type checked in anyway, it is usually better to use type checked css 
so that any issues can be prevented at compile time.

       "##
        ],
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

    let div_style = S.p("4px").m("2px");

    let p_style = S.mt("4px").display(CssDisplay::InlineBlock);

    let button_style = S
        .radius(px(2))
        .bg_color(hsl(40, 70, 30))
        .color("white")
        .px(px(16))
        .py(px(12))
        .outline_none()
        .m(px(6));

    div![
        div_style,
        h3!["Styled with shorter conveinience methods"],
        md![
            r#"There are conviencience methods available  in order to make defining styles more efficient.
For instance `pl()` for `padding-left()`. Horizontal/ vertical padding and margins can be set with `.mx()`, 'py()` etc.

Furthermore all properties with single variant values can be set by appending the value name to the property method.

For instance, `.display_inline_block()` will in effect call `.display(CssDisplay::InlineBlock)` which generates the css `display: inline-block;`.

Similarly, `.flex_direction_column()` will call `.flex_direction(CssFlexDirection::Column)` which generates the css `flex-direction: column;`.

See `css_values.rs` for more information.

Here is an example of some shortcuts.
```
S.radius(px(2))
    .bg_color(hsl(40,70,30))
    .color("white")
    .px(px(16))
    .py(px(12))
    .outline_none()
    .m(px(6));
```
       "#
        ],
        p![p_style, "You have clicked ", counter, " times",],
        button![
            button_style,
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn hover_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = S
        .bg_color(hsl(200, 40, 30))
        .color(seed_colors::Base::White)
        .radius(px(4))
        .outline_none()
        .px(px(16))
        .py(px(12))
        .m(px(6));

    let hover_style = S
        .hover()
        .bg_color(seed_colors::Green::No4)
        .color(seed_colors::Base::Black);
    let active_style = S.active().bg_color(seed_colors::Green::No6);

    div![
        S.p(px(3)).m(px(3)),
        h3!["Hover and other Pseudo-selectors styled buttons"],
        md![
            r#" Unlike direct inline styles, you can correctly use pseudo-selectors
to style things like `:hover` status.  Simply use the pseudo selector name as a method `.hover()` method on a style.  Please note that this 
sets all properties defined in that style object to be affected by `:hover`.

Typically you would use two styles, one normal style and one a hover.
        
``` 
S.bg_color(hsl(200,40,30))
    .color("white")
    .outline_none()
    .px(px(16))
    .py(px(12))
    .m(px(6)),
S.hover().bg_color("green").color("black"),
S.active().bg_color("darkgreen")
```
"#
        ],
        p![
            S.mt(px(2)).display_inline_block(),
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            button_style,
            hover_style,
            active_style,
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn media_query_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = S
        .bg_color(seed_colors::Red::No4)
        .color(seed_colors::Base::Black)
        .px(px(16))
        .py(px(12))
        .radius(px(6))
        .m(px(12));

    let media_query = S
        .media("@media only screen and (max-width: 700px)")
        .bg_color(seed_colors::Green::No4);

    div![
        S.p(px(2)).m(px(2)),
        h3!["Media Query Styled Counter - shrink width to less than 700px"],
        md![r#"
Media queries can be used in a number of ways.  The most basic way is by using the `media()` method on a style. This will 
result in that style being nested within a media query block.

For instance :
```
S.media("@media only screen and (max-width: 700px)").bg_color("green")
``` 
will ensure that the background colour will be green if the screen is 700px or less wide.

There are a number of other (and better) ways to define media queries that involve themes and breakpoints. This will be discussed later.
        "#],
        p![
            S.m(px(2)).display_inline_block(),
            "You have clicked ",
            counter,
            " times (Shrink this page width!)",
        ],
        button![
            button_style,
            media_query,
            "Increment Counter",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn variant_counter() -> Node<Msg> {
    let danger = use_state(|| false);
    let counter = use_state(|| 0);
    let base_button_style = S
        .bg_color(seed_colors::Gray::No6)
        .color(seed_colors::Base::White)
        .px(px(16))
        .py(px(12))
        .radius(px(6))
        .m(px(12));

    let danger_style = base_button_style.bg_color(seed_colors::Red::No7);
    let ok_style = base_button_style.bg_color(seed_colors::Green::No5);

    div![
        S.p(px(2)).m(px(2)),
        h3!["Variants of Styles"],
        p![
            S.mt(px(4)).display_inline_block(),
            "You have clicked ",
            counter,
            " times",
        ],
        button![
            base_button_style, // group styles that need the same class
            "Click to swap variants of the other button",
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

fn pink_theme() -> Theme {
    Theme::default()
        .set_color(Color::Primary, hsl(200, 70, 50))
        .set_color(Color::Secondary, hsl(180, 60, 50))
}

fn themed_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    div![use_themes(
        || vec![pink_theme()],
        || div![
            S.p(px(2)).m(px(2)),
            h3!["With Theme"],
            p![
                S.mt(px(2)).display_inline_block(),
                "You have clicked ",
                counter,
                " times",
            ],
            button![
                S.color((Color::Secondary, rgba(0.0, 0.0, 0.0, 0.0))) // Padding will take the 3rd space theme value, or 10px if not present
                    .m("8px") // Margin will take the 2nd space theme value, or 8px if not present
                    .bg_color((Color::Primary, rgba(0.0, 0.0, 0.0, 0.0))) // The primary color value, or "red" if not present
                    .color("white")
                    .radius(px(2))
                    .display_inline_block(),
                "Increment Counter",
                counter.on_click(|v| *v += 1)
            ]
        ]
    ),]
}

fn styles_on_args_counter(user_style: seed_style::Style) -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = S
        .bg_color(seed_colors::Red::No6)
        .color(seed_colors::Base::White)
        .px(px(10))
        .py(px(8))
        .m(px(4));

    div![
        S.p(px(2)).m(px(2)),
        h3!["Style From Arguments"],
        p![
            S.mt(px(2)).display_inline_block(),
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
