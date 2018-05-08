//! Layout experiment
//! Prototyping an SVG page layout engine.
//! Design is somewhat coupled: constructors passing arguments through, static polymorphism.
//! But this is destined to be part of a special purpose application, not a library.

mod demo;
mod layout;
mod music_typeset;

fn main() {
    demo::demo_typeset();
}
