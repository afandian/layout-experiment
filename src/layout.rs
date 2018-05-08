//! Layout experiment


use std::fmt::Write;

use std;

// Average character width as multiple of font height px.
const TEXT_CHAR_RATIO: f32 = 0.5;

#[derive(Clone, Debug)]
pub struct Config {
    pub draw_bounding_box: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Point(pub f32, pub f32);

#[derive(Clone, Copy, Debug)]
pub struct Dimensions(pub f32, pub f32);

impl Dimensions {
    pub fn plus_margin(&self, margin: Margin) -> Dimensions {
        Dimensions(
            self.0 + margin.total_width(),
            self.1 + margin.total_height(),
        )
    }

    pub fn none() -> Dimensions {
        Dimensions(0.0, 0.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub top_left: Point,
    pub dimensions: Dimensions,
}

/// Bounds, but the origin is offset.
#[derive(Clone, Copy, Debug)]
pub struct BoundsWithOffset {
    pub top_left: Point,
    pub dimensions: Dimensions,
    pub origin: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Margin {
    pub n: f32,
    pub e: f32,
    pub s: f32,
    pub w: f32,
}

impl Margin {
    pub fn new_uniform(margin: f32) -> Margin {
        Margin {
            n: margin,
            e: margin,
            s: margin,
            w: margin,
        }
    }

    pub fn new(n: f32, e: f32, s: f32, w: f32) -> Margin {
        Margin { n, e, s, w }
    }

    pub fn none() -> Margin {
        Margin {
            n: 0.0,
            e: 0.0,
            s: 0.0,
            w: 0.0,
        }
    }

    pub fn total_width(&self) -> f32 {
        self.w + self.e
    }

    pub fn total_height(&self) -> f32 {
        self.n + self.s
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

#[derive(Debug, Clone, Copy)]
pub struct TextSpec {
    // Vertical text size in px.
    pub size: f32,

    // Anchor within the container.
    // As the container tries its best to be the appropriate width, this is only a hint.
    pub anchor_hint: TextAnchor,
}

impl TextSpec {
    pub fn new(size: f32, anchor_hint: TextAnchor) -> TextSpec {
        TextSpec { size, anchor_hint }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutSpec {
    pub dimensions: Dimensions,
    pub margin: Margin,
    /// Offset has different behaviour depending on the container.
    /// In LTR and LTRJustify:
    /// - x-offset skews the node left or right from its place without affecting anything else
    /// - y-offset moves the node up or down. It can be negative, in which case the container
    /// increases its height to accommodate all nodes, including margin. This means that a vertical
    /// origin of zero can be used for the centre line. This is very handy for plotting on a stave,
    /// middle line is zero, and letting ledger-lines be automatically accommodated.
    pub offset: Point,
}

impl LayoutSpec {
    pub fn none() -> LayoutSpec {
        LayoutSpec {
            dimensions: Dimensions::none(),
            margin: Margin::none(),
            offset: Point(0.0, 0.0),
        }
    }

    pub fn with_dimensions(&self, x: f32, y: f32) -> LayoutSpec {
        LayoutSpec {
            dimensions: Dimensions(x, y),
            ..*self
        }
    }

    pub fn with_margin(&self, n: f32, e: f32, s: f32, w: f32) -> LayoutSpec {
        LayoutSpec {
            margin: Margin { n, e, s, w },
            ..*self
        }
    }

    pub fn with_offset(&self, x: f32, y: f32) -> LayoutSpec {
        LayoutSpec {
            offset: Point(x, y),
            ..*self
        }
    }
}

fn no_callback(buf: &mut String, bounds: BoundsWithOffset) -> () {}

#[derive(Clone, Copy)]
pub struct Callbacks {
    /// Draw the background. The origin can be offset, e.g. LTRJustify
    pub draw_background: fn(buf: &mut String, bounds: BoundsWithOffset) -> (),
}

impl Callbacks {
    pub fn none() -> Callbacks {
        Callbacks {
            draw_background: no_callback,
        }
    }
}

impl std::fmt::Debug for Callbacks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // Nothing to do.
        // There's no useful info to be had here.
        Result::Ok(())
    }
}

/// A node in the tree.
/// Polymorphism represented as an enum rather than a traits, as that
/// would require a messy proliferation of boxed trait objects and lifetimes.
/// This does constrain the extensibility, but this is intended to be embedded directly in the
/// music typsetting code.
#[derive(Debug, Clone)]
pub enum Node {
    Blank(LayoutSpec, Callbacks),
    Block(LayoutSpec, Callbacks),
    SolidBlock(LayoutSpec, Callbacks),
    PlaceholderFrame(LayoutSpec, Callbacks),
    /// Left-to-right layout.
    /// Takes dimensions from children.
    LTR(LayoutSpec, Callbacks, Vec<Node>),
    /// Top-to-bottom layout.
    /// Takes dimensions from children.
    TTB(LayoutSpec, Callbacks, Vec<Node>),
    /// Left-to-right layout, justified to supplied width.
    /// Height is derived from content (value in LayoutSpec is ignored).
    LTRJustify(LayoutSpec, Callbacks, Vec<Node>),
    /// Text of given font size.
    Text(LayoutSpec, Callbacks, TextSpec, String),
}

/// Given a collection of nodes, find the numerically lowest and highest
/// points that a node touches, including its margin and offset.
fn get_y_bounds(nodes: &Vec<Node>) -> (f32, f32) {
    let lower = nodes
        .iter()
        .map(|x| x.get_offset().1 - x.get_margin().n)
        .fold(0.0, f32::min);
    let upper = nodes
        .iter()
        .map(|x| x.get_offset().1 + x.get_outer_dimensions().1)
        .fold(0.0, f32::max);

    (lower, upper)
}

// Node constructors
impl Node {
    /// Get dimensions including margin. Ignore offset.
    fn get_outer_dimensions(&self) -> Dimensions {
        self.get_dimensions().plus_margin(self.get_margin())
    }

    pub fn new_placeholder_frame(layout: LayoutSpec, callbacks: Callbacks) -> Node {
        Node::PlaceholderFrame(layout, callbacks)
    }

    pub fn new_ltr(layout: LayoutSpec, callbacks: Callbacks) -> Node {
        Node::LTR(layout, callbacks, vec![])
    }

    pub fn new_blank(layout: LayoutSpec, callbacks: Callbacks) -> Node {
        Node::Blank(layout, callbacks)
    }

    pub fn new_ltr_justify(layout: LayoutSpec, callbacks: Callbacks) -> Node {
        Node::LTRJustify(layout, callbacks, vec![])
    }

    pub fn new_ttb(layout: LayoutSpec, callbacks: Callbacks) -> Node {
        Node::TTB(layout, callbacks, vec![])
    }

    pub fn new_text(
        layout: LayoutSpec,
        callbacks: Callbacks,
        text_spec: TextSpec,
        text: String,
    ) -> Node {
        Node::Text(layout, callbacks, text_spec, text)
    }

    pub fn new_solid_block(layout: LayoutSpec, callbacks: Callbacks) -> Node {
        Node::SolidBlock(layout, callbacks)
    }

    pub fn new_block(layout: LayoutSpec, callbacks: Callbacks) -> Node {
        Node::Block(layout, callbacks)
    }
}

// Accessors
impl Node {
    // Get intrinsic dimensions of this Node (i.e. exclusing margin).
    // These are sometimes derived dynamically, so different from layout.dimensions
    fn get_dimensions(&self) -> Dimensions {
        match self {
            &Node::Blank(layout, _)
            | &Node::PlaceholderFrame(layout, _)
            | &Node::Block(layout, _)
            | &Node::SolidBlock(layout, _) => layout.dimensions,

            &Node::LTR(ref layout, _, ref items) => {
                // Accommodate the full y-range of items' offsets.
                // We'll need to fit them all in.
                let (lower, upper) = get_y_bounds(items);
                let height = upper - lower;

                // In left-to-right, widths add up. Heights take max.
                let width = items.iter().map(|x| x.get_outer_dimensions().0).sum();

                Dimensions(width, height)
            }

            &Node::TTB(ref layout, _, ref items) => {
                // In top-to-bottom, heights add up. Widths take max.
                let height = items.iter().map(|x| x.get_outer_dimensions().1).sum();
                let width = items
                    .iter()
                    .map(|x| x.get_outer_dimensions().0)
                    .fold(0.0, f32::max);

                Dimensions(width, height)
            }

            &Node::LTRJustify(ref layout, _, ref items) => {
                // Accommodate the full y-range of items' offsets.
                // We'll need to fit them all in.
                let (lower, upper) = get_y_bounds(items);
                let height = upper - lower;

                Dimensions(layout.dimensions.0, height)
            }

            &Node::Text(_, _, ref text_spec, ref text) => {
                // For now, derive width from a heuristic. Hopefully this will work out, on average.
                Dimensions(
                    text_spec.size * TEXT_CHAR_RATIO * text.len() as f32,
                    text_spec.size,
                )
            }
        }
    }

    // Get the layout spec of this node, or a blank one.
    fn get_layout(&self) -> LayoutSpec {
        match self {
            &Node::Blank(layout, _)
            | &Node::Block(layout, _)
            | &Node::SolidBlock(layout, _)
            | &Node::PlaceholderFrame(layout, _)
            | &Node::LTR(layout, _, _)
            | &Node::TTB(layout, _, _)
            | &Node::LTRJustify(layout, _, _)
            | &Node::Text(layout, _, _, _) => layout,
        }
    }

    /// Get the margin, zero margin if there isn't one.
    fn get_margin(&self) -> Margin {
        self.get_layout().margin
    }

    /// Get the margin, zero margin if there isn't one.
    fn get_offset(&self) -> Point {
        self.get_layout().offset
    }

    fn get_callbacks(&self) -> Callbacks {
        match self {
            &Node::Blank(_, callbacks)
            | &Node::Block(_, callbacks)
            | &Node::SolidBlock(_, callbacks)
            | &Node::PlaceholderFrame(_, callbacks)
            | &Node::LTR(_, callbacks, _)
            | &Node::TTB(_, callbacks, _)
            | &Node::LTRJustify(_, callbacks, _)
            | &Node::Text(_, callbacks, _, _) => callbacks,
        }
    }
}

impl Node {
    // Push a child.
    // If this is the wrong kind of node, just ignore.
    // TODO not perfect!
    pub fn append_child(&mut self, child: Node) {
        match self {
            &mut Node::LTR(_, _, ref mut items)
            | &mut Node::TTB(_, _, ref mut items)
            | &mut Node::LTRJustify(_, _, ref mut items) => {
                items.push(child);
            }

            _ => (),
        }
    }

    // Draw this item at this origin.
    // This will be inside whatever margin the node asked for, as respecting that bit of layout is the responsibility of the parent.
    pub fn draw(&self, config: &Config, buf: &mut String, origin: Point) {
        // When these functions retrieve self dimensions, it's done via the
        // self.dimensions call (which is polymorphic) rather than directly from the struct.
        // so we're working to the same interface as the parent laying this out.
        let dimensions = self.get_dimensions();

        let callbacks = self.get_callbacks();

        // The unmatched fields are handled in a uniform fashion by above self getters.
        match self {
            &Node::Blank(_, _) => (),

            &Node::Block(_, _) => {
                write!(
                    buf,
                    "<rect x='{}' y='{}' width='{}' height='{}' \
                     style='fill:none;stroke:black;stroke-width:1' />\n",
                    origin.0, origin.1, dimensions.0, dimensions.1
                ).unwrap();
            }

            &Node::SolidBlock(_, _) => {
                write!(
                    buf,
                    "<rect x='{}' y='{}' width='{}' height='{}' \
                     style='fill:#333 solid;stroke:black;stroke-width:1' />\n",
                    origin.0, origin.1, dimensions.0, dimensions.1
                ).unwrap();
            }

            &Node::PlaceholderFrame(_, _) => {
                write!(
                    buf,
                    "<rect x='{}' y='{}' width='{}' height='{}' \
                      style='fill:none;stroke:black;stroke-width:1' />\n
                     <line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1' stroke='black' />\n
                     <line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1' stroke='black' />\n",
                    origin.0,
                    origin.1,
                    dimensions.0,
                    dimensions.1,
                    origin.0,
                    origin.1,
                    origin.0 + dimensions.0,
                    origin.1 + dimensions.1,
                    origin.0 + dimensions.0,
                    origin.1,
                    origin.0,
                    origin.1 + dimensions.1
                ).unwrap();
            }

            &Node::LTR(_, _, ref children) => {
                let mut x = origin.0;
                let mut y = origin.1;

                // Offset all children's potentially negative offsets.
                let (lower, _) = get_y_bounds(children);
                y -= lower;

                for ref node in children.iter() {
                    node.draw(
                        config,
                        buf,
                        Point(
                            x + node.get_margin().w + node.get_offset().0,
                            y + node.get_margin().n + node.get_offset().1,
                        ),
                    );
                    x += node.get_outer_dimensions().0;
                }

                if config.draw_bounding_box {
                    // y-origin line
                    write!(
                        buf,
                        "<line x1='{}' y1='{}' x2='{}' y2='{}' \
                         class='debug debug-node' stroke-dashoffset='0' stroke-dasharray='4,4' style='stroke:red;stroke-style:dashed;stroke-width:1'   />\n",
                        origin.0, y, origin.0 + dimensions.0, y+1.0
                    ).unwrap();
                }
            }

            &Node::TTB(_, _, ref children) => {
                let mut x = origin.0;
                let mut y = origin.1;

                for ref node in children.iter() {
                    node.draw(
                        config,
                        buf,
                        Point(x + node.get_margin().w, y + node.get_margin().n),
                    );
                    y += node.get_outer_dimensions().1;
                }
            }

            &Node::LTRJustify(ref layout, _, ref children) => {
                let mut x = origin.0;
                let mut y = origin.1;

                // Offset all children's potentially negative offsets.
                let (lower, _) = get_y_bounds(children);
                y -= lower;

                // Supply the bounding record and also the adjusted y-offseted origin.
                (callbacks.draw_background)(
                    buf,
                    BoundsWithOffset {
                        top_left: origin,
                        origin: Point(x, y),
                        dimensions,
                    },
                );

                if children.len() == 0 {
                    return;
                }

                // Special case for a single child, just draw it on left-hand side.
                if children.len() == 1 {
                    let child = children.first().unwrap();
                    child.draw(
                        config,
                        buf,
                        Point(
                            x + child.get_margin().w + child.get_offset().0,
                            y + child.get_margin().n + child.get_offset().1,
                        )
                        // Point(x + child.get_margin().w, y + child.get_margin().n),
                    );
                    return;
                }

                // Otherwise we have a left and right item which should be anchored left and right,
                // plus any number of others which should be distributed between them.

                let first = children.first().unwrap();
                let last = children.last().unwrap();

                // First should be left-aligned, last should be right-aligned.
                first.draw(
                    config,
                    buf,
                    // Point(x + first.get_margin().w, y + first.get_margin().n),
                    Point(
                        x + first.get_margin().w + first.get_offset().0,
                        y + first.get_margin().n + first.get_offset().1,
                    ),
                );

                // And the middle ones should be distributed in the remaining width.
                // Distribution should be done by dividing up available (possibly negative) whitespace
                // not by the centre of each object.

                let remaining_width_in_container = layout.dimensions.0
                    - (last.get_outer_dimensions().0 + first.get_outer_dimensions().0);

                let middle_children = &children[1..children.len() - 1];

                // The width of the content we need to lay out.
                let middle_content_width: f32 = middle_children
                    .iter()
                    .map(|x| x.get_outer_dimensions().0)
                    .sum();

                // Number of padding whitespaces needed, and how wide they are.
                let num_whitespaces = middle_children.len() as f32 + 1.0;

                let remaining_whitespace = remaining_width_in_container - middle_content_width;
                let space_between_items = remaining_whitespace / num_whitespaces;

                // Shuffle up so we start after the first one.
                x += first.get_outer_dimensions().0;

                for ref node in middle_children.iter() {
                    x += space_between_items;
                    node.draw(
                        config,
                        buf,
                        Point(
                            x + node.get_margin().w + node.get_offset().0,
                            y + node.get_margin().n + node.get_offset().1,
                        ),
                    );
                    x += node.get_outer_dimensions().0;
                }

                x += space_between_items;
                last.draw(
                    config,
                    buf,
                    Point(
                        x + last.get_margin().w + last.get_offset().0,
                        y + last.get_margin().n + last.get_offset().1,
                    ),
                );

                if config.draw_bounding_box {
                    // y-origin line.
                    write!(
                        buf,
                        "<line x1='{}' y1='{}' x2='{}' y2='{}' \
                         class='debug debug-node' stroke-dashoffset='0' stroke-dasharray='4,4' style='stroke:red;stroke-style:dashed;stroke-width:1;fill:none'   />\n",
                        origin.0, y, origin.0 + dimensions.0, y+1.0
                    ).unwrap();
                }
            }

            &Node::Text(_, _, text_spec, ref text) => {
                // x means different things depending on anchor. Offset for correct behaviour.
                let x = origin.0 + match text_spec.anchor_hint {
                    TextAnchor::Start => 0.0,
                    TextAnchor::Middle => dimensions.0 / 2.0,
                    TextAnchor::End => dimensions.0,
                };

                let text_anchor = match text_spec.anchor_hint {
                    TextAnchor::Start => "start",
                    TextAnchor::Middle => "middle",
                    TextAnchor::End => "end",
                };

                // TODO escape text.
                write!(
                    buf,
                    "<text x='{}' y='{}' width='{}' height='{}' font-size='{}' text-anchor='{}' >\n{}\n</text>",
                    x, origin.1 + dimensions.1,
                    dimensions.0, dimensions.1,
                    text_spec.size,
                    text_anchor,
                    text
                ).unwrap();
            }
        }

        if config.draw_bounding_box {
            // Bounding box of content.
            write!(
                buf,
                "<rect x='{}' y='{}' width='{}' height='{}' \
                 class='debug debug-node' stroke-dashoffset='0' stroke-dasharray='4,4' style='fill:none;stroke:blue;stroke-style:dashed;stroke-width:1'   />\n",
                origin.0, origin.1, dimensions.0, dimensions.1
            ).unwrap();

            // Bounding box including outer margin.
            write!(
                buf,
                "<rect x='{}' y='{}' width='{}' height='{}' \
                 class='debug debug-node' stroke-dashoffset='4' stroke-dasharray='4,4' style='fill:none;stroke:green;stroke-width:1'   />\n",
                origin.0 - self.get_margin().w, origin.1 - self.get_margin().n, dimensions.0 + self.get_margin().w + self.get_margin().e, dimensions.1 + self.get_margin().n + self.get_margin().s
            ).unwrap();
        }
    }
}

///! Left-to-right container.
///! Height depends on contents. Width depends on contents / container.
pub struct Page {
    root: Node,
    margin: Margin,
}

impl Page {
    pub fn new(margin: Margin) -> Page {
        Page {
            margin,
            // Start with nothing. An appropriate root note will get swapped in.
            root: Node::Blank(
                LayoutSpec {
                    margin: Margin::none(),
                    dimensions: Dimensions::none(),
                    offset: Point(0.0, 0.0),
                },
                Callbacks::none(),
            ),
        }
    }

    pub fn set_root(&mut self, root: Node) {
        self.root = root;
    }

    pub fn write(&self, config: &Config, buf: &mut String) {
        let dimensions = self.root.get_outer_dimensions().plus_margin(self.margin);

        write!(
            buf,
            "<svg version='1.1' baseProfile='full' width='{}' height='{}' \
             xmlns='http://www.w3.org/2000/svg'>\n",
            dimensions.0, dimensions.1
        ).unwrap();

        self.root.draw(
            config,
            buf,
            Point(
                self.margin.w + self.root.get_margin().w,
                self.margin.n + self.root.get_margin().n,
            ),
        );

        if config.draw_bounding_box {
            write!(
                buf,
                "<rect x='{}' y='{}' width='{}' height='{}' \
                 class='debug debug-page'  style='fill:none;stroke:blue;stroke-width:1'  />\n",
                0, 0, dimensions.0, dimensions.1
            ).unwrap();
        }

        write!(buf, "</svg>").unwrap();
    }
}
