use crate::app_styling::theme::*;

use crate::{Model, Msg};
use seed::{prelude::*, *};

use seed_style::*;
use seed_style::{pc, px};

pub fn view(_model: &Model) -> Node<Msg> {
    div![
        md![r#"
# Getting Started

Seed Style is a comprehensive styling, layout and theming solution for Seed apps.  

It is influenced by, and has almost all of the features of, popular libraries such as Styled Components, Styled-System, 
XStyled, Theme-UI and atomic-layout. Phew! Thats a lot!  

The idea is that as a developer you can use the library in a way that suits your workflow and use as much or as little 
of the features as you need.

Want to create web apps with responsive, themeable styling? let's dive right in!

## Setting up the app environment.

Visit [https://rustup.rs/](https://rustup.rs/) and install rust, ensure you select the `nightly` toolchain.

Then in the terminal type

```
> git clone https://github.com/rebo/seed-style-quickstart-basic
> cd seed-styling-quickstart-basic
> cargo make start
```

Visit `http://localhost:8000` and you will see the basic quickstart home page.

## Styling a Html Element

Clear out the contents of the `themed_view()` function and replace as below:

```rust
pub fn themed_view(_model: &Model) -> Node<Msg> {
    p![
        s().font_size(px(24)), 
        "Hello World from Seed!"
    ]
}
```

Restarting the app with `cargo make start` and refreshing the page will show the hello world text at font size 24px. 

## How it works

The `s()` function creates a `Style` object which is then modified by adding rules using methods such as `font_size`.
This `Style` object is responsible for updating the DOM and ensuring all styles are available to specific elements.

The argument to `font_size()` is `px(24)` which is a helper method that creates an `ExactLength` which the `font_size()` method accepts.
However we can use many different arguments to our property methods, including:

a) A typed Css value such as a `CssColor` enum.  
b) A typed measurement, `ExactLength`, for properties that expect a measure.  
c) A helper function that creates one of the above, e.g. `rgb()` , `hsl()` or `px()` etc.  
d) A theme alias, such as `Color::DarkPrimary`.  
e) An array that can set breakpoint dependent values.  
f) An array that can set breakpoint dependent values at defined scales.  
g) An integer that chooses a value from a theme scale.  
h) A plain `'static &str`.  

"#],
        div![
            s().border_width(px(1))
                .my(4)
                .b_style_solid()
                .b_color(seed_colors::Gray::No4)
                .radius(px(3))
                .display_flex()
                .align_items_stretch()
                .flex_direction_row(),
            div![s().width(px(30)).flex_none().bg_color(Color::MutedPrimary)],
            div![
                s().p(px(12)),
                md![r#"
In fact you can use any argument that implements the `UpdateStyle` trait, which all the above do. Advanced 
users can therefore extend Seed Style by implementing `UpdateStyle` to process arbitrary input."#]
            ]
        ],
        md![r#"

## Basic Themes

Seed Style includes comprehensive theming capabilities, which enables common values to be set and re-used throughout your application. Our themes
conform to the "Theme Specification" which is means we use a `Theme` object to store styles and values. Add the following style to the above
code:

```rust
pub fn themed_view(_model: &Model) -> Node<Msg> {
    p![
        s().font_size(px(24))
            .color(Color::Primary)
            .border_bottom_width(px(3))
            .border_style_solid()
            .border_color(seed_colors::Red::No4),
        "Hello World from Seed!"
    ]
}
```

This will set the text color to the primary color set in `themes.rs`. It will also set the border color to the color described by `seed_colors::Red::No4` 
this color is a preset color that can be made available if the `style_presets()` theme is used.

## Global Styles       

If you inspect the text it will also have a font `Lato` already applied. What is going on here? We certainly did not
specify it in the `Style` object.

Seed Style includes a very useful feature, global styles, which enable styles to be set globally for all elements of a specific type.
Think of this as a replacement for a global `.css` file. The advantage of doing this within Seed is that global styles can make use
of theme specific values.

Edit `global_styles.rs` and replace the word `Lato` with `Arial` and restart. The main 
app font is now Arial.


## Summary

In this short getting started tutorial we have seen how to download and run a Seed quickstart, implement basic styles on a paragraph,
use themes to set specific re-used values and seen how to use global styles to apply make site wide changes.

"#],
    ]
}
