use crate::compositions::*;
use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::{px,pc,rgba, ExactLength};
use seed_style::*;
use crate::app_styling::theme::*;
use web_sys::{HtmlElement, HtmlTextAreaElement};

#[view_macro]
fn center_view<Ms>(mut root: Node<Ms>, children: Vec<Node<Ms>>) -> Node<Ms> {
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


#[derive(Default)]
pub struct AdditionalStyle {
    pub style: Option<seed_style::Style>,
}


fn scroll_event_handler(
    event: Ev,
    textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
    preview_el: StateAccess<ElRef<HtmlElement>>
    ) -> EventHandler<Msg> {
    textarea_el.input_ev(event, move |el, _| {
        if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
            let textarea_scroll_percentage = {
                let textarea_max_scroll_top = textarea.scroll_height() - textarea.client_height();
                if textarea_max_scroll_top == 0 {
                    0.
                } else {
                    f64::from(textarea.scroll_top()) / f64::from(textarea_max_scroll_top)
                }
            };
            let new_preview_scroll_top = {
                let preview_max_scroll_top = preview.scroll_height() - preview.client_height();
                f64::from(preview_max_scroll_top) * textarea_scroll_percentage
            };
            preview.set_scroll_top(new_preview_scroll_top as i32);
        }
    })
    }

// In in lib.rs...
#[topo::nested]
fn markdown_editor(on_submit: impl FnOnce(String) -> Msg + 'static + Clone) -> Node<Msg>{
    let source = use_state(|| String::new());


    let preview_el = use_state(ElRef::<HtmlElement>::default);
    let textarea_el = use_state(ElRef::<HtmlTextAreaElement>::default);

    Column![
        s().height(px(400)),
        Row![  // s().flex("1 1 0%").min_height(px(0)),
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Markdown:"],
                textarea![ 
                    el_ref(&textarea_el.get()),
                    s().font_family("'Lucida Console', Monaco, monospace").p(px(4)).b_color(seed_colors::Gray::No6).b_width(px(1)).b_style_solid().w(pc(100)).h(pc(100)),
                    attrs!{At::Type => "textbox"},  
                    bind(At::Value, source),
                    scroll_event_handler(Ev::KeyUp ,textarea_el, preview_el),
                    scroll_event_handler(Ev::Scroll, textarea_el, preview_el),
                ],
            ],
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Preview:"],
                div![ 
                    class!["markdown-body"],  
                    md![&source.get()],
                    el_ref(&preview_el.get()),
                    s().overflow_auto().p(px(4)).b_color(seed_colors::Gray::No6).bg_color(seed_colors::Indigo::No1).b_width(px(1)).b_style_solid().h(pc(100)),
                ]
            ]
        ],    
        Row![flex = Flex::None,
            Item![align = RowAlign::Right, 
                button![
                "Submit (See console log)",
                mouse_ev(Ev::Click, move |_| {
                    let markdown_element = preview_el.get().get().expect("markdown-body doesn't exist");
                    on_submit(markdown_element.inner_html())
                })
                ]
            ]
        ]
    ]
} 


#[derive(Default)]
pub struct AdditionalStyles {
    pub styles: Vec<seed_style::Style>
}

#[view_macro]
fn fancy_button_view<Ms>(args: AdditionalStyle, mut root: Node<Ms>, children: Vec<Node<Ms>> )-> Node<Ms>{
    as_tag![
        button,
        root,
        s().padding_x(&[px(18), px(24)])
        .padding_y(&[px(8), px(12)])
        .margin(px(8))
        .my(px(20))
        .font_size(&[px(16), px(24)]).background_color(seed_colors::Indigo::No6)
            .color(seed_colors::Base::White)
            .border_radius(px(3))
            .outline_none(),
        s().hover()
            .background_color(seed_colors::Blue::No5)
            .cursor_pointer(),
        if let Some(style) = args.style{
            style
        } else {
            s()
        },
        children
    ]
}


#[view_macro]
fn base_button_view<Ms>(args: AdditionalStyles, mut root: Node<Ms>, children: Vec<Node<Ms>> )-> Node<Ms> {
    as_tag![
        button,
        root,
        s().border_radius(px(3))
            .outline_none(),
        s().hover()
            .cursor_pointer(),
        children
    ]
}
#[view_macro]
fn small_button_view<Ms: 'static>(args: AdditionalStyles,  root: Node<Ms>, children: Vec<Node<Ms>>) -> Node<Ms> {
    base_button![
        extend = root,
        s().padding_x(&[px(12), px(18)])
        .padding_y(&[px(4), px(8)])
        .margin(px(4))
        .my(px(12))
        .font_size(&[px(12), px(14)]),
        args.styles,
        children
    ]
}
#[view_macro]
fn big_button_view<Ms : 'static>(args: AdditionalStyles,  root: Node<Ms>, children: Vec<Node<Ms>>) -> Node<Ms> {
    base_button![
        extend = root,
        s().padding_x(&[px(18), px(24)])
            .padding_y(&[px(8), px(12)])
            .margin(px(8))
            .my(px(20))
            .font_size(&[px(16), px(24)]),
            args.styles,
        children
        
    ]
}

        



fn modal(modal_content: StateAccess<Option<fn() -> Node<Msg>>>) -> Node<Msg> {
    if let Some(content) = modal_content.get(){
        div![
            div![
                div![
                    s()
                        .position_absolute()
                        .width(pc(100))
                        .height(pc(100))
                        .bg_color(rgba(30,30.,30.,0.5))
                ]
            ],
                s().position_fixed().z_index("50").overflow_auto().display_flex()
                .top(px(0))
                .right(px(0))
                .bottom(px(0))
                .left(px(0))
                ,
                // C.fixed, C.inset_0, C.z_50, C.overflow_auto, C.flex,
            div![
                s().position_relative().px(px(40)).py(px(24))
                .bg_color(Color::Background)
                .w(pc(80)).max_w(px(700))
                .m_auto()
                .display_flex()
                .flex_direction_column()
                .radius(px(12))
                .box_shadow(r#"
                0px 2.2px 0.1px -6px   rgba(0, 0 ,0 ,0.350),
                0px 5.3px 6px -6px     rgba(0 ,0 ,0 ,0.228),
                0px 10px 14.2px -6px   rgba(0 ,0 ,0 ,0.202),
                0px 17.9px 24.8px -6px rgba(0 ,0 ,0 ,0.190),
                0px 33.4px 39.4px -6px rgba(0 ,0 ,0 ,0.177),
                0px 80.4px 69px -6px   rgba(0, 0 ,0 ,0.146)
                "#)
                ,
                    // C.relative, C.p_8, C.bg_white, C.w_full, C.max_w_5xl, C.m_auto, C.flex_col, C.flex, C.rounded_sm, C.shadow_2xl
                div![
                    s().flex_direction_column(),
                    div![h2![s().font_weight_v900(), s().font_size(px(24)), "Code Example"]],
                    div![s().p(px(4)), content()],
                    Row![
                        Item![ align = RowAlign::Right,
                            button![
                                attrs! {At::Type => "button"},
                                
                                    s().mx(px(4))
                                    .bg_color(seed_colors::Indigo::No6)
                                    .color(seed_colors::Base::White)
                                    .font_weight_v900()
                                    .py(px(4))
                                    .px(px(8))
                                    .bb_width(px(4))
                                    .b_color(seed_colors::Indigo::No5)
                                    .radius(px(6))  ,
                                    s().hover().bg_color(seed_colors::Indigo::No5)

                                  
                                ,
                                modal_content.mouse_ev(Ev::Click, |mc,_| *mc = None),
                                "Close"
                            ],
                    ]]]
                ]]
            



    } else {
        empty![]
    }
    
   
}

#[derive(Clone)]
pub struct SectionArgs{
    sig: String,
    title: String,
    modal_content: StateAccess<Option<fn()->Node<Msg>>>,
    code_example: Option<fn()->Node<Msg>>,
}

impl Default for SectionArgs {
    fn default() -> Self {
        SectionArgs{
        title: "".to_string(),
        sig: "".to_string(),
        modal_content: use_state(||None),
        code_example: None,
        }
        
    }
}

#[view_macro]
pub fn sec_view<Ms: 'static>(args: SectionArgs, _root: Node<Ms>, _children: Vec<Node<Ms>>, description: Node<Ms>, code_example: Node<Ms>)  -> Node<Ms>{
    
    let mut c = code_example;
    let (title,sig,modal_content,code_example) = (args.title, args.sig, args.modal_content, args.code_example);
    div![
        h3![
            title
        ],
        pre![code![sig]],
        description.clone(),

        pre![
            as_tag![
                code,
                c
            ]
        ],
        center![
        fancy_button![
            style = Some(s().radius(px(8))),
            "Show Example",
            modal_content.on_click(move |c| *c = code_example)
        ]
        ]

    ]
}


#[topo::nested]
fn use_state_example() -> Node<Msg> {
    let count = use_state(|| 0);
    div!["Count:",
        count.get().to_string(),
        button!["Increase Count",  s()
            .mx(px(8))
            .bg_color(seed_colors::Green::No6)
            .color(seed_colors::Base::White)
            .font_weight_v900()
            .py(px(4))
            .px(px(8))
            .font_size(px(12))
            .bb_width(px(2))
            .b_color(seed_colors::Gray::No4)
            .radius(px(6)),
            s().hover().bg_color(seed_colors::Green::No5)
    , count.mouse_ev(Ev::Click, |count, _| *count += 1)
    ]
    ]
}



#[topo::nested]
fn if_example() -> Node<Msg> {
    use std::cmp::Ordering;
    let input_a = use_state(String::new);
    let input_b = use_state(String::new);

    if input_a.changed() || input_b.changed() {
        after_render(move |_| {
            if let (Ok(a), Ok(b)) = (input_a.get().parse::<i32>(), input_b.get().parse::<i32>()) {
                let smallest = match a.cmp(&b) {
                    Ordering::Less => "<li>A is the smallest</li>",
                    Ordering::Greater => "<li>B is the smallest</li>",
                    Ordering::Equal => "<li>Neither is the smallest</li>",
                };

                if let Some(elem) = get_html_element_by_id("list") {
                    let _ = elem.insert_adjacent_html("beforeend", smallest);
                }
            }
        });
    }

    div![
        "A:",
        input![bind(At::Value, input_a),   s().border_color(seed_colors::Gray::No6).radius(px(4)).border_width(px(1)).py(px(4)),],
        "B:",
        input![bind(At::Value, input_b),  s().border_color(seed_colors::Gray::No6).radius(px(4)).border_width(px(1)).py(px(4)),],
        ul![id!("list"), "Smallest Log:"],
    ]
}

fn md_view() -> Node<Msg> {
    markdown_editor(Msg::SubmitMarkdownHtml)
}



pub fn view(_model : &Model) -> Node<Msg>{
    
    render_centred_article(_model, |_| {
        let modal_content = use_state(||None);
        
    div![
        modal(modal_content),
    
    h1!["Hooks Tutorial"],
md![r#"

## Design

In this tutorial we will be creating a reusable markdown renderer component. The only brief is that the user should be able to pass a message
to the component which sends the rendered html to Seed on submission.

What we are going to do in this section is:

1. Design the rough layout and styling for the component
1. Decide what state needs to be stored locally
1. Ensure state can be outputted into a preview div

Visually we want a split view, the left effectively being one big textbox, the right being a preview render of the markdown.
We also want a submit button below both these panes. Add the below code to `lib.rs` to setup the above layout.

We will be making use of Seed Style features to assign with the styling and layout.
"#],

"Here is an example of what we will create: ", fancy_button!["Example",modal_content.on_click(move |c| *c = Some(md_view))]



,md![r#"
```
// In in lib.rs...
#[topo::nested]
fn markdown_editor() -> Node<Msg> {
    Column![
        s().height(px(400)),
        Row![ 
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Markdown:"],
                textarea![ 
                    s().w(pc(100)).h(pc(100)),
                    attrs!{At::Type => "textbox"},  
                ],
            ],
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Preview:"],
                div![ 
                    class!["markdown-body"],  
                    s().overflow_auto().h(pc(100)),
                ]
            ]
        ],    
        Row![flex = Flex::None,
            Item![align = RowAlign::Right, button!["Submit (See console log)"]]
        ]
    ]
} 
```
The reason we annotate with `#[topo::nested]` is so that `markdown_editor` can operate as its own component with local state. 

Style wise, it lays out a column of two rows, the first row contains two columns for the markdown input and the preview pane.  The second row aligns a submit button to the right.
Note that `Row!` and `Column!` are both flex grow with zero basis. Which means they fill their container as much as possible.
This is why we set `Flex::None` on the title and button rows because we don't want them to grow.

This component can be rendered by calling it in the root view:

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    markdown_editor()
}
```

Currently nothing in the page is functional, the above code only sets up the layout.

### Cargo Watch

At this stage it would be worth setting up a cargo watch loop to rebuild the wasm file and re-serve so that you can see your changes
more immediately in the browser.

Run the following in separate terminal windows

```
cargo make serve
```
and 
```
cargo make watch
```

`cargo make serve` will ensure that your server is always running and `cargo make watch` will automatically re-compile the `.wasm` file. 
Therefore the only thing that you will need to do is refresh the browser after updating any of your rust files.

### Data Flow

So what do we want our component to do? When the user types in the textarea we want the contents to be processed
by a markdown processor and the results viewable in the preview box on the right.

We would also like the preview box to be in sync with the textarea cursor.

Datawise, this means we need a **state variable** to store the current textarea content, this then gets processed by a filter 
on an Input` event.  

In order to create this **state variable** we will use the first (and most used) hook function `use_state()`. Add the following
at the top of the function.

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    let source = use_state(|| String::new());
    ...
```
This creates a **state variable** accessor `source`. This accessor is used to get and set a `String` value associated with this component. 

Next we need to bind this source to the `value` attribute of the textarea. Modify the `textarea!` to include this a bind directive.

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    ...
    textarea![
        bind(At::Value, source),
        ...
]
```


Lastly lets ensure that the bind is working correctly we simple pipe the raw `textarea![]` input into the preview div:

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    ...

    Column![ 
        Item![flex = Flex::None, align = ColumnAlign::Top, "Preview:"],
        div![ 
            class!["markdown-body"],  
            source.get(),
            ...
        ]
    ..
    ]
```
Refreshing your browser now (`https://localhost:8000`) and typing in the textarea should output the text directly within the 
markdown preview div.

## Step 2 - Markdown processing


What we are going to do in this section is 

1. Process the source state variable as markdown prior to rendering in the view.

### How to process

We now have a basic bind set up, updating the textarea will update the `source` state variable.
This is then directly output to the preview div.

Instead of outputting directly to the preview div, we want it to be processed as markdown. 
Fortunately Seed has an in-built macro that renders markdown from a `&str`.

Simply wrap `source.get()` in `md!(&source.get())` in the markdown preview div:

```
// In in lib.rs...
...
Column![ 
    Item![flex = Flex::None, align = ColumnAlign::Top, "Preview:"],
    div![ 
        class!["markdown-body"],  
        md![&source.get()],
        ..
    ]
...

```

Here is the `markdown_editor` function at this stage. 

```
/ In in lib.rs...
#[topo::nested]
fn markdown_editor() -> Node<Msg> {
    let source = use_state(|| String::new());

    Column![
        s().height(px(400)),
        Row![ 
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Markdown:"],
                textarea![ 
                    s().w(pc(100)).h(pc(100)),
                    attrs!{At::Type => "textbox"},  
                    bind(At::Value, source),
                ],
            ],
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Preview:"],
                div![ 
                    class!["markdown-body"],  
                    md![&source.get()],
                    s().overflow_auto().h(pc(100)),
                ]
            ]
        ],    
        Row![flex = Flex::None,
            Item![align = RowAlign::Right, button!["Submit (See console log)"]]
        ]
    ]
} 
```

## Step 3 - Prettifying the output



What we are going to do in this section is 

1. Use a Github styled markdown CSS
1. Fix content overflows in the preview div
1. Improve the visual look of the textarea

### Github styled markdown CSS

The UI currently is functional but it can be improved, specifically regarding the preview render.

We therefore need to style both to better improve the UI.

We will use `github-markdown-css` for this, we can simply use the CDN version of this file:

```
// in index.html
... 
<head>
...
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/4.0.0/github-markdown.min.css">
.... 
```

Because we have already used the class `markdown-body` for our markdown preview div then the CSS should just work.

1. Fix content overflows in the preview div

Currently the styled processed markdown will overflow the preview div. Therefore we need to ensure that `overflow` is set to auto
for this div. Furthermore we can adjust the styling on the div for an improved look. Modify the existing `class!`s  as follows: 

```
/// in lib.rs
...
div![ 
    class!["markdown-body"],  
    md![&source.get()],
    el_ref(&preview_el.get()),
    s().overflow_auto().p(px(4)).b_color(seed_colors::Gray::No6).bg_color(seed_colors::Indigo::No1).b_width(px(1)).b_style_solid().h(pc(100)),
]
...
```

This will ensure the preview pane's markdown is rendered correctly.

### Improve the look of the textarea

Furthermore we would like the textarea input to be mono-space. Therefore adjust its class:

```
// In in lib.rs...

textarea![
...
textarea![
    s().font_family("'Lucida Console', Monaco, monospace").p(px(4)).b_color(seed_colors::Gray::No6).b_width(px(1)).b_style_solid().w(pc(100)).h(pc(100)),
    attrs!{At::Type => "textbox"},  
    bind(At::Value, source),
],
...
],
```            

Lets try how it all works now, save the file refresh the browser. Try typing the following into the textarea: 

```
# Seed Rocks

**Yes** indeed it does *rock*.

```
Here is what the markdown editor function should look like now:

```

// In in lib.rs...
#[topo::nested]
fn markdown_editor() -> Node<Msg> {
    let source = use_state(|| String::new());

    Column![
        s().height(px(400)),
        Row![ 
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Markdown:"],
                textarea![ 
                    s().font_family("'Lucida Console', Monaco, monospace").p(px(4)).b_color(seed_colors::Gray::No6).b_width(px(1)).b_style_solid().w(pc(100)).h(pc(100)),
                    attrs!{At::Type => "textbox"},  
                    bind(At::Value, source),
                ],
            ],
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Preview:"],
                div![ 
                    class!["markdown-body"],  
                    md![&source.get()],
                    s().overflow_auto().p(px(4)).b_color(seed_colors::Gray::No6).bg_color(seed_colors::Indigo::No1).b_width(px(1)).b_style_solid().h(pc(100)),
                ]
            ]
        ],    
        Row![flex = Flex::None,
            Item![align = RowAlign::Right, button!["Submit (See console log)"]]
        ]
    ]
} 

```


## Step 4 - Adding auto scrolling

### Auto scrolling the preview

When we edit the textarea we ideally would like the preview to scroll to a similar position. 
This would enable our edits to be easier to see. Therefore we want to programmatically scroll the `markdown-body` div 
on `KeyUp` and also on `Scroll` events.

In order to do this we need to identify the preview div and also the textarea with `ElRef`s. 
These are Seed's way of identifying individual elements. 

Due to the fact that we are going to refer to specific html elements via `web_sys` we need to add that as a dependency.

In `Cargo.toml` add the following to the dependencies section: 

```
[dependencies]
...
web-sys = "0.3.39"
...
```

and enable access to the following types at the top of `lib.rs`:

```
use web_sys::{HtmlElement, HtmlTextAreaElement};
```

after the `let source = use_state..` line add two more `use_state` hooks. 

```
// In in lib.rs...

let preview_el = use_state(ElRef::<HtmlElement>::default);
let textarea_el = use_state(ElRef::<HtmlTextAreaElement>::default);
```

This provides access to two el_refs which we can later associate with specific elements. 

In order to do this we use the `el_ref()` function within the respective elements...

```
// In in lib.rs...

textarea![
el_ref(&textarea_el.get()),
...
```

and 

```
// In in lib.rs...

div![
el_ref(&preview_el.get()),
class!["markdown-body"],
...
```

In order to set the respective scroll on the preview we use a simple percentage of textarea
scroll as a guide.

This is achieved via the following event handler, add this to the bottom of the `textarea![]`
node:

```
// In in lib.rs...
textarea![
...

...
textarea_el.input_ev(Ev::KeyUp, move |el, _| {
    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
        let scroll_percentage = (textarea.scroll_top() as f64) / (textarea.scroll_height() as f64);
        let new_scroll_top = (preview.scroll_height() as f64) * scroll_percentage;
        preview.set_scroll_top(new_scroll_top as i32);
    }
}),
]
```

Testing this will demonstrate basic percentage based scrolling.

There are some issues with this simple percentage based scroll above. For markup that 
results in larger rendered markdown, such as headers, the scrolling will not perfectly match up.

Furthermore we also need an identical `EventHandler` callback for an `Ev::Scroll` event.  You could cut and paste code above
however we can prevent needless repetition by using a function.  

We fix both these issues by using the code below.

First, remove the `EventHandler` above and add the following function to `lib.rs`.

```
//in lib.rs 

fn scroll_event_handler(
event: Ev,
textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
preview_el: StateAccess<ElRef<HtmlElement>>
) -> EventHandler<Msg> {
textarea_el.input_ev(event, move |el, _| {
    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
        let textarea_scroll_percentage = {
            let textarea_max_scroll_top = textarea.scroll_height() - textarea.client_height();
            if textarea_max_scroll_top == 0 {
                0.
            } else {
                f64::from(textarea.scroll_top()) / f64::from(textarea_max_scroll_top)
            }
        };
        let new_preview_scroll_top = {
            let preview_max_scroll_top = preview.scroll_height() - preview.client_height();
            f64::from(preview_max_scroll_top) * textarea_scroll_percentage
        };
        preview.set_scroll_top(new_preview_scroll_top as i32);
    }
})
}
```

Finally, the following two lines at the bottom of the `textarea![]` will generate the correct event handlers:

```
//in lib.rs  

textarea![
...

... 
scroll_event_handler(Ev::KeyUp ,textarea_el, preview_el),
scroll_event_handler(Ev::Scroll, textarea_el, preview_el),

]
```

Once all the above is completed, scrolling and cursor navigating through the textarea will 
result in a corresponding scroll of the preview div.


Try pasting the following into the textarea and try scrolling or moving the cursor around:

```
* this
* is
* a 
* very
* long
* list
* that
* goes
* on
* and
1. on
1. and
1. on
1. it
* Should
* demonstrate
* Auto scrolling the preview 
* in response to 
* scrolling of the 
* text area
* and cursor movement


```

## Step 5 - Adding Message submssion

We now modify the function signature to allow an arbitrary message to be passed to Seed's update function.
The message will be sent to Seed when the submit button is pressed.

The message should permit a String argument which should be the content of the rendered markdown. 
Hence we will use the following `markdown_editor()` function signature: 

```
// In in lib.rs...

fn markdown_editor(on_submit: impl FnOnce(String) -> Msg + 'static + Clone) -> Node<Msg>
```

We modify the `Msg` type to add a variant that accepts a `String` argument.

```
// In in lib.rs...

enum Msg {
NoOp,
SubmitMarkdownHtml(String),
}
```

We add an `Ev::Click` event handler to the submit button, this sends the `Msg` to Seed.

```
// In in lib.rs...

button![
class!["bg-green-400 p-4 m-2"],
"Submit (See console log)",
mouse_ev(Ev::Click, move |_| {
    let markdown_element = preview_el.get().get().expect("markdown-body doesn't exist");
    on_submit(markdown_element.inner_html())
})
]
```

We handle the `Msg` in the Seed app's update function by printing it to the console. The last tweak is adjusting the calling function in the view:

```
// In in lib.rs...

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
match msg {
    Msg::SubmitMarkdownHtml(html) => log!(html),
    Msg::NoOp => {}
}
} 

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
markdown_editor(Msg::SubmitMarkdownHtml)
}
```

Now when the form button is clicked, an output of the processed html will be
logged to the console from the Seed update function.

The final `lib.rs` file is below:

```
pub struct Model {}

#[derive(Clone)]
pub enum Msg {
    SubmitMarkdownHtml(String),
}


fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}


// Default app start...
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}



#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
markdown_editor(Msg::SubmitMarkdownHtml)
}


fn scroll_event_handler(
    event: Ev,
    textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
    preview_el: StateAccess<ElRef<HtmlElement>>
    ) -> EventHandler<Msg> {
    textarea_el.input_ev(event, move |el, _| {
        if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
            let textarea_scroll_percentage = {
                let textarea_max_scroll_top = textarea.scroll_height() - textarea.client_height();
                if textarea_max_scroll_top == 0 {
                    0.
                } else {
                    f64::from(textarea.scroll_top()) / f64::from(textarea_max_scroll_top)
                }
            };
            let new_preview_scroll_top = {
                let preview_max_scroll_top = preview.scroll_height() - preview.client_height();
                f64::from(preview_max_scroll_top) * textarea_scroll_percentage
            };
            preview.set_scroll_top(new_preview_scroll_top as i32);
        }
    })
}

// In in lib.rs...
#[topo::nested]
fn markdown_editor(on_submit: impl FnOnce(String) -> Msg + 'static + Clone) -> Node<Msg>{
    let source = use_state(|| String::new());


    let preview_el = use_state(ElRef::<HtmlElement>::default);
    let textarea_el = use_state(ElRef::<HtmlTextAreaElement>::default);

    Column![
        s().height(px(400)),
        Row![  
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Markdown:"],
                textarea![ 
                    el_ref(&textarea_el.get()),
                    s().font_family("'Lucida Console', Monaco, monospace").p(px(4)).b_color(seed_colors::Gray::No6).b_width(px(1)).b_style_solid().w(pc(100)).h(pc(100)),
                    attrs!{At::Type => "textbox"},  
                    bind(At::Value, source),
                    scroll_event_handler(Ev::KeyUp ,textarea_el, preview_el),
                    scroll_event_handler(Ev::Scroll, textarea_el, preview_el),
                ],
            ],
            Column![ 
                Item![flex = Flex::None, align = ColumnAlign::Top, "Preview:"],
                div![ 
                    class!["markdown-body"],  
                    md![&source.get()],
                    el_ref(&preview_el.get()),
                    s().overflow_auto().p(px(4)).b_color(seed_colors::Gray::No6).bg_color(seed_colors::Indigo::No1).b_width(px(1)).b_style_solid().h(pc(100)),
                ]
            ]
        ],    
        Row![flex = Flex::None,
            Item![align = RowAlign::Right, 
                button![
                "Submit (See console log)",
                mouse_ev(Ev::Click, move |_| {
                    let markdown_element = preview_el.get().get().expect("markdown-body doesn't exist");
                    on_submit(markdown_element.inner_html())
                })
                ]
            ]
        ]
    ]
} 


```

## Step 6 - Reusing the markdown editor

So far we have created a functional live markdown editor with local state
which stores, processes, and renders the markdown source in a preview pane.

This editor can be re-used freely in the current Seed app by simply calling the `markdown_editor` function
freely multiple times.  For instance changing the view to the following creates four markdown editors
all of which function independently.

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
div![
    markdown_editor(Msg::SubmitMarkdownHtml),
    markdown_editor(Msg::SubmitMarkdownHtml),
    markdown_editor(Msg::SubmitMarkdownHtml),
    markdown_editor(Msg::SubmitMarkdownHtml),
]
}
```

However there is an issue if we want to re-use this component in a different app. This is because the component
currently relies on specifically that the `Msg` type to be used as the `on_submit` argument and in the return type as part of a `Node<Msg>`.

We therefore need to adjust the code to allow for this function to be freely re-used in any Seed application that may use a completely different
message type.  

In order to do this we will make the function generic over the message type.

Change the `markdown-editor` function signature as follows:

```
#[topo::nested]
fn markdown_editor<Ms, F>(on_submit: F) -> Node<Ms>
where
F: FnOnce(String) -> Ms + 'static + Clone,
Ms: 'static,
{
    ...
```
What we have done is create a generic function rather than a function that is specific for our app's `Msg` type.
The `Ms` type is a type parameter that we supply to the function to tell it what message type it should use.

We also need to ensure that all parts of our component refer to this generic `Ms` type.

If you look in the function body of `markdown_editor` you will not see any `Msg` type referenced directly,
and therefore you might think that no further changes are needed. However have a look again at the `scroll_event_handler` function: 

```
fn scroll_event_handler<Msg>(
event: Ev,
textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
preview_el: StateAccess<ElRef<HtmlElement>>,
) -> EventHandler<Msg> {
```

This return an `EventHandler<Msg>` and the `Msg` is concrete here!

To fix we simply use a generic type parameter here as well. So replace the `scroll_event_handler` signature with the below:  

```
fn scroll_event_handler<Ms>(
event: Ev,
textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
preview_el: StateAccess<ElRef<HtmlElement>>,
) -> EventHandler<Ms>
where
Ms: Default + 'static,
{
```

Now this function will use the a generic `Ms` type as well. Rust is smart enough to
realise that it has to use the same `Ms` type due to the return type of `markdown_editor`. 

Once the above changes are made then we can call our markdown editor as below and use it freely in any Seed app.

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
markdown_editor(Msg::SubmitMarkdownHtml)
}
```

The great thing is because the `Ms` type is defined by the argument passed to `on_submit` (in this case `Msg::SubmitMarkdownHtml`) **we don't actually have to 
explicitly state the message type to be used**. Our api surface is clean and easy to use.

### Taking the widget further.  

As it stands the widget is composable, versatile and fulfills the brief.   It can be
freely used in any Seed project (as long as hooks have been enabled) and can be rendered to the 
view with a simple function call. 

That said, it could be extended in a number of ways.  Here are some ideas:

* Configurable styling / theme. 
* A "Cancel" button along with associated Seed messaging.  
* Buttons and shortcuts to quickly add things like bolding text. I.e. a short cut that bolds
the highlighted word on `CTRL/CMD-B`.
* A "Reset" button that clears the edit pane.
* A version with a 'preview' and 'edit' tab instead of side by side panes.  
* Performance considerations - on modern hardware (2019 Laptop) every keypress of the above widget
takes 7.5ms to complete in release mode and 33ms to complete in debug mode. This is probably acceptable
considering that this entire page is also being re-rendered and diffed in seed (including al of this tutorial markdown).
That said some performance tweaks could be implemented. For instance, not re-rendering the output on every single 
keypress and only at fixed intevals.
* A Seed Hooks version - instead of taking a `Msg` argument it takes a Seed Hooks state accessor. 
The widget then updates the accessor's state variable on submit.



For the last suggestion, the only things you would need to adjust are the function signature for the `markdown_editor` 
and the submit button callback:

```
// in alt_md.rs

pub fn markdown_editor_state<Ms, F>(md_state: StateAccess<String>) -> Node<Ms>
where
F: FnOnce(String) -> Ms + 'static + Clone,
Ms: Default + 'static,
{
...
button![
    class!["bg-green-400 p-4 m-2"],
    "Submit (See console log)",
    mouse_ev(Ev::Click, move |_| {
        let markdown_element = preview_el.get().get().expect("markdown-body doesn't exist");
        md_state.set(markdown_element.inner_html()); 
    })
]
...
}
```



"#]






    ]
})
}