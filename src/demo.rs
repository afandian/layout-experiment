use std::fs::File;
use std::io::Write;

use std::fmt::Write as _fw;
use layout;


// Spacing between lines on the stave. A notehead is also this height.
const LINE_SPACING: f32 = 10.0;
// Height of a place (tone) on the stave.
const PLACE_SPACING: f32 = LINE_SPACING / 2.0;


const NUM_LINES: u32 = 5;
const LINES_EITHER_SIDE_MIDDLE: u32 = (NUM_LINES - 1) / 2;
const STAVE_HEIGHT: f32 = (NUM_LINES - 1) as f32 * LINE_SPACING;

/// Callback to draw a stave as the background to a Node.
fn draw_stave(buf: &mut String, bounds: layout::BoundsWithOffset) -> () {
    let mut line_y = bounds.origin.1 - (STAVE_HEIGHT / 2.0) ;
   
    // We'll be given the origin and bounds, but it will be with relation to the middle line.
    // Therefore offset everything up by half a stave.

    for line in 0..NUM_LINES {
        write!(
            buf,
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1' stroke='black' />\n",
            bounds.origin.0,
            line_y,
            bounds.origin.0 + bounds.dimensions.0,
            line_y
        ).unwrap();
        line_y += LINE_SPACING;
    }
}

pub fn demo_1() {
    let stave_callback = layout::Callbacks {
        draw_background: draw_stave,
    };

    let small_margin = layout::LayoutSpec::none().with_margin(20.0, 0.0, 0.0, 0.0);

    let no_margin = layout::LayoutSpec::none();

    let big_margin = layout::LayoutSpec::none().with_margin(10.0, 50.0, 10.0, 50.0);

    let big_text = layout::TextSpec::new(30.0, layout::TextAnchor::Middle);
    let normal_text_l = layout::TextSpec::new(20.0, layout::TextAnchor::Start);
    let normal_text_m = layout::TextSpec::new(20.0, layout::TextAnchor::Middle);
    let normal_text_r = layout::TextSpec::new(20.0, layout::TextAnchor::End);
    let small_text_m = layout::TextSpec::new(15.0, layout::TextAnchor::Middle);

    let mut page = layout::Page::new(layout::Margin::new_uniform(20.0));

    let mut ttb = layout::Node::new_ttb(
        layout::LayoutSpec::none().with_margin(20.0, 0.0, 0.0, 0.0),
        layout::Callbacks::none(),
    );

    let mut ltr_1 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    ltr_1.append_child(layout::Node::Blank(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(200.0, 0.0),
        layout::Callbacks::none(),
    ));

    ltr_1.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        big_text,
        "My Cat's Got No Arms".to_string(),
    ));
    ltr_1.append_child(layout::Node::Blank(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(200.0, 0.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(ltr_1);

    let mut author = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    author.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "Old codswolloping song".to_string(),
    ));
    author.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_r,
        "Trad. Cornish".to_string(),
    ));
    ttb.append_child(author);

    let mut ltr_2 = layout::Node::new_ltr(small_margin, layout::Callbacks::none());

    let mut ltr_2_ttb_1 = layout::Node::new_ttb(
        layout::LayoutSpec::none().with_margin(20.0, 0.0, 0.0, 0.0),
        layout::Callbacks::none(),
    );

    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(10.0, 50.0, 10.0, 50.0)
            .with_dimensions(400.0, 20.0),
        layout::Callbacks::none(),
    ));

    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(10.0, 50.0, 10.0, 50.0)
            .with_dimensions(400.0, 20.0),
        layout::Callbacks::none(),
    ));

    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(10.0, 50.0, 10.0, 50.0)
            .with_dimensions(400.0, 20.0),
        layout::Callbacks::none(),
    ));

    ltr_2.append_child(ltr_2_ttb_1);

    let mut ltr_2_ttb_2 = layout::Node::new_ttb(
        layout::LayoutSpec::none().with_margin(20.0, 0.0, 0.0, 0.0),
        layout::Callbacks::none(),
    );

    ltr_2_ttb_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(10.0, 50.0, 10.0, 50.0)
            .with_dimensions(400.0, 40.0),
        layout::Callbacks::none(),
    ));

    ltr_2_ttb_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(10.0, 50.0, 10.0, 50.0)
            .with_dimensions(400.0, 40.0),
        layout::Callbacks::none(),
    ));

    ltr_2.append_child(ltr_2_ttb_2);
    ttb.append_child(ltr_2);

    let mut lr_blocks = layout::Node::new_ltr(small_margin, layout::Callbacks::none());
    let mut left_block = layout::Node::new_ttb(
        layout::LayoutSpec::none().with_margin(20.0, 0.0, 0.0, 0.0),
        layout::Callbacks::none(),
    );

    let mut stave_1 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(500.0, 0.0),
        layout::Callbacks::none(),
    );

    stave_1.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    left_block.append_child(stave_1);

    let mut stave_2 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(500.0, 0.0),
        layout::Callbacks::none(),
    );

    stave_2.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    stave_2.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    left_block.append_child(stave_2);

    // left_block.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut stave_3 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(500.0, 20.0),
        layout::Callbacks::none(),
    );

    stave_3.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    stave_3.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_3.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    left_block.append_child(stave_3);

    let mut stave_5 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(500.0, 0.0),
        layout::Callbacks::none(),
    );

    stave_5.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_5.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_5.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_5.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_5.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    left_block.append_child(stave_5);

    let mut stave_10 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(500.0, 0.0),
        layout::Callbacks::none(),
    );

    stave_10.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    stave_10.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    stave_10.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    stave_10.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10.append_child(layout::Node::Block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    left_block.append_child(stave_10);

    // left_block.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    lr_blocks.append_child(left_block);
    lr_blocks.append_child(layout::Node::Blank(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 0.0),
        layout::Callbacks::none(),
    ));
    lr_blocks.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(480.0, 180.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(lr_blocks);

    let mut stave_100 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 20.0),
        layout::Callbacks::none(),
    ));

    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 40.0),
        layout::Callbacks::none(),
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 60.0),
        layout::Callbacks::none(),
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 80.0),
        layout::Callbacks::none(),
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 80.0),
        layout::Callbacks::none(),
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 80.0),
        layout::Callbacks::none(),
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 80.0),
        layout::Callbacks::none(),
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 60.0),
        layout::Callbacks::none(),
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 40.0),
        layout::Callbacks::none(),
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(100.0, 20.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(stave_100);

    let mut stave_10_uneven_1 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(50.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(40.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(30.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(10.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(5.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(stave_10_uneven_1);

    let mut stave_10_uneven_2 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(80.0, 20.0),
        layout::Callbacks::none(),
    ));

    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(50.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(70.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(50.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(50.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(50.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(40.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(50.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(35.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(50.0, 20.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(stave_10_uneven_2);

    let mut stave_very_uneven = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(50.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(80.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(500.0, 20.0),
        layout::Callbacks::none(),
    ));
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(250.0, 20.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(stave_very_uneven);

    //
    // Stave 1
    //

    let mut notes_stave_layout = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 00.0, 20.0, 00.0)
            .with_dimensions(1000.0, 0.0),
        stave_callback,
    );

    let notehead_layout = layout::LayoutSpec::none()
        .with_dimensions(20.0, 10.0)
        .with_margin(0.0, 10.0, 0.0, 10.0);

    let barline_layout = layout::LayoutSpec::none().with_dimensions(1.0, STAVE_HEIGHT).with_offset(0.0, -STAVE_HEIGHT / 2.0);

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 0.0-PLACE_SPACING),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*3.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_block(
        notehead_layout
            .with_offset(0.0, PLACE_SPACING*3.0)
            .with_margin(0.0, 20.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 0.0-PLACE_SPACING),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*3.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout
            .with_offset(0.0, PLACE_SPACING*3.0)
            .with_margin(0.0, 20.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout
            .with_offset(0.0, PLACE_SPACING*2.0)
            .with_margin(0.0, 20.0, 0.0, 15.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*1.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*1.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*2.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*1.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_block(
        notehead_layout
            .with_offset(0.0,  PLACE_SPACING * 0.0)
            .with_margin(0.0, 40.0, 0.0, 40.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 0.0-PLACE_SPACING),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*3.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_block(
        notehead_layout
            .with_offset(0.0, 15.0)
            .with_margin(0.0, 20.0, 0.0, PLACE_SPACING*3.0),
        layout::Callbacks::none(),
    ));

    // I got bored working this out in terms of note head multiples here. It's only a demo.

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, -5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 15.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout
            .with_offset(0.0, 15.0)
            .with_margin(0.0, 20.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout
            .with_offset(0.0, 10.0)
            .with_margin(0.0, 20.0, 0.0, 15.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_block(
        notehead_layout
            .with_offset(0.0, 0.0)
            .with_margin(0.0, 10.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout.append_child(layout::Node::new_block(
        barline_layout
            .with_margin(0.0, 0.0, 0.0, 5.0),
        layout::Callbacks::none(),
    ));

    let stave_again = notes_stave_layout.clone();
    let stave_again_again = notes_stave_layout.clone();

    ttb.append_child(notes_stave_layout);

    // Stave 2 has ledger lines (or will have).
    let mut notes_stave_layout_2 = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 00.0, 20.0, 00.0)
            .with_dimensions(1000.0, 0.0),
        stave_callback,
    );

    let notehead_layout = layout::LayoutSpec::none()
        .with_dimensions(20.0, 10.0)
        .with_margin(0.0, 10.0, 0.0, 10.0);

    let barline_layout = layout::LayoutSpec::none()
        .with_dimensions(1.0, 40.0)
        .with_offset(0.0, 0.0 - LINES_EITHER_SIDE_MIDDLE as f32 * LINE_SPACING);

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, -5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 15.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_block(
        notehead_layout
            .with_offset(0.0, 15.0)
            .with_margin(0.0, 20.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 0.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 15.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout
            .with_offset(0.0, 15.0)
            .with_margin(0.0, 20.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout
            .with_offset(0.0, 10.0)
            .with_margin(0.0, 20.0, 0.0, 15.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_block(
        notehead_layout
            .with_offset(0.0, 0.0)
            .with_margin(0.0, 40.0, 0.0, 40.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, -5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*-9.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_block(
        notehead_layout
            .with_offset(0.0, 15.0)
            .with_margin(0.0, 20.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, -5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 15.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout
            .with_offset(0.0, 15.0)
            .with_margin(0.0, 20.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout
            .with_offset(0.0, 10.0)
            .with_margin(0.0, 20.0, 0.0, 15.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 5.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        notehead_layout.with_offset(0.0, PLACE_SPACING*9.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_solid_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_block(
        notehead_layout
            .with_offset(0.0, 0.0)
            .with_margin(0.0, 10.0, 0.0, 10.0),
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_block(
        barline_layout,
        layout::Callbacks::none(),
    ));

    notes_stave_layout_2.append_child(layout::Node::new_block(
        barline_layout.with_margin(0.0, 0.0, 0.0, 5.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(notes_stave_layout_2);

    //
    // Stave 3 and 4
    //

    // Now add copies of the first stave.
    ttb.append_child(stave_again);
    ttb.append_child(stave_again_again);

    ttb.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 50.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(layout::Node::Blank(
        layout::LayoutSpec::none().with_dimensions(0.0, 20.0),
        layout::Callbacks::none(),
    ));
    ttb.append_child(layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 50.0),
        layout::Callbacks::none(),
    ));

    let mut compound_stave = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1.0, 50.0),
        layout::Callbacks::none(),
    ));

    let mut compound_stave_sub_1 = layout::Node::new_ltr(small_margin, layout::Callbacks::none());
    compound_stave_sub_1.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_1.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 30.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_1.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 40.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_1.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));
    compound_stave.append_child(compound_stave_sub_1);

    compound_stave.append_child(layout::Node::new_solid_block(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1.0, 50.0),
        layout::Callbacks::none(),
    ));

    let mut compound_stave_sub_2 = layout::Node::new_ltr(small_margin, layout::Callbacks::none());
    compound_stave_sub_2.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 30.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_2.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 40.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_2.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 50.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_2.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(20.0, 60.0),
        layout::Callbacks::none(),
    ));
    compound_stave.append_child(compound_stave_sub_2);

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1.0, 50.0),
        layout::Callbacks::none(),
    ));

    let mut compound_stave_sub_3 = layout::Node::new_ltr(small_margin, layout::Callbacks::none());
    compound_stave_sub_3.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 60.0),
        layout::Callbacks::none(),
    ));

    compound_stave_sub_3.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 30.0),
        layout::Callbacks::none(),
    ));

    compound_stave_sub_3.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 20.0),
        layout::Callbacks::none(),
    ));

    compound_stave_sub_3.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 40.0),
        layout::Callbacks::none(),
    ));

    compound_stave.append_child(compound_stave_sub_3);

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1.0, 50.0),
        layout::Callbacks::none(),
    ));

    let mut compound_stave_sub_4 = layout::Node::new_ltr(small_margin, layout::Callbacks::none());
    compound_stave_sub_4.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 50.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_4.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 40.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_4.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(20.0, 30.0),
        layout::Callbacks::none(),
    ));
    compound_stave_sub_4.append_child(layout::Node::new_placeholder_frame(
        layout::LayoutSpec::none().with_dimensions(40.0, 10.0),
        layout::Callbacks::none(),
    ));
    compound_stave.append_child(compound_stave_sub_4);

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1.0, 50.0),
        layout::Callbacks::none(),
    ));

    layout::Node::PlaceholderFrame(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(400.0, 20.0),
        layout::Callbacks::none(),
    );
    ttb.append_child(compound_stave);

    ttb.append_child(layout::Node::Blank(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(200.0, 0.0),
        layout::Callbacks::none(),
    ));

    let mut words = layout::Node::new_ttb(
        layout::LayoutSpec::none().with_margin(20.0, 0.0, 0.0, 200.0),
        layout::Callbacks::none(),
    );

    words.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "My cat's he's got no arms or legs".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "He likes to climb up stairs ".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "I've never seen him doing it".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "I suspect he's never actually done it".to_string(),
    ));

    words.append_child(layout::Node::Blank(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(200.0, 0.0),
        layout::Callbacks::none(),
    ));

    words.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "My cat's a liar and quite possibly an idiot".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "But I do like him".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "Because he brings me mice".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "And socks when I leave them on the ground floor".to_string(),
    ));

    ttb.append_child(words);

    let mut footnote = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    footnote.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_l,
        "Cat Songs vol XVII".to_string(),
    ));
    footnote.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        small_text_m,
        "32".to_string(),
    ));
    footnote.append_child(layout::Node::new_text(
        no_margin,
        layout::Callbacks::none(),
        normal_text_r,
        "Mid to medium-sized villages Northwest Cornwall and beyond".to_string(),
    ));
    ttb.append_child(footnote);

    page.set_root(ttb);

    // Write normally.
    eprintln!("Write normal...");
    let mut string_buffer = String::new();
    let config = layout::Config {
        draw_bounding_box: false,
    };
    page.write(&config, &mut string_buffer);
    let mut f = File::create("demo.svg").expect("Unable to create file");
    f.write_all(string_buffer.as_bytes())
        .expect("Unable to write data");

    // And write with bounding boxes for debugging.
    eprintln!("Write debug...");
    let mut string_buffer = String::new();

    let config = layout::Config {
        draw_bounding_box: true,
    };
    page.write(&config, &mut string_buffer);

    let mut f = File::create("demo_debug.svg").expect("Unable to create file");
    f.write_all(string_buffer.as_bytes())
        .expect("Unable to write data");
}