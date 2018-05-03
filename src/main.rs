//! Layout experiment
//! Prototyping an SVG page layout engine.
use std::fs::File;
use std::io::Write;

mod layout;

fn demo_1() {
    let small_margin = layout::Margin::new(20.0, 0.0, 0.0, 0.0);
    let no_margin = layout::Margin::new(0.0, 0.0, 0.0, 0.0);
    let big_margin = layout::Margin::new(10.0, 50.0, 10.0, 50.0);

    let big_text = layout::TextSpec::new(30.0, layout::TextAnchor::Middle);
    let normal_text_l = layout::TextSpec::new(20.0, layout::TextAnchor::Start);
    let normal_text_m = layout::TextSpec::new(20.0, layout::TextAnchor::Middle);
    let normal_text_r = layout::TextSpec::new(20.0, layout::TextAnchor::End);
    let small_text_m = layout::TextSpec::new(15.0, layout::TextAnchor::Middle);

    let mut page = layout::Page::new(layout::Margin::new_uniform(20.0));

    let mut ttb = layout::Node::new_ttb(small_margin);

    let mut ltr_1 = layout::Node::new_ltr_justify(small_margin, 1000.0);
    ltr_1.append_child(layout::Node::Blank(
        layout::Dimensions(200.0, 0.0),
        small_margin,
    ));
    ltr_1.append_child(layout::Node::new_text(
        no_margin,
        big_text,
        "My Cat's Got No Arms".to_string(),
    ));
    ltr_1.append_child(layout::Node::Blank(
        layout::Dimensions(200.0, 0.0),
        small_margin,
    ));
    ttb.append_child(ltr_1);

    // ttb.append_child(layout::Node::Blank(layout::Dimensions(0.0, 50.0), small_margin));

    let mut author = layout::Node::new_ltr_justify(small_margin, 1000.0);
    author.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "Old codswolloping song".to_string(),
    ));
    author.append_child(layout::Node::new_text(
        no_margin,
        normal_text_r,
        "Trad. Cornish".to_string(),
    ));
    ttb.append_child(author);

    let mut ltr_2 = layout::Node::new_ltr(small_margin);

    let mut ltr_2_ttb_1 = layout::Node::new_ttb(small_margin);
    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(400.0, 20.0),
        big_margin,
    ));
    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(400.0, 20.0),
        big_margin,
    ));
    ltr_2_ttb_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(400.0, 20.0),
        big_margin,
    ));
    ltr_2.append_child(ltr_2_ttb_1);

    // ltr_2.append_child(layout::Node::Blank(layout::Dimensions(400.0, 100.0), small_margin));

    let mut ltr_2_ttb_2 = layout::Node::new_ttb(small_margin);
    ltr_2_ttb_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(400.0, 40.0),
        big_margin,
    ));
    ltr_2_ttb_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(400.0, 40.0),
        big_margin,
    ));
    ltr_2.append_child(ltr_2_ttb_2);
    ttb.append_child(ltr_2);

    // ttb.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut lr_blocks = layout::Node::new_ltr(small_margin);
    let mut left_block = layout::Node::new_ttb(small_margin);

    let mut stave_1 = layout::Node::new_ltr_justify(small_margin, 500.0);
    stave_1.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    left_block.append_child(stave_1);

    // left_block.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut stave_2 = layout::Node::new_ltr_justify(small_margin, 500.0);
    stave_2.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_2.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    left_block.append_child(stave_2);

    // left_block.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut stave_3 = layout::Node::new_ltr_justify(small_margin, 500.0);
    stave_3.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_3.append_child(layout::Node::Block(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_3.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    left_block.append_child(stave_3);

    // left_block.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut stave_5 = layout::Node::new_ltr_justify(small_margin, 500.0);
    stave_5.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_5.append_child(layout::Node::Block(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_5.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_5.append_child(layout::Node::Block(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_5.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    left_block.append_child(stave_5);

    // left_block.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut stave_10 = layout::Node::new_ltr_justify(small_margin, 500.0);
    stave_10.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::Block(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::Block(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::Block(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::Block(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::SolidBlock(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10.append_child(layout::Node::Block(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    left_block.append_child(stave_10);

    // left_block.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    lr_blocks.append_child(left_block);
    lr_blocks.append_child(layout::Node::Blank(
        layout::Dimensions(20.0, 0.0),
        small_margin,
    ));
    lr_blocks.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(480.0, 180.0),
        small_margin,
    ));
    ttb.append_child(lr_blocks);

    let mut stave_100 = layout::Node::new_ltr_justify(small_margin, 1000.0);
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 20.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 40.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 60.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 80.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 80.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 80.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 80.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 60.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 40.0),
        small_margin,
    ));
    stave_100.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(100.0, 20.0),
        small_margin,
    ));
    ttb.append_child(stave_100);

    // ttb.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut stave_10_uneven_1 = layout::Node::new_ltr_justify(small_margin, 1000.0);
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(50.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(40.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(30.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(10.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(5.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    ttb.append_child(stave_10_uneven_1);

    // ttb.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut stave_10_uneven_2 = layout::Node::new_ltr_justify(small_margin, 1000.0);
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(80.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(50.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(70.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(50.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(60.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(50.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(40.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(50.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(35.0, 20.0),
        small_margin,
    ));
    stave_10_uneven_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(50.0, 20.0),
        small_margin,
    ));
    ttb.append_child(stave_10_uneven_2);

    // ttb.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    let mut stave_very_uneven = layout::Node::new_ltr_justify(small_margin, 1000.0);
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(50.0, 20.0),
        small_margin,
    ));
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(80.0, 20.0),
        small_margin,
    ));
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        small_margin,
    ));
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(500.0, 20.0),
        small_margin,
    ));
    stave_very_uneven.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(250.0, 20.0),
        small_margin,
    ));
    ttb.append_child(stave_very_uneven);

    // ttb.append_child(layout::Node::Blank(layout::Dimensions(00.0, 20.0), small_margin));

    ttb.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(1000.0, 50.0),
        small_margin,
    ));
    ttb.append_child(layout::Node::Blank(
        layout::Dimensions(00.0, 10.0),
        small_margin,
    ));
    ttb.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(1000.0, 50.0),
        small_margin,
    ));
    ttb.append_child(layout::Node::Blank(
        layout::Dimensions(00.0, 20.0),
        small_margin,
    ));
    ttb.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(1000.0, 50.0),
        small_margin,
    ));

    // ttb.append_child(layout::Node::Blank(layout::Dimensions(00.0, 80.0), small_margin));

    let mut compound_stave = layout::Node::new_ltr_justify(small_margin, 1000.0);

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::Dimensions(1.0, 60.0),
        small_margin,
    ));

    let mut compound_stave_sub_1 = layout::Node::new_ltr(small_margin);
    compound_stave_sub_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        no_margin,
    ));
    compound_stave_sub_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 30.0),
        no_margin,
    ));
    compound_stave_sub_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 40.0),
        no_margin,
    ));
    compound_stave_sub_1.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        no_margin,
    ));
    compound_stave.append_child(compound_stave_sub_1);

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::Dimensions(1.0, 60.0),
        small_margin,
    ));

    let mut compound_stave_sub_2 = layout::Node::new_ltr(small_margin);
    compound_stave_sub_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 30.0),
        no_margin,
    ));
    compound_stave_sub_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 40.0),
        no_margin,
    ));
    compound_stave_sub_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 50.0),
        no_margin,
    ));
    compound_stave_sub_2.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 60.0),
        no_margin,
    ));
    compound_stave.append_child(compound_stave_sub_2);

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::Dimensions(1.0, 60.0),
        small_margin,
    ));

    let mut compound_stave_sub_3 = layout::Node::new_ltr(small_margin);
    compound_stave_sub_3.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 60.0),
        no_margin,
    ));
    compound_stave_sub_3.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 30.0),
        no_margin,
    ));
    compound_stave_sub_3.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 20.0),
        no_margin,
    ));
    compound_stave_sub_3.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 40.0),
        no_margin,
    ));
    compound_stave.append_child(compound_stave_sub_3);

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::Dimensions(1.0, 60.0),
        small_margin,
    ));

    let mut compound_stave_sub_4 = layout::Node::new_ltr(small_margin);
    compound_stave_sub_4.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 50.0),
        no_margin,
    ));
    compound_stave_sub_4.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 40.0),
        no_margin,
    ));
    compound_stave_sub_4.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(20.0, 30.0),
        no_margin,
    ));
    compound_stave_sub_4.append_child(layout::Node::PlaceholderFrame(
        layout::Dimensions(40.0, 10.0),
        no_margin,
    ));
    compound_stave.append_child(compound_stave_sub_4);

    compound_stave.append_child(layout::Node::SolidBlock(
        layout::Dimensions(1.0, 60.0),
        small_margin,
    ));

    ttb.append_child(compound_stave);

    ttb.append_child(layout::Node::Blank(
        layout::Dimensions(00.0, 20.0),
        small_margin,
    ));


    let mut words = layout::Node::new_ttb(layout::Margin::new(5.0, 0.0, 50.0, 200.0));
    words.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "My cat's he's got no arms or legs".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "He likes to climb up stairs ".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "I've never seen him doing it".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "I suspect he's never actually done it".to_string(),
    ));
    
    words.append_child(layout::Node::Blank(
        layout::Dimensions(10.0, 20.0),
        small_margin,
    ));

    words.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "My cat's a liar and quite possibly an idiot".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "But I do like him".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "Because he brings me mice".to_string(),
    ));
    words.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "And socks when I leave them on the ground floor".to_string(),
    ));

    ttb.append_child(words);

    let mut footnote = layout::Node::new_ltr_justify(small_margin, 1000.0);
    footnote.append_child(layout::Node::new_text(
        no_margin,
        normal_text_l,
        "Cat Songs vol XVII".to_string(),
    ));
    footnote.append_child(layout::Node::new_text(
        no_margin,
        small_text_m,
        "32".to_string(),
    ));
    footnote.append_child(layout::Node::new_text(
        no_margin,
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

fn main() {
    demo_1();
}
