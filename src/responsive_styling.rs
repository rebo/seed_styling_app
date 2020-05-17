use crate::app_styling::theme::*;

use crate::{Model, Msg};
use seed::{prelude::*, *};

use seed_style::*;
use seed_style::{pc, px};

pub fn view(_model: &Model) -> Node<Msg> {
    div![md![r#"
# Responsive Styling

It is trivial to setup responsive styling within Seed Style.  Support is first class and includes everything from media query support, through values
aligned with breakpoint scales, conditional css evaluation and breakpoint conditional rendering. 

As an example of how easy it is to render responsively, the following has a font-size of 12px at the first defined breakpoint and a font size of 16px at the second.

```rust
div![
    s().font_size(&[px(12), px(24)]),
    "I am bigger on bigger screens!"
]
```

That's it is as easy as that.

## How it works

by passing an array to a css property method you tell the style to use the values in order aligned with the breakpoint scale provided by the current theme.  These breakpoints 
can be entirely user set, however Seed Style ships with a basic predefined set if you choose to use them.

```rust
// default seed provided breakpoints
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum SeedBreakpoint {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}
impl BreakpointTheme for SeedBreakpoint {} 

pub fn default_bp_theme() -> Theme {
    use SeedBreakpoint::*;
    Theme::new("default_bp_theme")
        .set_breakpoint(ExtraSmall, (0, Some(600))) // Breakpoints are upper bound exclusive lower bound inclusive.
        .set_breakpoint(Small, (600, Some(960)))
        .set_breakpoint(Medium, (960, Some(1280)))
        .set_breakpoint(Large, (1280, Some(1920)))
        .set_breakpoint(ExtraLarge, (1920, None))
        .breakpoint_scale([600, 960, 1280, 1920]) 
}
```        

Therefore in the above example the font size will be 12px for displays up to 599 pixels wide and 16px for displays 600 and upwards.

## Media Query Support

Media queries are trivial to include in your styling blocks. Simply include the `media()` function in a `Style` declaration and 
all the styles will be scoped according to that query.

For instance: 
```
let media_query = s()
.media("@media only screen and (max-width: 700px)")
.bg_color(seed_colors::Green::No4);
```

## Conditional styles

Sometimes you want styles to only apply at a specific breakpoint and do not want to write the media query yourself.  You can use the methods

* `only()`
* `only_and_above()`
* `only_and_below()`
* `except()`

to render styles at certain breakpoints accordingly.  For instance:

``` 
div![
    s().only_and_below(Medium).bg_color(rgb(255,0,0)),
    "This is red only at Medium and lower breakpoints"
]
```

## Conditional rendering

Sometimes you want to render seed views only at specific breakpoints and do not want to render them at all of not at those breakpoints.  You can use the functions:

* `only()`
* `only_and_above()`
* `only_and_below()`
* `except()`

to render seed views at certain breakpoints accordingly.  For instance:

``` 
only_and_above(Medium)(||
    div![
        "This div will only be rendered at Medium and higher breakpoints"
    ]
)
```

In order to support conditional rendering you need some extra plumbing to track window resizing.  Add this to the app's init method:

```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    ...
    orders.stream(streams::window_event(Ev::Resize, |_| Msg::WindowResized));
    ...
}
```
This will fire a WindowResized message whenever it, well, resizes.

For performance reasons you also want to block re-rendering on every resize, and only re-render if a breakpoint has been crossed. 
Therefore add the following to the app's update method:

```
// firmly decided.
fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        ...
        Msg::WindowResized => {
            conditionally_skip_rendering::<Breakpoint, _, _, _>(|| default_bp_theme(), orders)
        }
        ...
    }
}

Just ensure you pass in a copy of the theme that defines the relevant breakpoints.

## Summary

We have covered several ways in which your application can be trivially responsive, from simple value scales on breakpoints, to
media queries, conditional styling and finally total conditional rendering.
```







breakpoint s
css values,  Seed Style supports the [Theme Specification](https://theme-ui.com/theme-spec/) which essentially a common layout for a Theme Object which
is responsible for maintaining both a list of value aliases, as well as a set of value scales which can be used within an application.

Typically you would use a theme to control access to common values that might be applied to a variety of properties. For instance a 
`CssColor can be applied to a `CssBackgroundColor` and a `CssBorderColor` amongst other values.

Themed values in Seed Style are fully scoped and therefore can be nested inside other components that use other themes.

Themes are an important tool in maintaining design consistency and a logical design system that means adjusting your application' style
 in a consistent manner is as painless as possible.

## Defining Theme Aliases

An important aspect of designing a theme is deciding what value aliases might be used within an application. You can create these aliases
as enums, for instance the below lists a set of color aliases that might be used.  

```rust
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Color {
    Background,
    MainText,
    Primary,
    MutedPrimary,
    DarkPrimary,
    MutedSecondary,
    Secondary,
    DarkSecondary,
    Highlight,
}
impl ColorTheme for Color {} // Allows you to use a `Color` variant as a CssColor alias in the theme.
```

Notice that specific theme values have not been set for these aliases, we do this by creating a `Theme` instance.

## Creating a `Theme` instance

Once you have decided what aliases you will allow you can assign css values to them when instantiating a theme:

```rust
let theme = Theme::new("dark_theme")
.set_color(Color::Background, CssColor::Hex(0x333333))
.set_color(Color::MainText, CssColor::Hex(0xDDDDDD))
.set_color(Color::DarkPrimary, hsl(300, 70, 30))
.set_color(Color::MutedPrimary, CssColor::Hsl(300.0, 70.0, 90.0))
.set_color(Color::Secondary, hsl(200, 60, 40)) // or use the hsl shortcut
.set_color(Color::MutedSecondary, hsl(200, 15, 30)) // or use the hsl shortcut
.set_breakpoint(Breakpoint::ExtraSmall, (0, Some(600))) // Breakpoints are upper bound exclusive lower bound inclusive.
.set_breakpoint(Breakpoint::Small, (600, Some(960)))
.set_breakpoint(Breakpoint::Medium, (960, Some(1280)))
.set_breakpoint(Breakpoint::Large, (1280, Some(1920)))
.set_breakpoint(Breakpoint::ExtraLarge, (1920, None))
.set_shadow(Shadow::RightEdge, "8px 0px 6px -8px #222222");
```

The `Theme` constructor takes the name of the theme, this is important because it can be used for live swapping themes if your application 
wants to support this feature.  The above code sets some color theme values, breakpoints, and a shadow theme.

These aliases can then be used directly within a style property method. I.e.

```
div![
    s().color(Color::MainText).bg_color(Color::Background).box_shadow(Shadow::RightEdge)
]
```

### Theme scales

Along with theme aliases theme scales are another important aspect to a theme. They allow css values to be set as a scale, or array of values.
This is handy if you do not want to explicitly name theme values, and also to create a consistent scale of values.

For instance,

```
theme.space_scale(&[px(2), px(4), px(8), px(12), px(16), px(24), px(32)])
```

sets up a Space scale these are CSS values that can be applied to properties which relate to spacing such `CssMargin`, `CssPadding` etc.
This is useful in ensuring you refer to a restricted number of different spacings and so that these spacings are consistently adjustable from
a central location ( The theme object !).  The alternative for this is every object having its spacing properties set to say a pixel measure independently,
this means you could have hundreds of different spacing scales that do not work together to deliver a consistent design.

Once a scale is set up you can simply refer to a measure in that scale by using the index of the scale.  For instance,

```rust
div![
    s().margin(2),
    "Hello"
]
```

Refers to `8px` in the above scale.  Note scales are currently 0-based indexed, this may change in response to feedback.

## Default values

Sometimes you may want to use a theme value if it is available or otherwise use a default. You can do this by passing a 2-tuple to
a style property. For instance:

```
div![
    s().bg_color( (Color::Primary, rgb(255,0,0) )),
    "Hello"
]
```
Uses the `Primary` color theme, but if its not been set then it will default to red.

## Accessing and scoping themes

You need to ensure your thee is in-scope in order to access the CSS values that it stores. This is to enable nesting of themes and allow 
say a sub-component to have its own defined themes.

Access is governed by the `use_themes()` function and it is often the first expression in the main app view function.

```rust
use_themes(
    || vec![style_presets(), my_theme()],
    || {
        themed_global_styles();
        ... themeable content here....
    },
)
```

You pass it an array of themes to apply as the first argument, and the content to render as the second argument. Please note that these are 
passed as closures so that themes do not need to be instantiated on every render.

Anything in the second argument closure will have access to the provided theme.

Often it is  to also register global styles just inside this second block, this ensures that your global styles are registered and updated 
with the relevant theme values.
"#],]
}
