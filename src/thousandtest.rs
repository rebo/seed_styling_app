use crate::{Model, Msg};
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::measures::px;
use seed_style::*;

use rand::prelude::*;

#[topo::nested]
pub fn view(_model: &Model) -> Node<Msg> {
    div![
        s().display_flex().flex_direction_column(),
        h1![s().font_size(px(18)).pb(px(5)), "Load Test"],
        p![
            s().px(px(4)),
            r#"Each time you hit redraw this will create 1000
divs each with a freshly calculated random style. These styles then get 
added to the style sheet.  Obviously this is very workload heavy, typically seed style caches
repeated styles therefore the stylesheet is only being hit on new styles.  On a 2011 laptop in release mode
 this renders in 84ms for a fresh style-uncached render. If styles are cached, which is 
usually the case, this renders in 16ms. "#
        ],
        p![r#"
Note: not much work has yet gone into improving styling performance yet.
Everytime you hit redraw it adds 1000 styles to the app's stylesheet so you probably want to refresh after testing this!
Note #2: If this appears laggy you probably have built in debug mode the timing here is approx 450ms!.
"#],
        button![
            s().radius(px(4))
                .w_auto()
                .mx(px(24))
                .my(px(4))
                .box_sizing_border_box()
                .bg_color(seed_colors::Red::No5)
                .px(px(24))
                .py(px(8)),
            "redraw!",
            mouse_ev(Ev::Click, |_| ())
        ],
        div![
            s().display_grid()
                .grid_template_columns("repeat(40,auto)")
                .grid_template_rows("repeat(25,auto)"),
            (0..1000).map(|_| div![s()
                .bg_color(hsl(
                    random::<f64>() * 255.,
                    random::<f64>() * 255.,
                    random::<f64>() * 100.
                ))
                .width(px(10))
                .height(px(10))])
        ]
    ]
}
