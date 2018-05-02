//! Layout experiment

use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct Config {
    pub draw_bounding_box: bool,
}

#[derive(Clone, Debug)]
pub struct Point(f32, f32);

#[derive(Clone, Copy, Debug)]
pub struct Dimensions(pub f32, pub f32);

#[derive(Clone, Debug)]
pub struct Bounds {
    origin: Point,
    dimensions: Dimensions,
}

#[derive(Debug, Clone)]
pub enum Node {
    Blank(Dimensions),
    Block(Dimensions),
    SolidBlock(Dimensions),
    PlaceholderFrame(Dimensions),
    LTR(Vec<Node>),
    TTB(Vec<Node>),
}

impl Node {
    fn get_dimensions(&self) -> Dimensions {
        match self {
            &Node::Blank(dimensions)
            | &Node::PlaceholderFrame(dimensions)
            | &Node::Block(dimensions)
            | &Node::SolidBlock(dimensions) => dimensions,

            &Node::LTR(ref items) => {
                // In left-to-right, widths add up. Heights take max.
                let width = items.iter().map(|x| x.get_dimensions().0).sum();
                let height = items
                    .iter()
                    .map(|x| x.get_dimensions().1)
                    .fold(0.0, f32::max);

                Dimensions(width, height)
            }
            &Node::TTB(ref items) => {
                // In top-to-bottom, heights add up. Widths take max.
                let height = items.iter().map(|x| x.get_dimensions().1).sum();
                let width = items
                    .iter()
                    .map(|x| x.get_dimensions().0)
                    .fold(0.0, f32::max);

                Dimensions(width, height)
            }
        }
    }

    pub fn new_ltr() -> Node {
        Node::LTR(vec![])
    }

    pub fn new_ttb() -> Node {
        Node::TTB(vec![])
    }

    // Push a child.
    // If this is the wrong kind of node, just ignore.
    // TODO not perfect!
    pub fn append_child(&mut self, child: Node) {
        match self {
            &mut Node::LTR(ref mut items) => {
                items.push(child);
            }

            &mut Node::TTB(ref mut items) => {
                items.push(child);
            }

            _ => (),
        }
    }

    pub fn draw(&self, config: &Config, buf: &mut String, origin: Point) {
        // When these functions retrieve self dimensions, it's done via the
        // self.dimensions call (which is polymorphic) rather than directly from the struct.
        // so we're working to the same interface as the parent laying this out.
        let dimensions = self.get_dimensions();

        match self {
            &Node::Blank(_) => (),

            &Node::Block(_) => {
                write!(
                    buf,
                    "<rect x='{}' y='{}' width='{}' height='{}' \
                     style='fill:none;stroke:black;stroke-width:1' />\n",
                    origin.0, origin.1, dimensions.0, dimensions.1
                ).unwrap();
            }

            &Node::SolidBlock(_) => {
                write!(
                    buf,
                    "<rect x='{}' y='{}' width='{}' height='{}' \
                     style='fill:#333 solid;stroke:black;stroke-width:1' />\n",
                    origin.0, origin.1, dimensions.0, dimensions.1
                ).unwrap();
            }

            &Node::PlaceholderFrame(_) => {
                write!(
                    buf,
                    "<rect x='{}' y='{}' width='{}' height='{}' \
                      style='fill:none;stroke:black;stroke-width:1' />\n
                     <line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1' stroke='black' />\n
                     <line x1='{}' y1='{}' x2='{}' y2='{}' stroke-width='1' stroke='black' />\n",
                    origin.0, origin.1,
                    dimensions.0, dimensions.1,
                    origin.0, origin.1, origin.0 + dimensions.0, origin.1 + dimensions.1,
                    origin.0 + dimensions.0, origin.1, origin.0, origin.1 + dimensions.1
                ).unwrap();
            }

            &Node::LTR(ref children) => {
                let mut x = origin.0;
                let mut y = origin.1;

                for ref node in children.iter() {
                    node.draw(config, buf, Point(x, y));
                    x += node.get_dimensions().0;
                }
            }

            &Node::TTB(ref children) => {
                let mut x = origin.0;
                let mut y = origin.1;

                for ref node in children.iter() {
                    node.draw(config, buf, Point(x, y));
                    y += node.get_dimensions().1;
                }
            }
        }

        if config.draw_bounding_box {
            write!(
                buf,
                "<rect x='{}' y='{}' width='{}' height='{}' \
                 class='debug debug-node' style='fill:none;stroke:blue;stroke-width:2;opacity:0.5' />\n",
                origin.0, origin.1, dimensions.0, dimensions.1
            ).unwrap();
        }
    }
}

// fn draw(&self, config: &Config, buf : &mut String, x: f32, y: f32, xx: f32, yy: f32);

///! Left-to-right container.
///! Height depends on contents. Width depends on contents / container.
pub struct Page {
    root: Node,
}

impl Page {
    pub fn new() -> Page {
        Page {
            // Start with nothing. An appropriate root note will get swapped in.
            root: Node::Blank(Dimensions(0.0, 0.0)),
        }
    }

    pub fn set_root(&mut self, root: Node) {
        self.root = root;
    }

    pub fn write(&self, config: &Config, buf: &mut String) {
        let dimensions = self.root.get_dimensions();

        write!(
            buf,
            "<svg version='1.1' baseProfile='full' width='{}' height='{}' \
             xmlns='http://www.w3.org/2000/svg'>\n",
            dimensions.0, dimensions.1
        ).unwrap();

        self.root.draw(config, buf, Point(0.0, 0.0));

        if config.draw_bounding_box {
            write!(
                buf,
                "<rect x='{}' y='{}' width='{}' height='{}' \
                 class='debug debug-page' style='fill:none;stroke:blue;stroke-width:2;opacity:0.5' />\n",
                0, 0, dimensions.0, dimensions.1
            ).unwrap();
        }

        write!(buf, "</svg>").unwrap();
    }
}
