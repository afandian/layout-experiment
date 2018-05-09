use std::fs::File;
use std::io::Write;

use layout;
use music_typeset;
use std::fmt::Write as _fw;

pub fn demo_typeset() {
    let config = layout::Config {
        draw_bounding_box: false,
    };

    let mut chunks: Vec<music_typeset::Chunk> = vec![];
    chunks.push(music_typeset::Chunk::Title("Bear Dance".to_string()));

    let mut stave: Vec<music_typeset::StaveEntity> = vec![];
    stave.push(music_typeset::StaveEntity::Note(
        0,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -4,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -4,
        music_typeset::NoteHeadSymbol::Empty,
    ));
    stave.push(music_typeset::StaveEntity::Barline(
        music_typeset::BarlineSymbol::Single,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        0,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -4,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -4,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -3,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Barline(
        music_typeset::BarlineSymbol::Single,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -2,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -2,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -3,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -2,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Barline(
        music_typeset::BarlineSymbol::Single,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -1,
        music_typeset::NoteHeadSymbol::Empty,
    ));
    stave.push(music_typeset::StaveEntity::Barline(
        music_typeset::BarlineSymbol::Double,
    ));
    chunks.push(music_typeset::Chunk::Stave(stave));

    // Line 2
    let mut stave: Vec<music_typeset::StaveEntity> = vec![];
    stave.push(music_typeset::StaveEntity::Note(
        0,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        0,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -1,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -1,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Barline(
        music_typeset::BarlineSymbol::Single,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -2,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -2,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -3,
        music_typeset::NoteHeadSymbol::Empty,
    ));
    stave.push(music_typeset::StaveEntity::Barline(
        music_typeset::BarlineSymbol::Single,
    ));

    stave.push(music_typeset::StaveEntity::Note(
        -4,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -2,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -3,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -5,
        music_typeset::NoteHeadSymbol::Filled,
    ));
    stave.push(music_typeset::StaveEntity::Barline(
        music_typeset::BarlineSymbol::Single,
    ));
    stave.push(music_typeset::StaveEntity::Note(
        -4,
        music_typeset::NoteHeadSymbol::Empty,
    ));
    stave.push(music_typeset::StaveEntity::Barline(
        music_typeset::BarlineSymbol::Double,
    ));

    chunks.push(music_typeset::Chunk::Stave(stave));

    let page = music_typeset::typeset(&chunks);

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
