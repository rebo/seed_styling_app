use super::theme::*;
use seed_style::*;

pub fn init_styles() {
    GlobalStyle::default()
        .style(
            "a,ul,li,div,p,h1,h2,h3,h4,li,dd,dt,button,label,input",
            s().font_family("'Lato',sans-serif")
                .webkit_font_smoothing_antialiased(),
        )
        .style("button", s().align_self_center().outline_style_none())
        .style("html", s().box_sizing_border_box())
        .style("img", s().box_sizing_content_box())
        .style("*, *:before, *:after", s().box_sizing("inherit"))
        // make sure we never ever get horizontal scrollbars
        .style("body", s().max_width(vw(100)))
        .activate_init_styles()
}

pub fn themed_global_styles() {
    GlobalStyle::default()
        .style(
            "p",
            s().line_height(CssLineHeight::Number(1.6))
                .letter_spacing(rem(-0.00278))
                .mt(3),
        )
        .style(
            "body",
            s().bg_color(Color::Background).color(Color::MainText),
        )
        .style("a", s().color(Color::MainText))
        .style("a", s().visited().color(Color::MainText))
        .style("a", s().hover().color(Color::Primary))
        .style(
            "p code",
            s().color(Color::MainText)
                .font_size(0)
                .m(0)
                .p(0)
                .radius(px(3))
                .bg_color(Color::MutedSecondary),
        )
        .style(
            "h1",
            s().font_weight_v900()
                .display_block()
                .py(3)
                .mt(px(64 + 8))
                .mb(5)
                .font_size(px(36)),
        )
        .style(
            "h2",
            s().font_weight_v900()
                .b_style_solid()
                .bb_width(1)
                .b_color(Color::Primary)
                .display_block()
                .py(3)
                .mt(3)
                .mb(4)
                .font_size(px(24)),
        )
        .style(
            "h4",
            s().font_style_italic()
                .font_weight_v700()
                .display_block()
                .mt(1)
                .mb(2)
                .font_size(px(18)),
        )
        .style(
            "pre",
            s().color(Color::MainText)
                .display_flex()
                .flex_direction_row()
                .justify_content_center()
                .width(pc(100))
                .max_width(pc(100))
                .overflow_x_auto()
                .box_sizing_border_box()
                .px(4)
                .py(3)
                .my(2),
        )
        .style(
            "pre > code",
            s().display_inline_block()
                .bg_color(Color::MutedSecondary)
                .b_style_dotted()
                .overflow_x_auto()
                .display_inline_block()
                .b_width(1)
                .b_color(Color::Secondary)
                .box_sizing_border_box()
                .px(4)
                .py(3)
                .my(2)
                .radius(px(5))
                .font_size(&[px(14), px(18)]),
        )
        .activate_styles();
}
