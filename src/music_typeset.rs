use std::fmt::Write as _fw;
use layout;

/// Position on a stave relative to the centre line.

const NUM_LINES : StavePosition = 5;

type StavePosition = u32;

pub enum Chunk {
	 Title(String),
	 Stave(Vec<StaveEntity>)	
}

pub enum ClefSymbol {
	Treble,
	Bass,
	Tenor
}

pub enum NoteHeadSymbol{
	Filled,
	Empty,
	Cross
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

