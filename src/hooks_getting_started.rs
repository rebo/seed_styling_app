use crate::app_styling::theme::*;

use crate::{Model, Msg};
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
use seed_style::*;
use seed_style::{pc, px};
use seed_hooks::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightElement(el: web_sys::HtmlElement);
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAllUnder(el: web_sys::HtmlElement);
}


#[topo::nested]
fn section_desc<T: Into<String>>(href_name: T, title: T, description: T) -> Vec<Node<Msg>> {

    

    let title = md!(&title.into());
    let description = md!(&description.into());

    let desc_el = use_state(ElRef::<web_sys::HtmlElement>::default);


    do_once(|| 
        after_render(move |_| {
            if let Some(desc_el) = desc_el.get().get() {
                let code_children = desc_el.get_elements_by_tag_name("h3");

                for idx in 0..code_children.length() {
                    let code_el = code_children.item(idx).unwrap();
                    code_el.set_class_name("text-xl py-3 pt-4");
                }

                let code_children = desc_el.get_elements_by_tag_name("code");

                for idx in 0..code_children.length() {
                    let code_el = code_children.item(idx).unwrap();
                    code_el.set_class_name("language-rust");
                    highlightElement(code_el.dyn_into::<web_sys::HtmlElement>().unwrap());
                } 
            }   
        })
    ).reset_on_unmount();

    nodes![
        h2![
            s().m(px(3)).font_size(px(24)),
            a![attrs![At::Name=>href_name.into()], title]
        ],
        div![
            el_ref(&desc_el.get()),
            s().margin_top(px(3)),
            description
        ],
    ]
}




pub fn view(_model: &Model) -> Node<Msg> {
    div![
        md![r#"
# Getting Started

Seed Hooks is a comprehensive component state solution for Seed apps. With Seed Hooks you can create customisatble, re-usable and interactive components.

It is primarily influenced by React Hooks but also has some features from View and Svelte, for instance easy form input binding.

Want to create interactive web apps with re-usable components? let's dive right in!

## Setting up the app environment.

Visit [https://rustup.rs/](https://rustup.rs/) and install rust, ensure you select the `nightly` toolchain.

Then in the terminal type

```
> git clone https://github.com/rebo/seed-hooks-quickstart.git
> cd seed-hooks-quickstart
```

To run this quickstart project you need cargo-make installed:

```
cargo install cargo-make
```
This will install cargo-make in your `~/.cargo/bin`.
Make sure to add `~/.cargo/bin` directory to your PATH variable.

You will also need to ensure that Rust can target wasm by adding this component:

```
rustup target add wasm32-unknown-unknown
```

Check it compiles and serves correctly with:

```
cargo make build; cargo make serve
```

You can access the site from `http://localhost:8000`. This will display a simple button counter.

Currently **Seed Hooks** only work on nightly rust, this is due to requiring the feature `TrackCaller` therefore it is 
important to install a recently nightly. 

To install nightly rust do this:

```
rustup install nightly
```

To ensure that nightly is used for only this project add a `rust-toolchain` file to the project root, the contents of this file should be the single line `nightly`. 

## Seed Hooks Setup

Some of these settings will already be set if using the Seed Hooks quickstart.

In order to enable **Seed Hooks** add the following to `Cargo.toml` in the `[dependencies]` section. 

```
// In Cargo.toml...

seed_hooks = "0.1.3"
```

Next, Seed hooks rely on the nightly `TrackCaller` feature you need to add the `#![feature(track_caller)]` feature flag to the top of `lib.rs`.

You should also glob import the `seed_hooks` crate with:

```
// In in lib.rs...

use seed_hooks::*;
```

The final bit of setup required is to add a root component to the Seed view. This is achieved by annotating 
the main Seed view function with `#[topo::nested]`. For now replace the contents of the root view with a simple `div![]`.

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![]
}
```

This annotation means that the view function becomes part of the component hierarchy. Indeed this acts as the root component 
under which all other components are structured.

Remove all existing `Model` and `Msg` fields/variants. You will also want to remove the match processing of `Msg` in your update function.

The final base `lib.rs` should be :

```
#![feature(track_caller)]
use seed_hooks::*;
use seed::{prelude::*, *};

#[derive(Default)]
struct Model {}

enum Msg {}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![]
}


// init sets up simple routing, global CSS styles for css resets,
// and window resizing callback
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}

// Default app start...
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

```
## Creating a simple button counter

Modify the `view` function in `lib.rs` to now read this:

```rust
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    let count = use_state(||0);
    div![
        count
    ]
}
```

Re-compiling the quickstart with `cargo make serve` in the console, and then navigating to `http://localhost:8000` will show
a single zero and nothing else.

So what's going on? the initial render line `let count = use_state(||0);` creates an integer **state variable** 
this state is registered and stored in a thread local data structure.  The `use_state` function returns a `StateAccess<u32>`
which can be used for (amongst other things), displaying the state, updating the state and managing the states lifetime.

If the generic type to a `StateAccess<T>` implements `Display` then seed can automatically render this value. Hence inside the `div` 
we only need to refer to `count`.

On subsequent renders the `use_state` function just returns the `StateAccess` and does not try to re-initialise it to zero.

There is no point in storing state if we are not going to mutate it at some point, therefore update the `div` to now include an
increment button:

```rust
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    let count = use_state(||0);
    div![
        count,
        button!["Increment Counter", count.on_click(|c| *c += 1)],
    ]
}
```
The only Seed Hooks specific code here is `count.on_click(|c| *c += 1)`. This is a shortcut for setting up an `EventHandler` to mutate the 
count state variable.  The closure will be run when the button is clicked and the argument to the closure is the state variable itself.

Re-compiling the quickstart with `cargo make serve` in the console, and then navigating to `http://localhost:8000` will show
a zero and a button which when clicked increments the counter which clicked.


Styled example:

"#]
,

{    let count = use_state(||0);
    
        Row![
            s().margin_y(px(30)),
            padding = px(8),
            gap = px(12),
            Item![
                align = RowAlign::CenterMiddle,
                s().bg_color(seed_colors::Green::No6).color(seed_colors::Base::White),
                count,
            ],
            Item![
                align = RowAlign::Center,
                button![
                    s().bg_color(seed_colors::Green::No6)
                        .color(seed_colors::Base::White)
                        .font_size(px(24))
                        .px(px(24))
                        .py(px(12))
                        .radius(px(8)),
                    "Increment Counter",
                    count.on_click(|c| *c += 1)
                ],
            ]
        ]
    
}
]
}