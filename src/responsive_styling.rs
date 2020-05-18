use crate::app_styling::theme::*;

use crate::{Model, Msg};
use seed::{prelude::*, *};

use seed_style::*;
use seed_style::{pc, px};

pub fn view(_model: &Model) -> Node<Msg> {
    div![md![r#"
# Responsive Styling

It is trivial to setup responsive styling within Seed Style.  Support is first class and includes everything from media query support, values
aligned with breakpoint scales, conditional css evaluation and breakpoint-aware conditional rendering. 

As an example of how easy it is to render responsively, the following has a font-size of 12px at the first defined breakpoint and a font size of 24px at the second.

```rust
div![
    s().font_size(&[px(12), px(24)]),
    "I am bigger on bigger screens!"
]
```

It's as as easy as that.

## How it works

By passing an array to a css property method you tell the style to use the values in order aligned with the breakpoint scale provided by the current theme.  These breakpoints 
can be entirely user set, however Seed Style ships with a basic predefined set if you choose to use them.

```rust
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
        .set_breakpoint(ExtraSmall, (0, Some(600))) 
        .set_breakpoint(Small, (600, Some(960)))
        .set_breakpoint(Medium, (960, Some(1280)))
        .set_breakpoint(Large, (1280, Some(1920)))
        .set_breakpoint(ExtraLarge, (1920, None))
        .breakpoint_scale([600, 960, 1280, 1920]) 
}
```        

In the above example the font size will be 12px for displays up to 599 pixels wide and 24px for displays 600 and upwards.

## Media Query Support

Media queries are trivial to include in your styling blocks. Simply include the `media()` function in a `Style` declaration and 
all the styles will be scoped according to that query.

For instance: 
```
let media_query = s()
    .media("@media only screen and (max-width: 700px)")
    .bg_color(seed_colors::Green::No4);
```

Will set the background color to green up to 700px.

## Conditional styles

Sometimes you want styles to only apply at a specific breakpoint and do not want to write the media query yourself.  You can use the methods

* `only()`
* `only_and_above()`
* `only_and_below()`
* and `except()`

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
fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        ...
        Msg::WindowResized => {
            conditionally_skip_rendering::<Breakpoint, _, _, _>(|| default_bp_theme(), orders)
        }
        ...
    }
}
```

Just ensure you pass in a copy of the theme that defines the relevant breakpoints.

## Summary

We have covered several ways in which your application can be trivially responsive, from simple value scales on breakpoints, to
media queries, conditional styling and finally total conditional rendering.
"#],]
}
