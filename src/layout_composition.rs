use seed::{prelude::*};
use seed_style::*;
use seed_style::measures::{px,pc};
use crate::{Model,Msg};
use crate::theme::Breakpoint;
////////////////////////////////////////////////////
//
//  Layout Composition Example 
//
//  Demonstrates how to create custom areas that can be ued to define
//  a layout, completely agnostic of the contents of those areas.
//
//  This example is split between the layout definition
//  And the view which uses the layout in a  composition
//
////////////////////////////////////////////////////
// Layout definition
// ------------------------------------------------
// This example will be laid out as follows.
//
// The main page will be a traditional header sidebar content & footer at larger screens.
// And a linear Header, collapsable nav, content and footer at smaller screens.
//
// The header is split into logo, title and actions at larger screens
// And title and hambuger button at smaller screens
//
// The Sidebar has a title plus repeatable elements
//
// The main content has repeatable content arranged in a in a grid.

// We first define all the possible areas that can be arranged on the main app page
// They will not all be rendered, this will determined on the specific breakpoint
//
// Layout makes use of css grid extensively
// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum AppArea {
    Header,
    MainContent,
    Nav,
    Sidebar,
    Footer,
    None,
}
impl LayoutArea for AppArea {} // Allow Area to act as a LayoutArea for layout purposes.

// We now define some layouts to be used in the app.  The first a larger layout.
fn app_layout_large() -> Layout<AppArea> {
    use AppArea::*;
    
    #[cfg_attr(rustfmt, rustfmt_skip)] // ensure the layout array formatting matches the layout
    let layout = Layout::areas(&[
        &[ Header , Header     , Header     ],
        &[ Sidebar, MainContent, MainContent],
        &[ Footer , Footer     , Footer     ],
    ]);

    // `layout.style` applies a style to the grid container element
    // grid-template-rows set to `auto 1fr auto` to make the header and footer auto sized
    //  - the middle content row the largest possible size.
    // min-height 100% on the container to fill the app's container
    layout.style(
        S.grid_template_rows("auto 1fr auto")   // You can set styles with snake_case css property name and a string.
            .min_height(pc(100.)) // Or use typed measurement helpers `px()` - pixel, `pc()` - percent, `rem()` `em()` etc.
            .grid_gap(px(8)),
    )
    // This layout definition is completely content agnostic.
    // Infact it can be used in any seed app that needs a similar arrangement of areas.
}

// Small layout for devices such as phones, column layout.
// Note here we have `Nav` conditionally rendered instead of `Sidebar`,
fn app_layout_small() -> Layout<AppArea> {
    use AppArea::*;

    #[cfg_attr(rustfmt, rustfmt_skip)] // ensure the layout array formatting matches the layout
     Layout::areas(
        &[
            &[Header     ], 
            &[Nav        ], 
            &[MainContent], 
            &[Footer     ],
    ])
        .style(
            S.grid_template_rows("auto auto 1fr auto").min_height(pc(100.))
        )
}

// We have just defined the layouts for the main page, however we can also declare
// our layouts for other parts of the page.
// All layouts are completely content agnostic.

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum HeaderArea {
    Title,
    Actions,
    Hamburger,
    Logo,
}
impl LayoutArea for HeaderArea {}

// We can actually use the build pattern to remove much cruft.
fn header_layout_large() -> Layout<HeaderArea> {
    use HeaderArea::*;

    Layout::areas(&[&[Logo, Title, Title, Title, Actions]]) // Horizontal layout of logo title and actions
        .style(S.grid_template_columns("auto 1fr 1fr 1fr auto"))
        .area_style(Actions, S.justify_self_right()) // ensure actions are justified to the right
        .area_style(Logo, S.justify_self_left()) // ensure the logo is justified to the left
        
}

fn header_layout_small() -> Layout<HeaderArea> {
    use HeaderArea::*;
    Layout::areas(&[&[Title, Hamburger]])
        .style(S.grid_template_columns("1fr auto"))
        .area_style(Hamburger, S.justify_self_right())
        
}


// The Sidebar layout - consists of a title and then filled with a list of travel destinations
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Sidebar {
    Title,
}
impl LayoutArea for Sidebar {}

fn sidebar_layout() -> Layout<Sidebar> {
    use Sidebar::*;
    
    #[cfg_attr(rustfmt, rustfmt_skip)] 
    Layout::areas(&[&[Title]]) // a spanning title followed by repeating unamed list of travel destinations
        .style( 
            S.grid_template_rows("auto 1fr")
                .grid_gap(px(16))
                .justify_items_center(),
        )
        
}

// The Main Content Layout  notice this has no named areas
// Just a repeating grid, therefore we use the 'NoArea' Area type.
fn main_content_layout() -> Layout<NoArea> {
    
    Layout::<NoArea>::grid()
        .style(
        S.grid_template_columns("repeat(auto-fit, minmax(250px, 1fr))")
            .grid_auto_flow_row()
            .justify_items_center()
            .grid_template_rows("auto 1fr")
            .grid_gap(px(16))
            .w(pc(100.)),
        )
        
}




//
// Reviewing the code above you will see that there is zero content described.
// Just the layout of user defined areas. 
//
// In theory the above layouts in any app with similar layouts / Themes.
// These could be cut & pasted out freely, or stored in their own crate.
// Typically the above would be in a separate layout file that defines layout primatives

//// ------------------ End of Layout Definition ---------------------

////////////////////////////////////////////////////
// Layout View
// ------------------------------------------------
// We uses the above defined layouts in breakpoint aware`Compositions` that link
// layouts and content together. Content can be other compositions
//  
// Layout makes use of css grid extensively
// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout


pub fn view(model: &Model) -> Node<Msg> {
    use AppArea::*;
    use Breakpoint::*;

    //  To enable differerent layouts on different breakpoints we add layouts to a composition and render
    //
    //  If no breakpoints match, the composition will choose the smallest one.
    Composition::with_layouts(&[
        (Small, app_layout_large()),
        (ExtraSmall, app_layout_small()),
        ])
        // We assign content views to the various named areas, typically you would use a function pointer
        // however here we show how to use nested compositions.
        .set_content(Header, |model| 
            Composition::with_layouts(&[
                (Small, header_layout_large()),
                (ExtraSmall, header_layout_small()),
            ])
            .render(model)
        )
        .set_content(Sidebar, |model| 
            Composition::with_layout(ExtraSmall, sidebar_layout())
                // composition includes a mock_children helper which helps render multiple mock children divs
                // this is useful when planning out the layout
                //
                // once you are ready to create the child you would use the `add_child` method to add a child node directly
                .mock_children("Dest.", 20, px(400), px(100))
                .render(model)
        )
        .set_content(MainContent, |model| 
            Composition::with_layout(ExtraSmall, main_content_layout())
                .mock_children("Photo", 20, px(250), px(250))
                .render(model)
        )
    // notice we have not hooked up Footer with anything yet...

    // Also notice we have not actually really created any content, we have just worked on the generate layout.

    // We now render the above composition.
    .render(model)
}