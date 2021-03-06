use layout;

/// Position on a stave relative to the centre line.
type StavePosition = i16;

// Spacing between lines on the stave. A notehead is also this height.
const LINE_SPACING: f32 = 10.0;

// Height of a position on the stave.
const PLACE_SPACING: f32 = LINE_SPACING / 2.0;

// Number of lines on a stave.
const NUM_LINES: StavePosition = 5;

// Number of lines above and below the stave
const LINES_EITHER_SIDE_MIDDLE: StavePosition = (NUM_LINES - 1) / 2;

// Line to line.
const STAVE_HEIGHT: f32 = (NUM_LINES - 1) as f32 * LINE_SPACING;

const DOUBLE_BARLINE_SPACING: f32 = 4.0;

pub enum Chunk {
    Title(String),
    Stave(Vec<StaveEntity>),
}

pub enum ClefSymbol {
    Treble,
    Bass,
    Tenor,
}

pub enum NoteHeadSymbol {
    Filled,
    Empty,
    // Cross
}

pub enum BarlineSymbol {
    Single,
    Double,
    // End,
    // RepeatBefore,
    // RepeatAfter,
    // RepeatBeforeAfter,
}

pub enum StaveEntity {
    // Clef(StavePosition, ClefSymbol),
    Note(StavePosition, NoteHeadSymbol),
    Barline(BarlineSymbol),
    // Rest(StavePosition),
}

/// Node background callback
mod callbacks {
    use layout;
    use std::fmt::Write as _fw;

    /// Draw a stave as the background to a LTRJustify Node.
    pub fn draw_stave(buf: &mut String, bounds: layout::BoundsWithOffset) -> () {
        let mut line_y = bounds.origin.1 - (super::STAVE_HEIGHT / 2.0);

        // We'll be given the origin and bounds, but it will be with relation to the middle line.
        // Therefore offset everything up by half a stave.

        for line in 0..super::NUM_LINES {
            write!(
                buf,
                "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1.5' stroke='black' />\n",
                bounds.origin.0,
                line_y,
                bounds.origin.0 + bounds.dimensions.0,
                line_y
            ).unwrap();
            line_y += super::LINE_SPACING;
        }
    }

    pub fn draw_barline(buf: &mut String, bounds: layout::BoundsWithOffset) -> () {
        write!(
            buf,
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1.5' stroke='black' />\n",
            bounds.origin.0,
            bounds.origin.1,
            bounds.origin.0,
            bounds.origin.1 + bounds.dimensions.1,
        ).unwrap();
    }

    pub fn draw_double_barline(buf: &mut String, bounds: layout::BoundsWithOffset) -> () {
        write!(
            buf,
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1.5' stroke='black' />\n",
            bounds.origin.0,
            bounds.origin.1,
            bounds.origin.0,
            bounds.origin.1 + bounds.dimensions.1,
        ).unwrap();

        write!(
            buf,
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1.5' stroke='black' />\n",
            bounds.origin.0 + super::DOUBLE_BARLINE_SPACING,
            bounds.origin.1,
            bounds.origin.0 + super::DOUBLE_BARLINE_SPACING,
            bounds.origin.1 + bounds.dimensions.1,
        ).unwrap();
    }
}

pub fn render_title(title: &String) -> layout::Node {
    let big_text = layout::TextSpec::new(30.0, layout::TextAnchor::Middle);

    let mut ltr = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(1000.0, 0.0),
        layout::Callbacks::none(),
    );

    ltr.append_child(layout::Node::Blank(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(200.0, 0.0),
        layout::Callbacks::none(),
    ));

    ltr.append_child(layout::Node::new_text(
        layout::LayoutSpec::none(),
        layout::Callbacks::none(),
        big_text,
        title.to_string(),
    ));
    ltr.append_child(layout::Node::Blank(
        layout::LayoutSpec::none()
            .with_margin(20.0, 0.0, 0.0, 0.0)
            .with_dimensions(200.0, 0.0),
        layout::Callbacks::none(),
    ));

    ltr
}

pub fn render_stave(elems: &Vec<StaveEntity>) -> layout::Node {
    let stave_callback = layout::Callbacks {
        draw_background: callbacks::draw_stave,
    };

    let barline_callback = layout::Callbacks {
        draw_background: callbacks::draw_barline,
    };

    let double_barline_callback = layout::Callbacks {
        draw_background: callbacks::draw_double_barline,
    };

    let notehead_style = layout::LayoutSpec::none()
        .with_dimensions(LINE_SPACING * 1.75, LINE_SPACING)
        .with_margin(0.0, 10.0, 0.0, 10.0);

    let barline_style = layout::LayoutSpec::none()
        .with_dimensions(1.0, STAVE_HEIGHT)
        .with_offset(0.0, -STAVE_HEIGHT / 2.0);

    let double_barline_style = layout::LayoutSpec::none()
        .with_dimensions(DOUBLE_BARLINE_SPACING, STAVE_HEIGHT)
        .with_offset(0.0, -STAVE_HEIGHT / 2.0);

    let mut notes_stave_layout = layout::Node::new_ltr_justify(
        layout::LayoutSpec::none()
            .with_margin(20.0, 00.0, 20.0, 00.0)
            .with_dimensions(1000.0, 0.0),
        stave_callback,
    );

    for elem in elems {
        match elem {
            StaveEntity::Note(position, symbol) => match symbol {
                NoteHeadSymbol::Empty => {
                    // Upper edge of notehead.
                    let notehead_y = -PLACE_SPACING * (*position as f32) - PLACE_SPACING;

                    notes_stave_layout.append_child(layout::Node::new_block(
                        notehead_style.with_offset(0.0, notehead_y),
                        layout::Callbacks::none(),
                    ));
                }
                NoteHeadSymbol::Filled => {
                    // Upper edge of notehead.
                    let notehead_y = -PLACE_SPACING * (*position as f32) - PLACE_SPACING;

                    notes_stave_layout.append_child(layout::Node::new_solid_block(
                        notehead_style.with_offset(0.0, notehead_y),
                        layout::Callbacks::none(),
                    ));
                }
            },
            StaveEntity::Barline(symbol) => match symbol {
                BarlineSymbol::Single => {
                    notes_stave_layout
                        .append_child(layout::Node::new_blank(barline_style, barline_callback));
                }

                BarlineSymbol::Double => {
                    notes_stave_layout.append_child(layout::Node::new_blank(
                        double_barline_style,
                        double_barline_callback,
                    ));
                }
            },
        }
    }

    notes_stave_layout
}

pub fn typeset(chunks: &Vec<Chunk>) -> layout::Page {
    let mut page = layout::Page::new(layout::Margin::new_uniform(20.0));

    let mut ttb = layout::Node::new_ttb(
        layout::LayoutSpec::none().with_margin(20.0, 0.0, 0.0, 0.0),
        layout::Callbacks::none(),
    );

    for ref chunk in chunks {
        match chunk {
            Chunk::Title(ref title) => {
                ttb.append_child(render_title(title));
            }
            Chunk::Stave(ref elems) => {
                ttb.append_child(render_stave(elems));
            }
        }
    }

    page.set_root(ttb);

    page
}
