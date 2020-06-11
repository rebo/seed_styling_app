use crate::compositions::*;
use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::*;
use seed_style::{pc, px};
use seed_style::Row;


// #[derive(Default)]
// struct CenterArgs{}

pub fn view(model: &Model) -> Node<Msg> {
    render_centred_article(model, |_| {
        div![
            h1!["Simple Layout"],
            md![r#"
Seed Style provides both simple layout primitives as well as a fully comprehensive compositing layout system. 

## Row Layout

You can layout elements within a Row by using the `Row![]` macro. Each Item is then aligned left,center, or right by setting
the `align` argument to `RowAlign::Left`, `RowAlign::Right`, or `RowAlign::Center`.  Any children nodes not in an `Item![]`
are automatically assigned `RowAlign::left`. 

Options on `Row` are :

1. `gap`  - expects an ExactLength to set the gap between Items in the Row an
1. `flex` - often a `Row!` or `Column!` is a flex item, therefore flex will set this property. Default is `Flex::Full` i.e. `flex: 1 1 0%; min-height: 0px;min-width: 0px;` Which ensures 
the element grows and shrinks as much as needed. Other choices are `Flex::None` no flex, `Flex::Content` fits the content minimally, and Flex::Custom() for a custom value.
1. `padding` - the space around each item in the row or column.

Options on `Item` are :

1. `flex` - an Item is always a flex item, therefore flex will set this property. Default is `Flex::Content` which fits the content minimally. Other choices are `Flex::None` no flex,  
`Flex::Full` i.e. `flex: 1 1 0%; min-height: 0px;min-width: 0px;` Which ensures the element grows and shrinks as much as needed., and Flex::Custom() for a custom value.
1. RowAlign which aligns the item to Left, Center, Right, LeftTop, CenterTop, RightTop, LeftMiddle, CenterMiddle, RightMiddle, LeftBottom, CenterBottom, RightBottom.

Row `Items` are laid out first, followed up the Row's children in the left position.

The Row is guaranteed to to be laid out as a Row with no wrapping.

```rust
Row![
    gap = px(12),
    padding = px(8),
    Item![ "hello 1", align = RowAlign::Left ],
    Item![ "hello 2", align = RowAlign::Left ],
    Item![ "hello 3", align = RowAlign::Left ],

    Item![ "Center hellow", align = RowAlign::Center],

    Item![ "hello 4", align = RowAlign::Right],
    Item![ "hello 5", align = RowAlign::Right],
    Item![ "hello 6", align = RowAlign::Right],
]
```

Example with some colour styling:

"#],
    Row![
            gap = px(12),
            padding = px(8),
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 1" , align = RowAlign::Left],
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 2" ,  align = RowAlign::Left],
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 3" ,  align = RowAlign::Left],

            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 4" , align = RowAlign::Right],
            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 5" , align = RowAlign::Right],
            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 6" , align = RowAlign::Right],

            Item![s().h(pc(100)).bg_color(seed_colors::Green::No4), "center hello" , align = RowAlign::Center],
            
    ],
    md![r#"
## Column Layout

You can layout elements within a vertical Column by using the `Column![]` macro. Each Item is then aligned top,center, or bottom by setting
the `align` argument to `ColumnAlign::Top`, `ColumnAlign::Bottom`, or `ColumnAlign::Middle`.  Any children nodes not in an `Item![]`
are automatically assigned`ColumnAlign::Top`. 


Options on `Row` are :

1. `gap`  - expects an ExactLength to set the gap between Items in the Row an
1. `flex` - often a `Row!` or `Column!` is a flex item, therefore flex will set this property. Default is `Flex::Full` i.e. `flex: 1 1 0%; min-height: 0px;min-width: 0px;` Which ensures 
the element grows and shrinks as much as needed. Other choices are `Flex::None` no flex, `Flex::Content` fits the content minimally, and Flex::Custom() for a custom value.
1. `padding` - the space around each item in the row or column.

Options on `Item` are :

1. `flex` - an Item is always a flex item, therefore flex will set this property. Default is `Flex::Content` which fits the content minimally. Other choices are `Flex::None` no flex,  
`Flex::Full` i.e. `flex: 1 1 0%; min-height: 0px;min-width: 0px;` Which ensures the element grows and shrinks as much as needed., and Flex::Custom() for a custom value.
1. ColumnAlign which aligns the item to  Top, Middle, Bottom, TopLeft, MiddleLeft, BottomLeft, TopCenter, MiddleCenter, BottomCenter, TopRight, MiddleRight, BottomRight.

Column `Items` are laid out first, followed up the Column's children in the top position.

The Column is guaranteed to to be laid out as a single column with no wrapping.

```rust
Column![
    gap = px(12),
    padding = px(8),
    
    Item![ "hello 1", align = ColumnAlign::Top ],
    Item![ "hello 2", align = ColumnAlign::Top ],
    Item![ "hello 3", align = ColumnAlign::Top ],

    Item![ "Middle hello", align = ColumnAlign::Middle],

    Item![ "BottomLeft"  , align = ColumnAlign::BottomLeft  ],
    Item![ "BottomCenter", align = ColumnAlign::BottomCenter],
    Item![ "BottomRight" , align = ColumnAlign::BottomRight ],
]
```

Example with some colour styling:

"#]
    , Column![
        gap = px(12),
        padding = px(8),
        Item![s().bg_color(seed_colors::Red::No4), "hello 1" , align = ColumnAlign::Top],
        Item![s().bg_color(seed_colors::Red::No4), "hello 2" ,  align = ColumnAlign::Top],
        Item![s().bg_color(seed_colors::Red::No4), "hello 3" ,  align = ColumnAlign::Top],

        Item![s().bg_color(seed_colors::Indigo::No4), "BottomLeft" , align = ColumnAlign::BottomLeft],
        Item![s().bg_color(seed_colors::Indigo::No4), "BottomCenter" , align = ColumnAlign::BottomCenter],
        Item![s().bg_color(seed_colors::Indigo::No4), "BottomRight" , align = ColumnAlign::BottomRight],

        Item![s().bg_color(seed_colors::Green::No4), "Middle hello" , align = ColumnAlign::Middle],
        
],

md![r#"of course you can put rows in columns:"#],

Column![
        gap = px(12),
        Item![ Row![
            gap = px(12),
            padding = px(8),
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 1" , align = RowAlign::Left],
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 2" ,  align = RowAlign::Left],
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 3" ,  align = RowAlign::Left],

            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 4" , align = RowAlign::Right],
            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 5" , align = RowAlign::Right],
            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 6" , align = RowAlign::Right],

            Item![s().h(pc(100)).bg_color(seed_colors::Green::No4), "center hello" , align = RowAlign::Center],
            
    ]],
        Item![Row![
            gap = px(12),
            padding = px(8),
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 1" , align = RowAlign::Left],
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 2" ,  align = RowAlign::Left],
            Item![s().h(pc(100)).bg_color(seed_colors::Red::No4), "hello 3" ,  align = RowAlign::Left],

            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 4" , align = RowAlign::Right],
            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 5" , align = RowAlign::Right],
            Item![s().h(pc(100)).bg_color(seed_colors::Indigo::No4), "hello 6" , align = RowAlign::Right],

            Item![s().h(pc(100)).bg_color(seed_colors::Green::No4), "center hello" , align = RowAlign::Center],
            
    ]],
      
],
md![r#"and columns in rows:"#],
Row![
    gap = px(12),
    Item![
        s().w(pc(50)),
        Column![
        gap = px(12),
        padding = px(8),
        Item![s().bg_color(seed_colors::Red::No4), "hello 1" , align = ColumnAlign::Top],
        Item![s().bg_color(seed_colors::Red::No4), "hello 2" ,  align = ColumnAlign::Top],
        Item![s().bg_color(seed_colors::Red::No4), "hello 3" ,  align = ColumnAlign::Top],

        Item![s().bg_color(seed_colors::Indigo::No4), "BottomLeft" , align = ColumnAlign::BottomLeft],
        Item![s().bg_color(seed_colors::Indigo::No4), "BottomCenter" , align = ColumnAlign::BottomCenter],
        Item![s().bg_color(seed_colors::Indigo::No4), "BottomRight" , align = ColumnAlign::BottomRight],

        Item![s().bg_color(seed_colors::Green::No4), "Middle hello" , align = ColumnAlign::Middle],
        
],],
    Item![s().w(pc(50)),Column![
        gap = px(12),
        padding = px(8),
        Item![s().bg_color(seed_colors::Red::No4), "hello 1" , align = ColumnAlign::Top],
        Item![s().bg_color(seed_colors::Red::No4), "hello 2" ,  align = ColumnAlign::Top],
        Item![s().bg_color(seed_colors::Red::No4), "hello 3" ,  align = ColumnAlign::Top],

        Item![s().bg_color(seed_colors::Indigo::No4), "BottomLeft" , align = ColumnAlign::BottomLeft],
        Item![s().bg_color(seed_colors::Indigo::No4), "BottomCenter" , align = ColumnAlign::BottomCenter],
        Item![s().bg_color(seed_colors::Indigo::No4), "BottomRight" , align = ColumnAlign::BottomRight],

        Item![s().bg_color(seed_colors::Green::No4), "Middle hello" , align = ColumnAlign::Middle],
        
]],
],
        md![
r#"## Creating your own layout primitives

The `Row` and `Column` described here are created with tools from Seed Style. Look in the [extending seed](/extending_seed) section 
for how to do this.
"#]
]
})
}


