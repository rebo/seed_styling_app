use crate::app_styling::theme::*;

use crate::{Model, Msg};
use seed::{prelude::*, *};

use seed_style::px;
use seed_style::*;

pub fn view(_model: &Model) -> Node<Msg> {
    div![
        md![r#"
## Introduction

This page will show you how to get a Seed application that is ready for Seed Style setup. It will also take you through 
changing some styles in the quickstart. 

Finally it will demonstrate some advanced features of Seed Style, for instance conditional rendering.

## Getting the build environment set-up.

Visit [https://rustup.rs/](https://rustup.rs/) and install rust, ensure you select the `nightly` toolchain.

Then in the terminal type

```
> git clone https://github.com/rebo/seed_styling_quickstart
> cd seed-styling-quickstart
> cargo make start
```

Visit `http://localhost:8000` and you will see the quick-start home page.

## Let's change some styles...

Currently the main green button is defined like this:
            "#],
        div![
            s().display_flex().flex_direction_column(),
            button![
                s().background_color(Color::MutedPrimary)
                    .border_radius(px(5))
                    .margin_y(px(20))
                    .align_self_center()
                    .padding_x(&[px(18), px(24)])
                    .padding_y(&[px(8), px(12)]),
                s().hover().background_color(Color::Primary),
                s().active().background_color(Color::DarkPrimary),
                "Click Me",
            ]
        ],
        md![r#"
```
button![
    s().background_color(Color::MutedPrimary)
        .border_radius(px(5))
        .margin_y(px(20))
        .align_self_center()
        .padding_x(&[px(18), px(24)])
        .padding_y(&[px(8), px(12)]),
    s().hover().background_color(Color::Primary),
    s().active().background_color(Color::DarkPrimary),
    "Click Me",
]
```

The first line declares a button element, the next `s().bg_color(Color::MutedPrimary)` is the start of the definition of our style.
In Seed Style `s()` will create a new style builder. You can then add properties to this style by using the property name as a method name.

Also notice that the colours being set are provided by `Color::Primary` and `Color::DarkPrimary`. These are theme aliases to a colour defined by 
the current theme. You do not have to use theme colours, however it is often a good idea for colours that will get re-used often.

Seed Style is flexible and you can use a number of arguments to the property method.  
  
a) A typed Css value such as a `CssColor` enum.  
b) A typed measurement, `ExactLength`, for properties that expect a measure.  
c) A helper function that creates one of the above, e.g. `rgb()` , `hsl()` or `px()` etc.  
d) A theme alias, such as `Color::DarkPrimary`.  
e) An array that can set breakpoint dependent values.  
f) An array that can set breakpoint dependent values at defined scales.  
g) An integer that chooses a value from a theme scale.  
h) A plain `'static &str`.  
  
As you can see Seed Style provides a huge amount of flexibility for creating styles. 

We are going to change the green button and make it red.  Therefore change `s().background_color(Dark::Primary)` to `s().background_color(rgb(255,0,0))`.
This makes use of the rgb helper function to create a red color value.

```
button![
    s().background_color(rgb(255, 0, 0))
        .border_radius(px(5))
        .margin_y(px(20))
        .align_self_center()
        .padding_x(&[px(18), px(24)])
        .padding_y(&[px(8), px(12)]),
    s().hover().background_color(Color::Primary),
    s().active().background_color(Color::DarkPrimary),
    "Click Me",
]
```
Now recompile with `cargo make start` and refresh `http://localhost:8000`:
"#],
        div![
            s().display_flex().flex_direction_column(),
            button![
                s().bg_color(rgb(255, 0, 0))
                    .radius(px(5))
                    .my(px(20))
                    .align_self_center()
                    .padding_x(&[px(18), px(24)])
                    .padding_y(&[px(8), px(12)]),
                s().hover().bg_color(Color::Primary),
                s().active().bg_color(Color::DarkPrimary),
                "Click Me",
            ]
        ],
        md![r#"

Great, the button is now red, however hovering and clicking still uses the green theme shades!  These shades 
are governed by the `:hover` and `:active` pseudo selectors.  `hover()` applied to a style tags that entire block
with a `:hover` pseudo-selector.  Therefore change these lines to the following and restart:

```
s().hover().bg_color(CssColor::Hsl(0.,80.,70.)),
s().active().bg_color(CssColor::Hex(0x550000)),
```

This snippet introduces us to the CSS type that underpins each property. In this case `CssColor` which is an 
 enum with `Hsl()`,`Hex()`,`Rgba()`,`StringValue()` and `Inherit` variants.  See `css_values.rs` for a full list. Typically CSS 
 typed values can be fairly verbose therefore we typically use helper or shortcuts where available.  We have already seen
 one of these with `rgb(255,0,0)`. 
 
 Another common way to simplify CSS definitions has been created for no-argument property variants. For instance
 the CSS `display: ` property could be many values including `flex`, `block`, `hidden` etc.  We can directly use any of these by simply 
 prepending the variant name to the property. You can see this with `align_self_center()` which generates the css `align-self: center;`

## Easy conditional rendering

Using Seed Style we can easily render some content dependent upon screen side. This is particularly relevant for mobile first development 
that may not render some UI components at smaller screen sizes.

You can see an example of this in the quickstart in the block `only_and_above( Breakpoint::Medium, |model| ...`. This rendered the passed
closure *only* when at the `Medium` breakpoint and above.  This breakpoint is defined in the current theme, i.e. from 960px to 1279px.  

Lets add our own just above this: 

```
only_and_below( Breakpoint::ExtraSmall, |model| p!["Wow! That was easy"] )
```
And thats all you need to conditionally render content at certain breakpoints.

## What we have covered

We have learned about: 

*  How to download and run Seed Style quickstart.
*  How to change a themed color value to a specific color.
*  Seen how CSS pseudo selectors work, to control things like `:hover` state.
*  Understood how easy it is to use conditional rendering using Seed Style

"#],
    ]
}
