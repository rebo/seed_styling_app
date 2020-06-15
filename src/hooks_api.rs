use crate::compositions::*;
use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::{px,pc,rgba, ExactLength};
use seed_style::*;
use crate::app_styling::theme::*;


#[topo::nested]
fn set_list() -> Node<Msg> {
    let selected = use_state(||"");

    ul![ "Selected Item:", selected.get(),
        li![if selected.get()=="1" {s().bg_color(seed_colors::Green::No5)} else {s().bg_color(seed_colors::Red::No6)},s().radius(px(3)).p(px(5)).m(px(4)),"1st Item", mouse_ev(Ev::Click, move |_| selected.set("1")), s().cursor_pointer()],
        li![if selected.get()=="2" {s().bg_color(seed_colors::Green::No5)} else {s().bg_color(seed_colors::Red::No6)},s().radius(px(3)).p(px(5)).m(px(4)),"2nd Item", mouse_ev(Ev::Click, move |_| selected.set("2")),s().cursor_pointer()],
        li![if selected.get()=="3" {s().bg_color(seed_colors::Green::No5)} else {s().bg_color(seed_colors::Red::No6)},s().radius(px(3)).p(px(5)).m(px(4)),"3rd Item", mouse_ev(Ev::Click, move |_| selected.set("3")),s().cursor_pointer()],
        li![if selected.get()=="4" {s().bg_color(seed_colors::Green::No5)} else {s().bg_color(seed_colors::Red::No6)},s().radius(px(3)).p(px(5)).m(px(4)),"4th Item", mouse_ev(Ev::Click, move |_| selected.set("4")), s().cursor_pointer()],
        li![if selected.get()=="5" {s().bg_color(seed_colors::Green::No5)} else {s().bg_color(seed_colors::Red::No6)},s().radius(px(3)).p(px(5)).m(px(4)),"5th Item", mouse_ev(Ev::Click, move |_| selected.set("5")), s().cursor_pointer()],
    ]
}


#[topo::nested]
fn numberbind() -> Node<Msg> {
    let a = use_state(|| 0);
    let b = use_state(|| 0);

    div![
        input![attrs![At::Type=>"number"], bind(At::Value, a)],
        input![attrs![At::Type=>"number"], bind(At::Value, b)],
        p![a, "+", b,  "=" , a + b]
    ]
} 


struct NonCloneString(String);



#[topo::nested]
fn my_non_clone_input() -> Node<Msg> {
    let input_access = use_state(|| NonCloneString("".to_string()));
    let val = input_access.get_with(|v| format!("{}", v.0));

    div![
        input![attrs![At::Value => val], 
        s().border_color(seed_colors::Gray::No6).radius(px(4)).border_width(px(1)).py(px(4)),
            input_access.input_ev(Ev::Input, |i,text| *i=NonCloneString(text))
        ],
        format!("Text inputted: {}", val)
    ]
}

#[view_macro]
fn center_view<Ms>( root: Node<Ms>, children: Vec<Node<Ms>>) -> Node<Ms> {
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

fn update_example() -> Node<Msg> {
    let count = use_state(|| 3);

    div![
        big_button![
            s().bg_color(seed_colors::Red::No4),
            "-",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v -= 1);
                Msg::NoOp
            }),
        ],
        count,
        big_button![
            s().bg_color(seed_colors::Green::No4),
            "+",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v += 1);
                Msg::NoOp
            }),
        ],
    ]
}




#[derive(Default)]
pub struct AdditionalStyle {
    pub style: Option<seed_style::Style>,
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

        


//     ]

// }
    // let base_button = nicer_button![
    //     s().padding_x(&[px(4), px(8)])
    //         .padding_y(&[px(2), px(4)])
    //         .margin(px(8))
    //         .margin_top(px(18))
    //         .font_size(&[px(12), px(24)]),
    //         children
    // ];

    // // as_tag![
    // //     button,
    // //     base_button
    // // ]

    //     if let Some(style) = args.AdditionalStyles{
    //         styles
    //     } else {
    //         s()
    //     },
    //     children
    // ]
// }



#[topo::nested]
fn my_input() -> Node<Msg> {
    let input_access = use_state(|| "".to_string());

    Column![
        input![attrs![At::Value => input_access.get()], 
        s().border_color(seed_colors::Gray::No6).radius(px(4)).border_width(px(1)).py(px(4)),
            input_access.input_ev(Ev::Input, |i,text| *i=text)
        ],
        "Text inputted:", input_access
    ]
}


#[topo::nested]
fn new_state_example() -> Node<Msg> {
    let todos = use_state(|| vec![(use_state(String::new),false)]);
    div![
        todos.get().iter().enumerate().map(|(idx, todo)| {
            vec![
                input![
                    s().border_color(seed_colors::Gray::No6).radius(px(4)).border_width(px(1)).py(px(4)),
                    bind(At::Value, todo.0)
                ],
                small_button![
                    s().bg_color(seed_colors::Red::No6).color(seed_colors::Base::White),
                    s().hover().bg_color(seed_colors::Red::No5),
                    todos.on_click(move |t| {t.remove(idx);}),
                    "X" 
                ],
                if todo.1{
                    span!["(completed)"]
                } else {
                small_button![
                    s().bg_color(seed_colors::Green::No6).color(seed_colors::Base::White),
                    s().hover().bg_color(seed_colors::Green::No5),
                    todos.on_click(move |t| t[idx].1 = true),
                    "√" 
                ]
                },
            br![],
            ]
        }),
       big_button![
           s().bg_color(seed_colors::Blue::No6).color(seed_colors::Base::White),
           s().hover().bg_color(seed_colors::Blue::No5),
           todos.on_click( move |t| t.push((new_state(String::new),false))),
            "Add Todo" 
        ]
    ]
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


pub fn view(_model : &Model) -> Node<Msg>{
    
    render_centred_article(_model, |_| {
        let modal_content = use_state(||None);
        
    div![modal(modal_content),
    h1!["Seed Hooks Api Notes"],
    md![r#"
There are several functions, methods and objects that contribute to the functionality of seed hooks.
Some of the most important are described here.

## State Functions

State functions are functinos such as `use_state` that either store local component state or are important 
to some hook patterns such as `after_render` and `do_once`.

    "#]

    ,
        sec![
            title = "use_state".to_string(),
            sig ="fn use_state<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T>".to_string(),
            modal_content = modal_content,
            code_example = Some(use_state_example),
            description![
                md![r#"
`use_state` is the standard state function for storing of a local **state variable** for a component. 
It returns a `StateAccess` **state accessor** which is responsible for all getting, setting and updating of the underlying value.

The function takes a lazily evaluated closure which returns a value that gets stored on first execution. The stored value is
linked to the component by a `topo::Id` which is also stored in the `StateAccess` accessor.

The only limit on the type of value stored is a `'static` lifetime.
If the type implements clone then `get()` can be used to clone the underlying value.

The code snippet demonstrates the use of `use_state` to store a count which gets updated on a button click.
"#]
            ],
            code_example![
r#"#[topo::nested]
fn my_button() -> Node<Msg> {
    let count = use_state(|| 0);

    div![
        count.get(),
        button!["+", count.mouse_ev(Ev::Click, |count, _| *count += 1)],
    ]
}"#       
            ]
        ],

        sec![
            title = "new_state".to_string(),
            modal_content = modal_content,
            code_example = Some(new_state_example),
            sig ="fn new_state<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T>".to_string(),
            description![
                md![  r#"This function is identical to `use_state` with the exception that every time the function is executed it creates a new 
topological context. The closure runs on every execution.

The use-case for this is to allow creation of state variables and associated accessors in an event callback.

Consider the following code from a todos app. It will store a **state variable** when the button is clicked. But because the call-site, parent call tree, 
and slot are all identical it will undesirably refer to the same `topo::Id`. Therefore when multiple buttons are rendered in a view they
will all update the same state.

```
button![
    todos.mouse_ev(Ev::Click, move |t,_| t.push(use_state(String::new))),
    "Add" 
]
```
The problem with this is that every state accessor stored within the todo list will effectively refer to the same component. 
Simply using `new_state` instead of `use_state` will ensure that every accessor stored will refer to a new topological context:

```
button![
    todos.mouse_ev(Ev::Click, move |t,_| t.push(new_state(String::new))),
    "Add" 
]
```
Generally new_state should only be used in an EventHandler callback and never in a general view function.


This code example is a fully interactive todo list in 15 line of code. `todos` is a state accessor that stores 
a `Vec` of `String` state accessors, these are then used to store the state of each todo. 

`new_state` is used in the on click event do add a new unique todo.
"#]
],
            code_example![
r#"
#[topo::nested]
fn new_state_example() -> Node<Msg> {
    let todos = use_state(|| vec![(use_state(String::new),false)]);
    div![
        todos.get().iter().enumerate().map(|(idx, todo)| {
            vec![
                input![
                    bind(At::Value, todo.0)
                ],
                button![
                    todos.on_click(move |t| {t.remove(idx);}),
                    "X" 
                ],
                if todo.1{
                    span!["(completed)"]
                } else {
                button![
                    todos.on_click(move |t| t[idx].1 = true),
                    "√" 
                ]
                },
            br![],
            ]
        }),
       button![
           todos.on_click( move |t| t.push((new_state(String::new),false))),
            "Add Todo" 
        ]
    ]
}
"#       
            ]
        ],

        sec![
            title = "after_render".to_string(),
            modal_content = modal_content,
            code_example = Some(if_example),
            sig ="fn after_render<F: Fn(f64) -> () + 'static>(func: F)".to_string(),
            description![
                md![  r#"`after_render()` executes the closure supplied after the next render. The execution runs asynchronously,
specifically after the DOM tree has been created, diffed, and after the view has been painted to the window.
Often this is combined with `do_once()` which schedules an closure to be executed only once after the next page render.  

You typically use `after_render()` when triggering a dom interaction, for instance an animation or popup that is not part of the virtual dom tree.

This example renders two input boxes and after an input event schedules a calculation to update the dom manually"#
],
            code_example![
r#"
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
        input![bind(At::Value, input_a)],
        "B:",
        input![bind(At::Value, input_b)],
        ul![id!("list"), "Smallest Log:"],
    ]
}


"#       
            ]
        ],

        ]
        ,
md![
r#"
## StateAccess<T>

Seed Hook's **State Functions** return a `StateAccess<T>` value. This is an accessor which
provides amongst other features getter and setter access to the stored value of type T.

The `StateAccess<T>` accessor knows what component's state to update and therefore this accessor can be used 
in `EventHandler` callbacks to update state.

Please note that unlike React Hooks StateAccess getter & setters do not reschedule a re-render of the 
virtual DOM.

The struct implements `Copy` and therefore can be freely shared, this is independent as to whether `T` implements `Copy`.

The primary method used to retrieve stored data is `get()`, this only works with `Clone` types. For non-`Clone` types
the `get_with()` method is available.

Advanced patterns include using `bind()` to link an accessor to a DOM element's attribute or storing a 
collection of state accessors to manage complex tree structures.

"#
]
        ,



        sec![
            title = "get()".to_string(),
            modal_content = modal_content,
            code_example = Some(my_input),
            sig ="fn get(&self) -> T // T must be Clone + 'static".to_string(),
            description![
                md![  r#"This method returns a clone of the stored data, therefore in order for it to be used `T` must of course implement `Clone`.
Although all accesses will therefore cause an allocation due to the clone, this is the most direct way in which to access the stored data.
Care should be taken in understanding that the clone may be stale if this value is used in a callback.

For this reason using `update()` in a callback is usually preferable to using `set()`.

The example demonstrates displaying a value stored by an accessor from an `Input` event."#
],
            code_example![
r#"

#[topo::nested]
fn my_input() -> Node<Msg> {
    let input_access = use_state(|| "".to_string());

    div![
        input![attrs![At::Value => input_access.get()], 
            input_ev(Ev::Input , |text| {
                input_access.set(text);
                Msg::default()
            })
        ],
        format!("Text inputted: {}", input_access.get())
    ]
}

"#       
            ]
        ],

        ],
        sec![
            title = "get_with()".to_string(),
            modal_content = modal_content,
            code_example = Some(my_non_clone_input),
            sig ="fn get_with<F: FnOnce(&T) -> R, R>(self, func: F) -> R".to_string(),
            description![
                md![  r#"This method provides read access to a stored store variable via a closure.
This method is primarily used to read non-`Clone` values or where cloning is seen as expensive.

The typical pattern is to return a representation of the data stored from the `get_with()` closure. 
For instance, if a non-`Clone` struct that contains date information is stored then the closure might return
a `String` representation of this date information.

It is essential to understand that in order to provide unfettered read access to the stored value 
`get_with()` temporarily removes the value from the backing-store and re-inserts it at the end of the block. 
The effect of this is that any use of the `StateAccess`or within the `get_with()` closure is almost always an error.

The example demonstrates displaying a non Clone value stored by an accessor from an `Input` event"#
],
            code_example![
r#"
struct NonCloneString(String);

#[topo::nested]
fn my_non_clone_input() -> Node<Msg> {
    let input_access = use_state(|| NonCloneString("".to_string()));
    let val = input_access.get_with(|v| format!("{}", v.0));

    div![
        input![attrs![At::Value => val], 
            input_access.input_ev(Ev::Input, |i,text| *i=NonCloneString(text))
        ],
        format!("Text inputted: {}", val)
    ]
}

"#       
            ]
        ],

        ],

        sec![
            title = "set()".to_string(),
            modal_content = modal_content,
            code_example = Some(set_list),
            sig ="fn set(self, value: T)".to_string(),
            description![
                md![  r#"This method simply updates the stored value. `set()` is generally called in an `EventHandler` callback.
If the updated value depends on the current value it is generally better to use `update()` rather than `set()`

This example uses `set()` to set the value based on a clicked item in a list."#
],
            code_example![
r#"
#[topo::nested]
fn set_list() -> Node<Msg> {
    let selected = use_state(||"");

    ul![ "Selected Item:", selected.get(),
        li!["1st Item", mouse_ev(Ev::Click, move |_| { selected.set("1"); Msg::default() }), class![C.cursor_pointer]],
        li!["2nd Item", mouse_ev(Ev::Click, move |_| { selected.set("2"); Msg::default() }), class![C.cursor_pointer]],
        li!["3rd Item", mouse_ev(Ev::Click, move |_| { selected.set("3"); Msg::default() }), class![C.cursor_pointer]],
        li!["4th Item", mouse_ev(Ev::Click, move |_| { selected.set("4"); Msg::default() }), class![C.cursor_pointer]],
        li!["5th Item", mouse_ev(Ev::Click, move |_| { selected.set("5"); Msg::default() }), class![C.cursor_pointer]],
    ]
}

"#       
            ]
        ],

        ],

        sec![
            title = "update()".to_string(),
            modal_content = modal_content,
            code_example = Some(update_example),
            sig ="fn update<F: FnOnce(&mut T) -> ()>(self, func: F)".to_string(),
            description![
md![  r#"
This method simply updates the stored value by providing mutable access within a closure.
This is the prefered method if updating a value in place, particularly if the change depends on the existing value.

It is essential to understand that in order to provide unfettered read access to the stored value 
`update()` temporarily removes the value from the backing-store and re-inserts it at the end of the block. 
The effect of this is that any use of the `StateAccess`or within the `update()` closure is almost always an error.

This example demonstrates a increasing / decreasing counter by using `update()` on a `mouse_ev` click event.  Note that 
a shortcut that simplifies this pattern is  `count.on_click(|v| *v+=1 )`."#
],
            code_example![
r#"
#[topo::nested]
fn update_example() -> Node<Msg> {
    let count = use_state(|| 3);

    div![
        button![
            "-",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v -= 1);
                Msg::NoOp
            }),
        ],
        count,
        button![
            "+",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v += 1);
                Msg::NoOp
            }),
        ],
    ]
}
"#       
            ]
        ],

        ],

        md![r#"
## Developer Experience

Seed hooks provide a number of functions to simplify working with hooks.

"#],
        sec![
            title = "bind()".to_string(),
            modal_content = modal_content,
            code_example = Some(numberbind),
            sig ="fn bind<Ms: Default, T: 'static + std::str::FromStr + std::fmt::Display>( attr: At,
                val: StateAccess<T>,) -> (seed::virtual_dom::attrs::Attrs, seed::EventHandler<Ms>)".to_string(),
            description![
md![  r#"
it is a common requirement that the value of element attributes such as an input's 
value attribute is linked to some value. `bind()` provides a shortcut to link an attribute to a value.
You simplfy specify the attribute and state accessor to bind.  Currently limited to updating on `Input` events, 
therefore currently only usable with `input![]` elements.

The example on the right binds integers to an input and then calculates a value with them. Under this example is 
the code for similar functionality but without using `bind()`.                
"#
],
            code_example![
r#"
#[topo::nested]
fn numberbind() -> Node<Msg> {
    let a = use_state(|| 0);
    let b = use_state(|| 0);

    div![
        input![attrs![At::Type=>"number"], bind(At::Value, a)],
        input![attrs![At::Type=>"number"], bind(At::Value, b)],
        p![format!("{} + {} = {}", a, b, a + b)]
    ]
}


// Without bind there is a lot more boilerplate: 

#[topo::nested]
fn number_without_bind() -> Node<Msg> {
    let a = use_state(|| 0);
    let b = use_state(|| 0);

    div![
        input![attrs![At::Type=>"number", At::Value => a.get()], 
            input_ev(Ev::Input, |text| 
                {
                    if let Ok(a_i32) = text.parse::<i32>() {
                        a.set(a_i32)
                    }
                }
            )
        ],
        input![attrs![At::Type=>"number", At::Value => b.get()], 
        input_ev(Ev::Input, |text| 
            {
                if let Ok(b_i32) = text.parse::<i32>() {
                    b.set(a_i32)
                }
            }
        )
    ],
        p![format!("{} + {} = {}", a.get(), b.get(), a.get() + b.get())]
    ]
}

"#       
            ]
        ],

        ],
        md![
           r#"
## Glossary

**Seed hook** - any of the Seed functions that facilitate storing and updating of *component state*.  For example, functions such as `use_state`, `new_state` and `bind`. The term *hook* refers to React Hooks which have similar functionality.

**Component state** - the collection of *state variables* that are used and stored by a component.

**State variable** - a variable that is stored locally in a component by `use_state` or `new_state`.  i.e. the integer value in:

```
let counter = use_state(||0);
```

**State accessor**  - responsible for getting, setting, and updating a *state variable*
within its linked *topological context*. i.e. `name` in:

```
let name = use_state(||"Bob");
```

**Topologically aware function** - a function that has been annotated with `[topo::nested]`.
This function will have its own `topo::Id`. I.e. the below function is topologically aware: 

```
#[topo::nested]
pub fn view(model: &Model) -> impl View<Msg> {
    div![]
}
```        

**Topological context** -  the execution context of a *topologically aware function*. Based on
where in the source the function was called, any parent topologically aware functions, and 
a `slot` which counts sibling functions. Represented by a `topo::Id` value. i.e. the two `child` function calls
have a different topological context.

```
#[topo::nested]
pub fn parent(model: &Model) -> impl View<Msg> {
    div![
        child(),
        child()
    ]
}

#[topo::nested]
pub fn child(model: &Model) -> impl View<Msg> {
    span!["hi]
}
```

**Topology** - The tree like structure that is created by nesting of *topologically aware functions*. If `a()`, `b()`, `c()`
and `d()` are such functions and they are called in the following way:

```

fn layout(){
    a();
}

#[topo::nested]
fn a(){
    b();
    b():
    c();
}

#[topo::nested]
fn b(){
    c();
    c();
}

#[topo::nested]
fn c(){
    d();
}

#[topo::nested]
fn d(){
}
```

then the *topology* of `layout()` is : 

```
                root(a)
     _______________________  
    |            |          |
    b()          b()        c()
 _____         _____        |
|     |       |     |       d()
c()   c()     c()   c()
|     |       |     |
d()   d()     d()   d()
```
The reason this is important is that each node above will have its own `topo::Id` which means it can be uniquely identified. Therefore state can be associated locally with each node even though it is the same function.

**Execution context** - Where in the above topology a function has been executed.


**Slot** - An index for sibling nodes in the above topology. Needed to differentiate between two sequential `c()` calls. Occasionally needed to be specified when iterating over components in a non-stable order.  






            
            
            "#
        ]






    ]
})
}