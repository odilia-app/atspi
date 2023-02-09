use zbus::{
	zvariant::{
	    Basic, ObjectPath, Signature, ARRAY_SIGNATURE_CHAR, DICT_ENTRY_SIG_END_CHAR,
			    DICT_ENTRY_SIG_START_CHAR, STRUCT_SIG_END_CHAR, STRUCT_SIG_START_CHAR, VARIANT_SIGNATURE_CHAR,
	},
};
use atspi_codegen::*;

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
		}.to_string()
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
            'h' => (if input {
                "zbus::zvariant::Fd"
            } else {
                "zbus::zvariant::OwnedFd"
            })
            .into(),
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
	iface.name().split('.').next_back().expect("An interface must have a period in its name.").to_string()
}

fn into_rust_enum_str<S>(string: S) -> String 
	where S: Into<String> {
	// needed to escape the uUShadeEvent
	// make sure Count is its own word
	// make sure Width is its own word
	string.into()
		.replace("uU", "UU")
		.replace("count", "Count")
		.replace("width", "Width")
    //.replace("AddAccessible", "Add")
    //.replace("RemoveAccessible", "Remove")
}

fn events_ident<S>(string: S) -> String
	where S: Into<String> {
	let mut sig_name_event_str = string.into();
	sig_name_event_str.push_str("Events");
	into_rust_enum_str(sig_name_event_str)
}
fn event_ident<S>(string: S) -> String
	where S: Into<String> {
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
	
	format!("
		#[must_use]
		pub fn {function_name}(&self) -> {rust_type} {{
			self.0.{inner_name}()
		}}
	")
}

fn generate_sub_enum_from_interface(interface: &Interface) -> String {
  let last_after_period = iface_name(interface);
  match last_after_period.as_str() {
    "Cache" => "CacheEvents",
    "Socket" => "AvailableEvent",
    "Registry" => "EventListenerEvents",
    // this covers all other cases like Document, Object, etc.
    _ => "EventInterfaces",
  }.to_string()
  
}

fn generate_enum_variant_from_interface(interface: &Interface) -> String {
  // this will get "Object" in `org.a11y.atspi.Event.Object`,
  // or "Cache" in `org.a11y.atspi.Cache`.
  let last_after_period = iface_name(interface);
  match last_after_period.as_str() {
    "Cache" => "Cache",
    "Socket" => "Available",
    "Registry" => "Listener",
    // this covers all other cases like Document, Object, etc.
    _ => "Interfaces",
  }.to_string()
}

fn generate_try_from_event_impl_match_statement(signal: &Signal, interface: &Interface) -> String {
	let mod_name = iface_name(interface);
  let event_variant = generate_enum_variant_from_interface(interface);
  let sub_enum = generate_sub_enum_from_interface(interface);
	let name_ident = iface_to_enum_name(interface);
	let name_ident_plural = events_ident(name_ident);
	let sig_name = into_rust_enum_str(signal.name());
  let interface_name = iface_name(interface);
  match interface_name.as_str() {
    "Cache" => {
      // replace AddAccessible with Add.
      // this is because the struct itself is named AddAccessibleEvent, so there is no need for it to be specified fully in the outer enum.
      // for example CacheEvents::AddAccessible(AddAccessibleEvent); this is shortened to CacheEvents::Add(_) for convenience.
      let sig_name = sig_name.replace("Accessible", "");
      format!("if let Event::{event_variant}({sub_enum}::{sig_name}(inner_event)) = event {{")
    },
    "Registry" => {
      // add "Event" to the beginning of the sub_enum, this is beacuase it should be EventListenerEvents::*
      let sig_name = sig_name.replace("EventListener", "");
      format!("if let Event::{event_variant}({sub_enum}::{sig_name}(inner_event)) = event {{")
    },
    "Socket" => {
      format!("if let Event::{event_variant}(inner_event) = event {{")
    },
    _ => format!("if let Event::{event_variant}({sub_enum}::{mod_name}({name_ident_plural}::{sig_name}(inner_event))) = event {{")
  }
}

fn generate_match_rule_vec_impl(interface: &Interface) -> String {
	let iface_name = iface_to_enum_name(interface);
	let enum_name = events_ident(iface_name);
	let signal_variant_match_rules = interface.signals()
		.iter()
		.map(|signal| {
			let sig_event_name = event_ident(signal.name());
			format!("<{sig_event_name} as HasMatchRule>::match_rule()?")
		})
		.collect::<Vec<String>>()
		.join(",\n");
	format!("	impl HasMatchRules for {enum_name} {{
		fn match_rules() -> Result<Vec<zbus::MatchRule<'static>>, AtspiError> {{
			Ok(vec![
				{signal_variant_match_rules}
			])
		}}
	}}")
}
fn generate_match_rule_impl(signal: &Signal, interface: &Interface) -> String {
	let sig_name_event = event_ident(signal.name());
	let member_string = signal.name();
	let iface_long_name = interface.name();
	format!("	impl HasMatchRule for {sig_name_event} {{
		const INTERFACE: &'static str = \"{iface_long_name}\";
		const MEMBER: &'static str = \"{member_string}\";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {{
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}}
	}}")
}

fn generate_try_from_event_impl(signal: &Signal, interface: &Interface) -> String {
	let sig_name_event = event_ident(signal.name());
  let matcher = generate_try_from_event_impl_match_statement(signal, interface);
  format!("	impl TryFrom<Event> for {sig_name_event} {{
		type Error = AtspiError;
		fn try_from(event: Event) -> Result<Self, Self::Error> {{
       {matcher}
				Ok(inner_event)
			}} else {{
				Err(AtspiError::Conversion(\"Invalid type\"))
			}}
		}}
	}}")
}

fn generate_impl_from_signal(signal: &Signal, interface: &Interface) -> String {
	let sig_name_event = event_ident(signal.name());
  let try_from_event_impl = generate_try_from_event_impl(signal, interface);
	let functions = signal.args()
			.iter()
			.enumerate()
			.filter_map(|(i, arg)| {
					let func_name = i.try_into();
					let arg_name = arg.name();
					match (func_name, arg_name) {
						(Ok(ok_func_name), Some(_)) => Some(generate_fn_for_signal_item(arg, ok_func_name)),
						_ => None
					}
			})
			.collect::<Vec<String>>()
			.join("\n");

	format!("
	impl {sig_name_event} {{
		{functions}
	}}
{try_from_event_impl}")
}

fn iface_to_enum_name(interface: &Interface) -> String {
	interface.name().split('.').next_back().expect("Interface must contain a period").to_string()
}

fn generate_struct_from_signal(signal: &Signal) -> String {
	let sig_name_event = event_ident(signal.name());
	format!("
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct {sig_name_event}(pub(crate) AtspiEvent);
	")
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
	format!("				\"{raw_signal_name}\" => Ok({enum_name}::{enum_signal_name}({signal_struct_name}(ev))),")
}

fn generate_try_from_atspi_event(iface: &Interface) -> String {
	let iname = iface_name(iface);
	let error_str = format!("No matching member for {iname}");
	let impl_for_name = events_ident(&iname);
	let member_conversions = iface.signals()
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
	let structs = iface.signals()
			.iter()
			.map(|signal| generate_struct_from_signal(signal))
			.collect::<Vec<String>>()
			.join("\n");
	let impls = iface.signals()
			.iter()
			.map(|signal| generate_impl_from_signal(signal, iface))
			.collect::<Vec<String>>()
			.join("\n");
	let try_froms = generate_try_from_atspi_event(iface);
	let match_rule_impls = iface.signals()
		.iter()
		.map(|signal| generate_match_rule_impl(signal, iface))
		.collect::<Vec<String>>()
		.join("\n");
	let match_rule_vec_impl = generate_match_rule_vec_impl(iface);
	format!("
#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod {mod_name} {{
	use atspi_macros::TrySignify;
	use crate::{{
		Event,
		error::AtspiError,
		events::{{AtspiEvent, GenericEvent, EventInterfaces, HasMatchRule, HasMatchRules}},
		signify::Signified,
	}};
	use zbus;
	use zbus::zvariant::OwnedValue;
	{enums}
	{match_rule_vec_impl}
	{structs}
	{impls}
	{try_froms}
	{match_rule_impls}
}}
	")
}

fn generate_enum_from_iface(iface: &Interface) -> String {
	let name_ident = iface_to_enum_name(iface);
	let name_ident_plural = events_ident(name_ident);
	let signal_quotes = iface.signals()
			.into_iter()
			.map(generate_variant_from_signal)
			.collect::<Vec<String>>()
			.join("\n");
	format!("
	#[derive(Clone, Debug)]
	#[non_exhaustive]
	pub enum {name_ident_plural} {{
{signal_quotes}
	}}
	")
}

fn get_signal_names_from_interfaces(interfaces: Vec<&Interface>) -> String {
	interfaces
		.iter()
		.map(|iface| {
			let mut signal_events_names = iface.signals()
				.iter()
				.map(|signal| signal.name().to_owned() + "Event")
				.collect::<Vec<String>>();
			// if there is only one event, this is probably doesn't need the interface event on top. This is because no enum should be necessary to contain a single type.
			if signal_events_names.len() != 1 {
				let interface_ending = generate_sub_enum_from_interface(iface);
				signal_events_names.push(interface_ending);
			}
			signal_events_names.join(",")
		})
		.collect::<Vec<String>>()
		.join(",")
}

fn create_try_from_event_impl_from_xml(file_name: &str) -> String {
	let xml_file = std::fs::File::open(file_name).expect("Cannot read file");
	let data: Node = Node::from_reader(&xml_file).expect("Cannot deserialize file");
	let event_imports = get_signal_names_from_interfaces(data.interfaces());
	let iface_data = data.interfaces()
		.iter()
		.map(|iface| 
			iface.signals()
				.iter()
				.map(|signal| generate_try_from_event_impl(signal, iface))
				.collect::<Vec<String>>()
				.join("\n")
		)
		.collect::<Vec<String>>()
		.join("\n");
	format!("use crate::events::{{{}}};\n{}", event_imports, iface_data)
}

fn create_events_from_xml(file_name: &str) -> String {
	let xml_file = std::fs::File::open(file_name).expect("Cannot read file");
	let data: Node = Node::from_reader(&xml_file).expect("Cannot deserialize file");
	let iface_data = data.interfaces()
		.iter()
		.map(|iface| generate_mod_from_iface(iface))
		.collect::<Vec<String>>()
		.join("\n");
  format!("
  use crate::AtspiError;
  use crate::Event;
	{iface_data}
  ")
}

fn main() {
	println!("{}", create_events_from_xml("xml/Event.xml"));
	//println!("use crate::events::{{{}}};", get_signal_names_from_xml("xml/Cache.xml"));
	println!("{}", create_try_from_event_impl_from_xml("xml/Cache.xml"));
	println!("{}", create_try_from_event_impl_from_xml("xml/Registry.xml"));
	println!("{}", create_try_from_event_impl_from_xml("xml/Socket.xml"));
}
