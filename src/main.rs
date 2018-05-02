//! Layout experiment
//! Prototyping an SVG page layout engine.
use std::fs::File;
use std::io::Write;


mod layout;

fn demo_1() {

    let mut page = layout::Page::new();

    let mut ttb = layout::Node::new_ttb();

    let mut ltr_1 = layout::Node::new_ltr();
    ltr_1.append_child(layout::Node::Blank(layout::Dimensions(200.0, 0.0)));
    ltr_1.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(600.0, 100.0)));
    ltr_1.append_child(layout::Node::Blank(layout::Dimensions(200.0, 0.0)));
    ttb.append_child(ltr_1);

    ttb.append_child(layout::Node::Blank(layout::Dimensions(0.0, 50.0)));

    let mut ltr_2 = layout::Node::new_ltr();

    let mut ltr_2_ttb_1 = layout::Node::new_ttb();
    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(300.0, 20.0)));
    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(300.0, 20.0)));
    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(300.0, 20.0)));
    ltr_2.append_child(ltr_2_ttb_1);

    ltr_2.append_child(layout::Node::Blank(layout::Dimensions(400.0, 100.0)));

    let mut ltr_2_ttb_2 = layout::Node::new_ttb();
    ltr_2_ttb_2.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(300.0, 30.0)));
    ltr_2_ttb_2.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(300.0, 30.0)));
    ltr_2.append_child(ltr_2_ttb_2);

    ttb.append_child(ltr_2);

    ttb.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(1000.0, 50.0)));
    ttb.append_child(layout::Node::Blank(layout::Dimensions(00.0, 10.0)));
    ttb.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(1000.0, 50.0)));
    ttb.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0)));
    ttb.append_child(layout::Node::PlaceholderFrame(layout::Dimensions(1000.0, 50.0)));

    page.set_root(ttb);


    // Write normally.
    let mut string_buffer = String::new();
    let config = layout::Config {
        draw_bounding_box: false,
    };
    page.write(&config, &mut string_buffer);
    let mut f = File::create("demo.svg").expect("Unable to create file");
    f.write_all(string_buffer.as_bytes()).expect("Unable to write data");

    // And write with bounding boxes for debugging.
    let mut string_buffer = String::new();

    let config = layout::Config {
        draw_bounding_box: true,
    };
    page.write(&config, &mut string_buffer);

    let mut f = File::create("demo_debug.svg").expect("Unable to create file");
    f.write_all(string_buffer.as_bytes()).expect("Unable to write data");


}

fn main() {
    demo_1();
}
