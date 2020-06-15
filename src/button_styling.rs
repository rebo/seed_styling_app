use crate::compositions::*;
use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::px;
use seed_style::*;

pub fn view(model: &Model) -> Node<Msg> {
    render_centred_article(model, |_| {
        div![
            h1!["Button Styling"],
        
            p![
                r#"Using Seed Style we can apply styles in many different ways. This page demonstrates some applications that include 
the use of pseudo classes, media queries, and variant styling."#
            ],
            unstyled_counter(),
            basic_styled_counter(),
            css_styled_counter(),
            convenience_styled_counter(),
            hover_counter(),
            media_query_counter(),
            variant_counter(),
            styles_on_args_counter(s().bg_color(seed_colors::Indigo::No5).font_size(px(24))),
        ]
    })
}

fn unstyled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    div![
        s().display_flex().flex_direction_column(),
        h2!["Unstyled Button"],
        button![
            "Clicked (",
            counter,
            ") times",
            counter.on_click(|v| *v += 1)
        ],
        md![
            r#"This is a unstyled button, it uses seed hooks to store state with `use_state(|| 0)`.
                The counter is incremented with `counter.on_click(|v| *v += 1)` which creates an `Ev::OnClick` EventHandler.
                All the other buttons in this page use this simple pattern.
```
let counter = use_state(|| 0);
button![
    "Clicked (", counter, ") times",
    counter.on_click(|v| *v += 1 )
]
```                
            "#
        ],
    ]
}
#[topo::nested]
fn basic_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);
    div![
        s().display_flex().flex_direction_column(),
        h2!["Basic Styled Button"],
        button![
            s().padding_left(px(24))
                .padding_right(px(24))
                .padding_top(px(8))
                .padding_bottom(px(8))
                .border_radius(px(4))
                .margin(px(2))
                .background_color("teal")
                .color("white"),
            "Clicked (",
            counter,
            ") times",
            counter.on_click(move |v| *v += 1)
        ],
        md![
            r#"This is a basic styled button, the button is styled with the following:

```rust
s().padding_left(px(24))
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

Strings can be used for properties (e.g. `.padding_bottom("8px")`).  You can also use the specific Css value type directly. 
These types are enums, for instance demonstrated here with a `CssDisplay::InlineBlock` passed to the `.display()` method 
or the `CssOutlineStyle::None` passed to the `outline_style()` method.

The reason for preferring typed arguments is that typos are detected at compile time. For instance writing
this page I accidentally typed `Outine` instead of `Outline`. If this was pure css the error would only be 
spotted if I had thoroughly checked the resultant page and css for a property that is often easy to miss.

See `css_values.rs` for a complete list of properties and values available.
       "#
        ],
    ]
}

fn css_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = s().raw(
        r#"
        font-family: Verdana;
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
        s().display_flex().flex_direction_column(),
        h2!["Raw CSS Styled Button"],
        button![
            button_style,
            "Clicked (",
            counter,
            ") times",
            counter.on_click(|v| *v += 1)
        ],
        md![
            r##"This is a button styled with css directly using the `.raw()` method.  We can include any css that can be used inside a style block. In this case we have used:

```rust
s().raw(
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
Whilst this is convenient please note that this css is not type checked in anyway, it is usually better to use type checked css 
so that any issues can be prevented at compile time.

       "##
        ],
    ]
}

fn convenience_styled_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = s()
        .radius(px(2))
        .bg_color(hsl(40, 70, 30))
        .color("white")
        .px(px(16))
        .py(px(12))
        .outline_none()
        .m(px(6));

    div![
        s().display_flex().flex_direction_column(),
        h2!["Styled with shorter convenience methods"],
        md![
            r#"There are convenience methods available  in order to make defining styles more efficient.
For instance `pl()` for `padding-left()`. Horizontal/ vertical padding and margins can be set with `.mx()`, `py()` etc.

Furthermore all properties with single variant values can be set by appending the value name to the property method.

For instance, `.display_inline_block()` will in effect call `.display(CssDisplay::InlineBlock)` which generates the css `display: inline-block;`.

Similarly, `.flex_direction_column()` will call `.flex_direction(CssFlexDirection::Column)` which generates the css `flex-direction: column;`.

See `css_values.rs` for more information.

Here is an example of some shortcuts:

```rust
s().radius(px(2))
    .bg_color(hsl(40,70,30))
    .color("white")
    .px(px(16))
    .py(px(12))
    .outline_none()
    .m(px(6));
```
       "#
        ],
        button![
            button_style,
            "Clicked (",
            counter,
            ") times",
            counter.on_click(|v| *v += 1)
        ]
    ]
}

fn hover_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = s()
        .bg_color(hsl(200, 40, 30))
        .color(seed_colors::Base::White)
        .radius(px(4))
        .outline_none()
        .px(px(16))
        .py(px(12))
        .m(px(6));

    let hover_style = s()
        .hover()
        .bg_color(seed_colors::Green::No4)
        .color(seed_colors::Base::Black);
    let active_style = s().active().bg_color(seed_colors::Green::No6);

    div![
        s().display_flex().flex_direction_column(),
        h2!["Hover and other Pseudo-selectors styled buttons"],
        button![
            button_style,
            hover_style,
            active_style,
            "Clicked (",
            counter,
            ") times",
            counter.on_click(|v| *v += 1)
        ],
        md![
            r#" Unlike direct inline styles, you can correctly use pseudo-selectors
to style things like `:hover` status.  Simply use the pseudo selector name as a method `.hover()` method on a style.  Please note that this 
sets all properties defined in that style object to be affected by `:hover`.

Typically you would use two styles, one normal style and one a hover:
        
```rust
s().bg_color(hsl(200,40,30))
    .color("white")
    .outline_none()
    .px(px(16))
    .py(px(12))
    .m(px(6)),
s().hover().bg_color("green").color("black"),
s().active().bg_color("darkgreen")
```
"#
        ],
    ]
}

fn media_query_counter() -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = s()
        .bg_color(seed_colors::Red::No4)
        .color(seed_colors::Base::Black)
        .px(px(16))
        .py(px(12))
        .radius(px(6))
        .m(px(12));

    let media_query = s()
        .media("@media only screen and (max-width: 700px)")
        .bg_color(seed_colors::Green::No4);

    div![
        s().display_flex().flex_direction_column(),
        h2!["Media Query Styled Button - shrink width to less than 700px"],
        button![
            button_style,
            media_query,
            "Clicked (",
            counter,
            ") times",
            counter.on_click(|v| *v += 1)
        ],
        md![r#"
Media queries can be used in a number of ways.  The most basic way is by using the `media()` method on a style. This will 
result in that style being nested within a media query block.

For instance :
```rust
s().media("@media only screen and (max-width: 700px)")
    .bg_color("green")
``` 
will ensure that the background colour will be green if the screen is 700px or less wide.

There are a number of other (and better) ways to define media queries that involve themes and breakpoints. This will be discussed later.
        "#],
    ]
}

fn variant_counter() -> Node<Msg> {
    let danger = use_state(|| false);
    let counter = use_state(|| 0);
    let base_button_style = s()
        .bg_color(seed_colors::Gray::No6)
        .color(seed_colors::Base::White)
        .px(px(16))
        .py(px(12))
        .radius(px(6))
        .m(px(12));

    let danger_style = base_button_style.clone().bg_color(seed_colors::Red::No7);
    let ok_style = base_button_style.clone().bg_color(seed_colors::Green::No5);

    div![
        s().display_flex().flex_direction_column(),
        h2!["Style Variants"]

        ,
        button![
            base_button_style, // group styles that need the same class
            "Click to swap variants of the other button",
            danger.on_click(|v| *v = !*v)
        ],
        button![
            if danger.get() { danger_style } else { ok_style }, // group styles that need the same class
            "Clicked (",
            counter,
            ") times",
            counter.on_click(|v| *v += 1)
        ],
        md![r#"
Due to the flexibility of writing rust in view code variants of styles can be trivially implemented.

For instance we can have a base button style:

```rust
let base_button_style = s()
    .bg_color(seed_colors::Gray::No6)
    .color(seed_colors::Base::White)
    .px(px(16))
    .py(px(12))
    .radius(px(6))
    .m(px(12));
``` 

as well as `danger` and `ok` variants: 

```rust
    let danger_style = base_button_style.clone().bg_color(seed_colors::Red::No7);
    let ok_style = base_button_style.clone().bg_color(seed_colors::Green::No5);
```

We can then conditionally use this style directly inside the button macro:
```rust
button![
    if CONDITION { danger_style } else { ok_style },
    ...
]
```
        "#],
    ]
}

fn styles_on_args_counter(user_style: seed_style::Style) -> Node<Msg> {
    let counter = use_state(|| 0);

    let button_style = s()
        .bg_color(seed_colors::Red::No6)
        .color(seed_colors::Base::White)
        .px(px(10))
        .py(px(8))
        .m(px(4));

    div![
        s().display_flex().flex_direction_column(),
        h2!["Styles passed as arguments"],
        button![
            button_style,
            user_style,
            "Clicked (",
            counter,
            ") times",
            counter.on_click(|v| *v += 1)
        ],
        md![r#"
Because a style is just an object it can be passed in as the argument to a view function. e.g:

```
user_styled_btn(
    s().bg_color(seed_colors::Indigo::No5).font_size(px(24))
),
```

For instance this function defines a button with a user provided style, simply placing the user defined style
after the default style will ensure that default styles get overwritten by any clashing user style.

This is  in contrast to CSS class selectors which annoyingly have no way to specify CSS precedence other than
what order the CSS is is defined in the CSS file.

```rust
fn user_styled_btn(user_style: seed_style::Style) -> Node<Msg> {
    ...
    ...
    let button_style = s()
        .bg_color(seed_colors::Red::No6)
        .color(seed_colors::Base::White)
        .px(px(10))
        .py(px(8))
        .m(px(4));
    ...
    ...
    button![
        button_style, user_style,
        ...
    ]
}        
``` 
        "#],
    ]
}
