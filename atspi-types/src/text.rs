use serde::{Serialize, Deserialize};
use zvariant::Type;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
/// Level of granularity to get text of, in relation to a cursor position.
pub enum Granularity {
	/// Gives the character at the index of the cursor. With a line-style cursor (which is standard) this will get the chracter that appears after the cursor.
	Char,
	/// Gives the entire word in front of or which contains the cursor. TODO: confirm that it always chooses the word in front of the cursor.
	Word,
	/// Gives to entire sentence in fron of the or which contains the cursor. TODO: confirm that it always chooses the sentence after the cursor.
	Sentence,
	/// Gives the line, as seen visually of which the cursor is situated within.
	Line,
	/// Gives the entire block of text, regardless of where the cursor lies within it.
	Paragraph,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum ClipType {
	Neither,
	Min,
	Max,
	Both,
}

