use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
    vec,
};

use argh::FromArgs;
use atspi_codegen::*;
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use zbus::zvariant::{
    Basic, ObjectPath, Signature, ARRAY_SIGNATURE_CHAR, DICT_ENTRY_SIG_END_CHAR,
    DICT_ENTRY_SIG_START_CHAR, STRUCT_SIG_END_CHAR, STRUCT_SIG_START_CHAR, VARIANT_SIGNATURE_CHAR,
};

const STRIPPER_IGNORE_START: &str = "// IgnoreBlock start";
const STRIPPER_IGNORE_STOP: &str = "// IgnoreBlock stop";

enum AtspiEventInnerName {
    Detail1,
    Detail2,
    AnyData,
}

impl ToString for AtspiEventInnerName {
    fn to_string(&self) -> String {
        match self {
            Self::Detail1 => "detail1",
            Self::Detail2 => "detail2",
            Self::AnyData => "any_data",
        }
        .to_string()
    }
}

#[derive(Debug)]
enum ConversionError {
    FunctionAlreadyCreatedFor,
    UnknownItem,
}
impl TryFrom<usize> for AtspiEventInnerName {
    type Error = ConversionError;

    fn try_from(from: usize) -> Result<Self, Self::Error> {
        match from {
            0 => Err(ConversionError::FunctionAlreadyCreatedFor),
            1 => Ok(Self::Detail1),
            2 => Ok(Self::Detail2),
            3 => Ok(Self::AnyData),
            4 => Err(ConversionError::FunctionAlreadyCreatedFor),
            _ => Err(ConversionError::UnknownItem),
        }
    }
}

// taken from zbus_xmlgen: https://gitlab.freedesktop.org/dbus/zbus/-/blob/main/zbus_xmlgen/src/gen.rs
fn to_rust_type(ty: &str, input: bool, as_ref: bool) -> String {
    // can't haz recursive closure, yet
    fn iter_to_rust_type(
        it: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        input: bool,
        as_ref: bool,
    ) -> String {
        let c = it.next().unwrap();
        match *c as char {
            u8::SIGNATURE_CHAR => "u8".into(),
            bool::SIGNATURE_CHAR => "bool".into(),
            i16::SIGNATURE_CHAR => "i16".into(),
            u16::SIGNATURE_CHAR => "u16".into(),
            i32::SIGNATURE_CHAR => "i32".into(),
            u32::SIGNATURE_CHAR => "u32".into(),
            i64::SIGNATURE_CHAR => "i64".into(),
            u64::SIGNATURE_CHAR => "u64".into(),
            f64::SIGNATURE_CHAR => "f64".into(),
            // xmlgen accepts 'h' on Windows, only for code generation
            'h' => (if input { "zbus::zvariant::Fd" } else { "zbus::zvariant::OwnedFd" }).into(),
            <&str>::SIGNATURE_CHAR => (if input || as_ref { "&str" } else { "String" }).into(),
            ObjectPath::SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::ObjectPath<'_>"
                } else {
                    "zbus::zvariant::ObjectPath<'_>"
                }
            } else {
                "zbus::zvariant::OwnedObjectPath"
            })
            .into(),
            Signature::SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::Signature<'_>"
                } else {
                    "zbus::zvariant::Signature<'_>"
                }
            } else {
                "zbus::zvariant::OwnedSignature"
            })
            .into(),
            VARIANT_SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::Value<'_>"
                } else {
                    "zbus::zvariant::Value<'_>"
                }
            } else {
                "zbus::zvariant::OwnedValue"
            })
            .into(),
            ARRAY_SIGNATURE_CHAR => {
                let c = it.peek().unwrap();
                match **c as char {
                    '{' => format!(
                        "std::collections::HashMap<{}>",
                        iter_to_rust_type(it, input, false)
                    ),
                    _ => {
                        let ty = iter_to_rust_type(it, input, false);
                        if input {
                            format!("&[{ty}]")
                        } else {
                            format!("{}Vec<{}>", if as_ref { "&" } else { "" }, ty)
                        }
                    }
                }
            }
            c @ STRUCT_SIG_START_CHAR | c @ DICT_ENTRY_SIG_START_CHAR => {
                let dict = c == '{';
                let mut vec = vec![];
                loop {
                    let c = it.peek().unwrap();
                    match **c as char {
                        STRUCT_SIG_END_CHAR | DICT_ENTRY_SIG_END_CHAR => break,
                        _ => vec.push(iter_to_rust_type(it, input, false)),
                    }
                }
                if dict {
                    vec.join(", ")
                } else if vec.len() > 1 {
                    format!("{}({})", if as_ref { "&" } else { "" }, vec.join(", "))
                } else {
                    vec[0].to_string()
                }
            }
            _ => unimplemented!(),
        }
    }

    let mut it = ty.as_bytes().iter().peekable();
    iter_to_rust_type(&mut it, input, as_ref)
}

fn iface_name(iface: &Interface) -> String {
    iface
        .name()
        .split('.')
        .next_back()
        .expect("An interface must have a period in its name.")
        .to_string()
}

fn into_rust_enum_str<S>(string: S) -> String
where
    S: Into<String>,
{
    // needed to escape the uUShadeEvent
    // make sure Count is its own word
    // make sure Width is its own word
    string
        .into()
        .replace("uU", "UU")
        .replace("count", "Count")
        .replace("width", "Width")
}

fn events_ident<S>(string: S) -> String
where
    S: Into<String>,
{
    let mut sig_name_event_str = string.into();
    sig_name_event_str.push_str("Events");
    into_rust_enum_str(sig_name_event_str)
}

fn event_ident<S>(string: S) -> String
where
    S: Into<String>,
{
    let mut sig_name_event_str = string.into();
    sig_name_event_str.push_str("Event");
    into_rust_enum_str(sig_name_event_str)
}

fn generate_fn_for_signal_item(signal_item: &Arg, inner_event_name: AtspiEventInnerName) -> String {
    if signal_item.name().is_none() {
        return String::new();
    }
    // unwrap is safe due to check
    let function_name = signal_item.name().expect("No name for arg");
    let inner_name = inner_event_name.to_string();
    let rust_type = to_rust_type(signal_item.ty(), true, true);

    format!(
        "
		#[must_use]
		pub fn {function_name}(&self) -> {rust_type} {{
			self.0.{inner_name}()
		}}
	"
    )
}

fn generate_impl_from_signal(signal: &Signal) -> String {
    let sig_name_event = event_ident(signal.name());
    let functions = signal
        .args()
        .iter()
        .enumerate()
        .filter_map(|(i, arg)| {
            let func_name = i.try_into();
            let arg_name = arg.name();
            match (func_name, arg_name) {
                (Ok(func), Some(_)) => Some(generate_fn_for_signal_item(arg, func)),
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "
	impl {sig_name_event} {{
		{functions}
	}}
	"
    )
}

fn generate_signal_associated_example(mod_name: &str, signal_name: &str) -> String {
    format!(
        "{STRIPPER_IGNORE_START}
    /// # Example
    ///
    /// Even though this example employs `Tokio`, any runtime will do.
    ///
    /// Note that the example is minimized for rhe sake of brevity.
    /// More complete examples may be found in the `examples/` directory.
    ///
    /// ```
    /// use atspi::{{events::EventInterfaces, Event}};
    /// use atspi::identify::{mod_name}::{signal_name};
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {{
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {{
    /// #       let Ok(ev) = ev else {{ break }};
    ///         let Ok(event)  = {signal_name}::try_from(ev) else {{ continue }};
    ///     }}
    /// }}
    /// ```
    {STRIPPER_IGNORE_STOP}"
    )
}

fn generate_struct_from_signal(mod_name: &str, signal: &Signal) -> String {
    let sig_name_event = event_ident(signal.name());
    let example = generate_signal_associated_example(mod_name, &sig_name_event);
    format!(
        "
    {example}
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct {sig_name_event}(pub(crate) AtspiEvent);
	"
    )
}

fn generate_variant_from_signal(signal: &Signal) -> String {
    let sig_name = into_rust_enum_str(signal.name());
    let sig_name_event = event_ident(signal.name());
    format!("		{sig_name}({sig_name_event}),")
}

fn match_arm_for_signal(iface_name: &str, signal: &Signal) -> String {
    let raw_signal_name = signal.name();
    let enum_signal_name = into_rust_enum_str(raw_signal_name);
    let enum_name = events_ident(iface_name);
    let signal_struct_name = event_ident(raw_signal_name);
    format!(
        "				\"{raw_signal_name}\" => Ok({enum_name}::{enum_signal_name}({signal_struct_name}(ev))),"
    )
}

fn generate_try_from_atspi_event(iface: &Interface) -> String {
    let iname = iface_name(iface);
    let error_str = format!("No matching member for {iname}");
    let impl_for_name = events_ident(&iname);
    let member_conversions = iface
        .signals()
        .iter()
        .map(|signal| match_arm_for_signal(&iname, signal))
        .collect::<Vec<String>>()
        .join("\n");
    format!("
	impl TryFrom<AtspiEvent> for {impl_for_name} {{
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {{
			let Some(member) = ev.member() else {{ return Err(AtspiError::MemberMatch(\"Event w/o member\".into())); }};
			match member.as_str() {{
{member_conversions}
				_ => Err(AtspiError::MemberMatch(\"{error_str}\".into())),
			}}
		}}
	}}
	")
}

fn generate_mod_from_iface(iface: &Interface) -> String {
    let mod_name = iface_name(iface).to_lowercase();
    let enums = generate_enum_from_iface(iface);
    let structs = iface
        .signals()
        .iter()
        .map(|signal| generate_struct_from_signal(&mod_name, signal))
        .collect::<Vec<String>>()
        .join("\n");
    let impls = iface
        .signals()
        .iter()
        .map(|signal| generate_impl_from_signal(signal))
        .collect::<Vec<String>>()
        .join("\n");
    let try_froms = generate_try_from_atspi_event(iface);
    format!(
        "
#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod {mod_name} {{
	use atspi_macros::TrySignify;
	use crate::{{
		error::AtspiError,
		events::{{AtspiEvent, GenericEvent}},
		signify::Signified,
	}};
	use zbus;
	use zbus::zvariant::OwnedValue;
	{enums}
	{structs}
	{impls}
	{try_froms}
}}
	"
    )
}

fn generate_enum_associated_example(iface_name: &str) -> String {
    format!(
  "{STRIPPER_IGNORE_START}
    /// # Example
    ///
    /// Even though this example employs `Tokio`, any runtime will do.
    ///
    /// Note that this example is minimized for rhe sake of brevity.
    /// More complete examples may be found in the `examples/` directory.
    ///
    /// ```
    /// use atspi::{{events::EventInterfaces, Event}};
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {{
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {{
    /// #       let Ok(ev) = ev else {{ break }};
    ///          let Event::Interfaces(EventInterfaces::{iface_name}(_event)) = ev else {{ continue }};
    ///     }}
    /// }}
    /// ```
    {STRIPPER_IGNORE_STOP}"
    )
}

fn generate_enum_from_iface(iface: &Interface) -> String {
    let name_ident = iface
        .name()
        .split('.')
        .next_back()
        .expect("Interface must contain a period");
    let example_string = generate_enum_associated_example(name_ident);
    let name_ident_plural = events_ident(name_ident);
    let signal_quotes = iface
        .signals()
        .into_iter()
        .map(generate_variant_from_signal)
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        "
    {example_string}
	#[derive(Clone, Debug)]
	pub enum {name_ident_plural} {{
{signal_quotes}
	}}
	"
    )
}

pub fn create_events_from_xml(file_name: &str) -> String {
    let xml_file = std::fs::File::open(file_name).expect("Cannot read file");
    let data: Node = Node::from_reader(&xml_file).expect("Cannot deserialize file");
    let module_level_doc = {
        if let Some(doc) = data.doc() {
            let docdata = doc.data;
            format!("{STRIPPER_IGNORE_START}\n{docdata}\n{STRIPPER_IGNORE_STOP}")
        } else {
            String::new()
        }
    };
    let iface_data = data
        .interfaces()
        .iter()
        .map(|iface| generate_mod_from_iface(iface))
        .collect::<Vec<String>>()
        .join("\n");
    format!("{module_level_doc}\n{iface_data}")
}

/// Save manual doc-comments, then generating new sources and reinstate manual doc-comments.
#[derive(FromArgs, Default)]
struct Args {
    /// save manual doc-comments, then exit
    #[argh(switch, short = 's')]
    docs_file: bool,

    /// write manual doc-comments to stdout, then exit
    #[argh(switch, short = 'o')]
    docs_stdout: bool,

    /// regenerate sources from xml, write to stdout
    #[argh(switch, short = 'r')]
    regen_stdout: bool,

    /// regenerate sources from xml, write to source file
    #[argh(switch, short = 'f')]
    regen_file: bool,

    /// reinstate - restore docs from file
    #[argh(switch, short = 'i')]
    insert: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct ItemLevel {
    // distance to next 'identifier' / string we can associate the docs with
    dist: u8,
    doc: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct Comment {
    dist: u8,
    doc: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct ModuleLevel {
    doc: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum DocType {
    Module(ModuleLevel),
    Item(ItemLevel),
    Comment(Comment),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    None,
    Comment,
    ModuleLevel,
    ItemLevel,
    IgnoreBlock,
}

/// Reads from the source file into a Vec.
/// HashMap does not (necessarilly) preserve order of insertion.  Hence Vec.
fn read_file_to_vec(path: &Path) -> Vec<(Option<String>, DocType)> {
    let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
    let mut saved = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("could not open save file");

    let mut buf = String::new();
    let n = saved.read_to_string(&mut buf).expect("could not read to file to buf");
    println!("read {n} bytes to buffer.");

    let mut docblock: Vec<String> = Vec::new();
    let mut docstate = ParseState::None;
    let mut counter = 0;

    for line in buf.lines() {
        match docstate {
            ParseState::None => match line {
                line if line.trim().starts_with("//!") => {
                    docstate = ParseState::ModuleLevel;
                    docblock.push(line.into());
                    continue;
                }
                line if line.trim().starts_with("///") => {
                    docstate = ParseState::ItemLevel;
                    docblock.push(line.into());
                    continue;
                }
                line if line.trim().starts_with("//") => {
                    if line.contains(STRIPPER_IGNORE_START) {
                        docstate = ParseState::IgnoreBlock;
                        continue;
                    }
                    docstate = ParseState::Comment;
                    docblock.push(line.into());
                    continue;
                }
                _ => {
                    continue;
                }
            },

            ParseState::ModuleLevel => {
                // As long as `line` starts with '//' it is still comment. a mixed block is also a block.
                if line.trim().starts_with("//") {
                    docblock.push(line.into());
                } else {
                    docstate = ParseState::None;
                    let dt = DocType::Module(ModuleLevel { doc: docblock.clone() });
                    docblock.clear();
                    docvec.push((None, dt));
                }
                continue;
            }

            ParseState::ItemLevel => {
                // As long as `line` starts with '//' it is still comment. a mixed block is also a block.
                if line.trim().starts_with("//") {
                    docblock.push(line.into());
                } else {
                    if line.trim().starts_with("#[") {
                        counter += 1;
                        continue;
                    }
                    if !line.trim().is_empty() {
                        let docitem = ItemLevel { dist: counter, doc: docblock.clone() };
                        docblock.clear();
                        counter = 0;
                        docstate = ParseState::None;
                        let dt = DocType::Item(docitem);
                        docvec.push((Some(line.trim().into()), dt));
                        continue;
                    }
                }
            }

            ParseState::Comment => {
                if line.trim().starts_with("//") {
                    docblock.push(line.into());
                } else if line.trim().starts_with("#[") || line.trim().is_empty() {
                    counter += 1;
                } else if line.trim() == "{" {
                    // A single curly brace or an attribute macro is too common to be an 'anchor'
                    docstate = ParseState::None;
                    docblock.clear();
                    counter = 0;
                } else if !line.trim().is_empty() {
                    let docitem = Comment { dist: counter, doc: docblock.clone() };
                    let dt = DocType::Comment(docitem);
                    docvec.push((Some(line.trim().into()), dt));
                    docblock.clear();
                    counter = 0;
                    docstate = ParseState::None;
                }
                continue;
            }

            ParseState::IgnoreBlock => {
                if line.contains(STRIPPER_IGNORE_STOP) {
                    docstate = ParseState::None;
                }
                continue;
            }
        }
    }
    docvec
}

// Tries to match strings within source and return the docs to their associated 'lines'.
// If this does not work, we might want to use `syn`.
fn reinstate_docs(path: &Path, docvec: Vec<(Option<String>, DocType)>) {
    let mut source_string = String::new();
    let mut remains = docvec.clone();

    {
        let mut source = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("could not open save file");

        let _ = source
            .read_to_string(&mut source_string)
            .expect("could not read source file to string");
    }

    // Make Vec<String>s from whole String.
    let source_lines: Vec<String> = source_string.lines().map(|s| s.to_string()).collect();
    let mut source_and_doc_lines: Vec<String> = source_lines.clone();

    // For each key in map, look for lines in Vec that contain that key.
    // if so, insert docs that point, honoring distance and taking in account offset,
    for (k, v) in docvec {
        if k.is_none() {
            if let DocType::Module(ModuleLevel { ref doc }) = v {
                source_and_doc_lines.splice(0..0, doc.iter().cloned());
                remains.retain(|tup| *tup != (k.clone(), v.clone()));
                continue;
            } else {
                unreachable!("k == None implies ModuleLevel docs.");
            }
        }

        let pat = k.clone().unwrap();
        for s in source_lines.iter() {
            if s.contains(&pat) {
                let idx = source_and_doc_lines
                    .iter()
                    .position(|line| (*line).contains(&pat))
                    .expect("source_lines contains pat, therefore source_and_doc_lines does too");
                match v {
                    DocType::Item(ItemLevel { dist, ref doc }) => {
                        let i = idx - dist as usize;
                        source_and_doc_lines.splice(i..i, doc.iter().cloned());
                        remains.retain(|tup| *tup != (k.clone(), v.clone()));
                    }
                    DocType::Comment(Comment { dist, ref doc }) => {
                        let i = idx - dist as usize;
                        source_and_doc_lines.splice(i..i, doc.iter().cloned());
                        remains.retain(|tup| *tup != (k.clone(), v.clone()));
                    }
                    _ => {
                        unreachable!("k == None implies ModuleLevel docs.");
                    }
                }
                continue;
            }
        }
    }

    // collect all strings in vec
    let new_source: String = source_and_doc_lines
        .into_iter()
        .map(|line| if !line.ends_with('\n') { line + "\n" } else { line })
        .collect();

    // write string to source
    std::fs::write(path, new_source).expect("Unable to write file");

    if remains.is_empty() {
        return;
    }
    println!("The following items could not be reinstated:");
    println!("{remains:#?}");
    println!("Number of items not reinstated: {}", remains.len());
}

/// Writes the map to the path
fn write_map_to_file(docvec: &Vec<(Option<String>, DocType)>, path: &Path) {
    // open file
    let save_comments_file = File::create(path).expect("comments file should open");
    // Configure printstyle
    let pretty = PrettyConfig::new().depth_limit(4).indentor("    ".to_owned());
    // serialize and write map
    if to_writer_pretty(save_comments_file, docvec, pretty).is_ok() {
        println!("comments saved!");
    } else {
        eprintln!("Comments could not be formatted and saved.")
    }
}

/// Writes the doc-comments map to stdout
fn write_docs_to_stdout(docvec: &Vec<(Option<String>, DocType)>) {
    // Configure print-style
    let pretty = PrettyConfig::new().depth_limit(4).indentor("    ".to_owned());

    // acquire lock on stdout
    let stdout = std::io::stdout().lock();

    // serialize and write to stdout
    if to_writer_pretty(stdout, docvec, pretty).is_err() {
        eprint!("Comments could not be formatted and written to stdout.")
    }
}

/// Load RON file, deserialize to vec of docs
fn load_saved_comments(path: &Path) -> Vec<(Option<String>, DocType)> {
    let serialized =
        std::fs::read_to_string(path).expect("failed to read serialized docmap from file");

    // deserialize as map
    let docvec: Vec<(Option<String>, DocType)> =
        ron::from_str(&serialized).expect("recreation of HashMap from RON failed");
    docvec
}

/// Load comments map from file or generate new from source
/// # Errors
/// - if neither files exist, or
/// - on an IO or File error. (eg. corruption)
///
/// # Panics
/// If the conversion from string to docmap fails.
fn load_saved_docvec_or_gather_new(
    comments_path: &Path,
    path_to_source: &Path,
) -> Result<Vec<(Option<String>, DocType)>, ()> {
    if comments_path.exists() {
        let docvec = load_saved_comments(comments_path);
        println!("Loaded docs form saved file.");
        return Ok(docvec);
    }

    if path_to_source.exists() {
        let docvec = read_file_to_vec(path_to_source);
        println!("Gathered docs from source file.");
        return Ok(docvec);
    }

    // Neither exist:
    Err(())
}

fn xml_to_src_file(path: &Path) {
    let generated = create_events_from_xml("xml/Event.xml");
    let buf = generated.as_bytes();

    let mut source_file = File::create(path).expect("error opening source file");

    source_file
        .write_all(buf)
        .expect("error while writing to source file");
}

fn xml_to_src_stdout() {
    let generated_src = create_events_from_xml("xml/Event.xml");
    let buf = generated_src.as_bytes();

    // acquire lock on stdout and write all
    let mut stdout = std::io::stdout().lock();
    stdout
        .write_all(buf)
        .expect("stdout should not be interrupted while writing");
}

pub fn main() {
    let args: Args = argh::from_env();

    // File names:
    let source_file_name = "identify.rs";
    let comments_file_name = "saved_manual_docs.ron";

    // Assumes being run from atspi crate root
    let crate_root = Path::new("./");
    let src_path = Path::new("src/");

    // The program expects one argument at a time.
    match args {
        // '-f' | '--regen_file' regenerate from xml. write to source file.
        Args { regen_file: true, .. } => {
            print!("Writing source to file.. ");
            let path = crate_root.join(src_path).join(source_file_name);
            xml_to_src_file(&path);
            println!("done.");
        }

        // '-r' / '--regen' : regenerate from xml to stidout
        Args { regen_stdout: true, .. } => {
            xml_to_src_stdout();
        }

        // '-s' | '--save' : save doc-commnents to file
        Args { docs_file: true, .. } => {
            let path_to_source = crate_root.join(src_path).join(source_file_name);
            print!("Gathering docs.. ");
            let docvec = if path_to_source.exists() {
                read_file_to_vec(&path_to_source)
            } else {
                eprintln!("Source file does not exist");
                std::process::exit(0);
            };

            print!("saving.. ");
            let path = crate_root.join(comments_file_name);
            write_map_to_file(&docvec, &path);
            println!("done.");
        }

        // '-o' | '--docs-stdout' : write docs to stdout
        Args { docs_stdout: true, .. } => {
            let comments_path = crate_root.join(comments_file_name);
            let source_path = crate_root.join(src_path).join(source_file_name);
            let Ok(docvec) = load_saved_docvec_or_gather_new(&comments_path, &source_path)  else {
                eprintln!("could not load saved doc commnts, nor extract new from source.");
                std::process::exit(0);
            };
            write_docs_to_stdout(&docvec);
        }

        // '-i' | '--insert' reinstate docs in soruce file
        Args { insert: true, .. } => {
            let path_to_source = crate_root.join(src_path).join(source_file_name);
            let comments_path = crate_root.join(comments_file_name);
            if comments_path.exists() {
                let docvec = load_saved_comments(&comments_path);
                reinstate_docs(&path_to_source, docvec);
            } else {
                eprintln!("comments save file does nt exist.");
            }
        }
        _ => println!("unsupported combination of switches"),
    }
}
