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
enum AtspiEventInnerName2 {
    Kind,
    Detail1,
    Detail2,
    AnyData,
    Properties,
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
impl ToString for AtspiEventInnerName2 {
    fn to_string(&self) -> String {
        match self {
            Self::Kind => "kind",
            Self::Detail1 => "detail1",
            Self::Detail2 => "detail2",
            Self::AnyData => "any_data",
            Self::Properties => "properties",
        }
        .to_string()
    }
}

#[derive(Debug)]
enum ConversionError {
    FunctionAlreadyCreatedFor,
    UnknownItem,
}
impl TryFrom<usize> for AtspiEventInnerName2 {
    type Error = ConversionError;

    fn try_from(from: usize) -> Result<Self, Self::Error> {
        match from {
            0 => Ok(Self::Kind),
            1 => Ok(Self::Detail1),
            2 => Ok(Self::Detail2),
            3 => Ok(Self::AnyData),
            4 => Ok(Self::Properties),
            _ => Err(ConversionError::UnknownItem),
        }
    }
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
            <&str>::SIGNATURE_CHAR => (if input || as_ref { "String" } else { "String" }).into(),
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
                    "zbus::zvariant::OwnedValue"
                } else {
                    "zbus::zvariant::OwnedValue"
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

/// Takes the interface name, eg: 'org.a11y/atspi.Event.Mouse`
/// and return the last segment as String. In this example `Mouse`.
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
    //.replace("AddAccessible", "Add")
    //.replace("RemoveAccessible", "Remove")
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

fn generate_struct_literal_conversion_for_signal_item(signal_item: &Arg, inner_event_name: AtspiEventInnerName2) -> String {
    if signal_item.name().is_none() {
        return String::new();
    }
    // unwrap is safe due to check
    let field_name = signal_item.name().expect("No name for arg");
    let msg_field_name = inner_event_name.to_string();

    format!("{field_name}: body.{msg_field_name}")
}
fn generate_reverse_struct_literal_conversion_for_signal_item(signal_item: &Arg, inner_event_name: AtspiEventInnerName2) -> String {
    let rust_type = to_rust_type(signal_item.ty(), true, true);
    let value = if signal_item.name().is_none() {
      if rust_type == "zbus::zvariant::OwnedValue" {
        format!("zbus::zvariant::Value::U8(0).into()")
      } else {
        format!("{rust_type}::default()")
      }
    } else {
      let field_name = signal_item.name().expect("No name for arg");
      format!("event.{field_name}")
    };
    // unwrap is safe due to check
    let msg_field_name = inner_event_name.to_string();

    format!("{msg_field_name}: {value}")
}
fn generate_field_for_signal_item(signal_item: &Arg) -> String {
    if signal_item.name().is_none() {
        return String::new();
    }
    // unwrap is safe due to check
    let function_name = signal_item.name().expect("No name for arg");
    let rust_type = to_rust_type(signal_item.ty(), true, true);

    format!("   pub {function_name}: {rust_type},
")
}

fn generate_enum_variant_from_interface(interface: &Interface) -> String {
    // this will get "Object" in `org.a11y.atspi.Event.Object`,
    // or "Cache" in `org.a11y.atspi.Cache`.
    let last_after_period = iface_name(interface);
    match last_after_period.as_str() {
        "Cache" => "Cache".to_string(),
        "Socket" => "Available".to_string(),
        "Registry" => "Listener".to_string(),
        // this covers all other cases like Document, Object, etc.
        generic_event => format!("{generic_event}"),
    }
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
    _ => format!("if let Event::{event_variant}({sub_enum}::{sig_name}(inner_event)) = event {{")
  }
}

fn generate_try_from_event_impl(signal: &Signal, interface: &Interface) -> String {
    let sig_name_event = event_ident(signal.name());
    let matcher = generate_try_from_event_impl_match_statement(signal, interface);
    format!(
        "#[rustfmt::skip]
    impl TryFrom<Event> for {sig_name_event} {{
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {{
       {matcher}
				Ok(inner_event)
			}} else {{
				Err(AtspiError::Conversion(\"Invalid type\"))
			}}
		}}
	}}
    "
    )
}

fn generate_impl_from_signal(signal: &Signal, interface: &Interface) -> String {
    let try_from_event_impl = generate_try_from_event_impl(signal, interface);
    let generic_event_impl = generate_generic_event_impl(signal, interface);

    format!(
        "
    {generic_event_impl}
    {try_from_event_impl}
    "
    )
}

fn generate_sub_enum_from_interface(interface: &Interface) -> String {
    let last_after_period = iface_name(interface);
    match last_after_period.as_str() {
        "Cache" => "CacheEvents".to_string(),
        "Socket" => "AvailableEvent".to_string(),
        "Registry" => "EventListenerEvents".to_string(),
        // this covers all other cases like Document, Object, etc.
        generic_event => format!("{generic_event}Events"),
    }
}

fn iface_to_enum_name(interface: &Interface) -> String {
    interface
        .name()
        .split('.')
        .next_back()
        .expect("Interface must contain a period")
        .to_string()
}

fn generate_signal_associated_example(mod_name: &str, signal_event_name: &str, signal_name: &str, interface: &str) -> String {
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
    /// use atspi::Event;
    /// use atspi::identify::{mod_name}::{signal_event_name};
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {{
    ///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
    ///     let mut events = atspi.event_stream();
		/// #   atspi.register_event::<{signal_event_name}>().await.unwrap();
    ///     std::pin::pin!(&mut events);
    /// #   let output = std::process::Command::new(\"busctl\")
    /// #       .arg(\"--user\")
    /// #       .arg(\"call\")
    /// #       .arg(\"org.a11y.Bus\")
    /// #       .arg(\"/org/a11y/bus\")
    /// #       .arg(\"org.a11y.Bus\")
    /// #       .arg(\"GetAddress\")
    /// #       .output()
    /// #       .unwrap();
    /// #    let addr_string = String::from_utf8(output.stdout).unwrap();
    /// #    let addr_str = addr_string
    /// #        .strip_prefix(\"s \\\"\")
    /// #        .unwrap()
    /// #        .trim()
    /// #        .strip_suffix('\"')
    /// #        .unwrap();
    /// #   let mut base_cmd = std::process::Command::new(\"busctl\");
    /// #   let thing = base_cmd
    /// #       .arg(\"--address\")
    /// #       .arg(addr_str)
    /// #       .arg(\"emit\")
    /// #       .arg(\"/org/a11y/atspi/accessible/null\")
    /// #       .arg(\"{interface}\")
    /// #       .arg(\"{signal_name}\")
    /// #       .arg(\"siiva{{sv}}\")
    /// #       .arg(\"\")
    /// #       .arg(\"0\")
    /// #       .arg(\"0\")
    /// #       .arg(\"i\")
    /// #       .arg(\"0\")
    /// #       .arg(\"0\")
    /// #       .output()
    /// #       .unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {{
    ///         if let Ok(event) = {signal_event_name}::try_from(ev) {{
		/// #          break;
		///            // do something with the specific event you've received
		///         }} else {{ continue }};
    ///     }}
    /// }}
    /// ```
    {STRIPPER_IGNORE_STOP}"
    )
}

fn generate_struct_from_signal(mod_name: &str, signal: &Signal, iface: &Interface) -> String {
    let sig_name_event = event_ident(signal.name());
    let interface_name = iface.name();
    let example = generate_signal_associated_example(mod_name, &sig_name_event, &signal.name(), &interface_name);
    let fields = signal
        .args()
        .iter()
        .map(|arg| {
            generate_field_for_signal_item(arg)
        })
        .collect::<Vec<String>>()
        .join("");
    format!(
        "
    {example}
	#[derive(Debug, PartialEq, Clone)]
	pub struct {sig_name_event} {{
    pub item: crate::events::Accessible,
{fields}
}}
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
        "				\"{raw_signal_name}\" => Ok({enum_name}::{enum_signal_name}(ev.try_into()?)),"
    )
}

fn generate_try_from_atspi_event(iface: &Interface) -> String {
    let iname = iface_name(iface);
    let error_str = format!("No matching member for {iname}");
    let impl_for_name = events_ident(&iname);
		let enum_name = iface_to_enum_name(iface);
    let member_conversions = iface
        .signals()
        .iter()
        .map(|signal| match_arm_for_signal(&iname, signal))
        .collect::<Vec<String>>()
        .join("\n");
    format!("
	impl From<{impl_for_name}> for Event {{
		fn from(event_enum: {impl_for_name}) -> Self {{
        Event::{enum_name}(event_enum)
		}}
	}}
	impl TryFrom<&zbus::Message> for {impl_for_name} {{
		type Error = AtspiError;

		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {{
			let member = ev.member()
				.ok_or(AtspiError::MemberMatch(\"Event without member\".into()))?;
			match member.as_str() {{
{member_conversions}
				_ => Err(AtspiError::MemberMatch(\"{error_str}\".into())),
			}}
		}}
	}}
	")
}
fn generate_try_from_event_body(iface: &Interface, signal: &Signal) -> String {
    let iname = signal.name();
    let error_str = format!("No matching member for {iname}");
    let impl_for_name = event_ident(iname);
		let iface_variant = iface_name(iface);
		let enum_variant = events_ident(iface_variant.clone());
		let event_variant = into_rust_enum_str(iname);
    let reverse_signal_conversion_lit = signal
        .args()
        .iter()
        .enumerate()
        .filter_map(|(i, arg)| {
            let Ok(field_name) = i.try_into() else {
              return None;
            };
            Some(generate_reverse_struct_literal_conversion_for_signal_item(arg, field_name))
        })
        .collect::<Vec<String>>()
        .join(", ");
    let signal_conversion_lit = signal
        .args()
        .iter()
        .enumerate()
        .filter_map(|(i, arg)| {
            if arg.name().is_none() {
              return None;
            }
            let Ok(field_name) = i.try_into() else {
              return None;
            };
            Some(generate_struct_literal_conversion_for_signal_item(arg, field_name))
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!("
	impl From<{impl_for_name}> for {enum_variant} {{
		fn from(specific_event: {impl_for_name}) -> Self {{
			{enum_variant}::{event_variant}(specific_event)
		}}
	}}
	impl From<{impl_for_name}> for Event {{
		fn from(specific_event: {impl_for_name}) -> Self {{
			Event::{iface_variant}(specific_event.into())
		}}
	}}
  impl TryFrom<{impl_for_name}> for zbus::Message {{
    type Error = AtspiError;
    fn try_from(event: {impl_for_name}) -> Result<Self, Self::Error> {{
      Ok(zbus::MessageBuilder::signal(
						event.item.path,
						<{impl_for_name} as GenericEvent>::DBUS_INTERFACE,
						<{impl_for_name} as GenericEvent>::DBUS_MEMBER,
					)?
					.sender(event.item.name)?
					.build(&((EventBodyOwned {{
					{reverse_signal_conversion_lit}
					}}),))?
      )
    }}
  }}
  impl TryFrom<&zbus::Message> for {impl_for_name} {{
    type Error = AtspiError;
    fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {{
			let body = msg.body::<EventBodyOwned>()?;
			let item = msg.try_into()?;
      Ok(Self {{ item, {signal_conversion_lit} }})
    }}
  }}
	")
}

fn generate_match_rule_vec_impl(interface: &Interface) -> String {
    let iface_long_name = interface.name();
    let iface_name = iface_to_enum_name(interface);
    let enum_name = events_ident(iface_name);
    let match_rule = zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(iface_long_name).expect("Unable to use an interface: {iface_long_name}")
				.build();
    let match_rule_str = match_rule.to_string();
    format!(
        "	impl HasMatchRule for {enum_name} {{
      const MATCH_RULE_STRING: &'static str = \"{match_rule_str}\";
	}}"
    )
}

fn generate_registry_event_enum_impl(interface: &Interface) -> String {
    let iface_prefix = iface_name(interface);
    let iface_name = iface_to_enum_name(interface);
    let enum_name = events_ident(iface_name);
    format!(
        "	impl HasRegistryEventString for {enum_name} {{
		const REGISTRY_EVENT_STRING: &'static str = \"{iface_prefix}:\";
	}}"
    )
}
fn generate_registry_event_impl(signal: &Signal, interface: &Interface) -> String {
    let sig_name_event = event_ident(signal.name());
    let member_string = signal.name();
    let iface_prefix = iface_name(interface);
    format!(
        "	/*impl HasRegistryEventString for {sig_name_event} {{
		const REGISTRY_EVENT_STRING: &'static str = \"{iface_prefix}:{member_string}\";
	}}*/"
    )
}

fn generate_match_rule_impl(signal: &Signal, interface: &Interface) -> String {
    let sig_name_event = event_ident(signal.name());
    let member_string = signal.name();
    let iface_long_name = interface.name();
    let match_rule = zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(iface_long_name).expect("Unable to use an interface: {iface_long_name}")
				.member(member_string).expect("Unable to use a member: {member_string}")
				.build();
    let match_rule_str = match_rule.to_string();
    format!(
        "	/*impl HasMatchRule for {sig_name_event} {{
      const MATCH_RULE_STRING: &'static str = \"{match_rule_str}\";
	}}*/"
    )
}
// TODO
fn generate_generic_event_impl(signal: &Signal, interface: &Interface) -> String {
    let iface_prefix = iface_name(interface);
    let sig_name_event = event_ident(signal.name());
    let member_string = signal.name();
    let iface_long_name = interface.name();
    let match_rule = zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(iface_long_name).expect("Unable to use an interface: {iface_long_name}")
				.member(member_string).expect("Unable to use a member: {member_string}")
				.build();
    let match_rule_str = match_rule.to_string();
    let raw_member_name = signal.name();
    let raw_interface_name = interface.name();
    format!(
        "	impl GenericEvent for {sig_name_event} {{
      const DBUS_MEMBER: &'static str = \"{raw_member_name}\";
      const DBUS_INTERFACE: &'static str = \"{raw_interface_name}\";
      const MATCH_RULE_STRING: &'static str = \"{match_rule_str}\";
      const REGISTRY_EVENT_STRING: &'static str = \"{iface_prefix}:\";
    fn sender(&self) -> UniqueName<'_> {{
      self.item.name.clone().into()
    }}
    fn path<'a>(&self) -> ObjectPath<'_> {{
      self.item.path.clone().into()
    }}
	}}"
    )
}

fn generate_mod_from_iface(iface: &Interface) -> String {
    let mod_name = iface_name(iface).to_lowercase();
    let enums = generate_enum_from_iface(iface);
    let structs = iface
        .signals()
        .iter()
        .map(|signal| generate_struct_from_signal(&mod_name, signal, &iface))
        .collect::<Vec<String>>()
        .join("\n");
    let impls = iface
        .signals()
        .iter()
        .map(|signal| generate_impl_from_signal(signal, iface))
        .collect::<Vec<String>>()
        .join("\n");
    let try_from_atspi = generate_try_from_atspi_event(iface);
    let from_event_body = iface
        .signals()
        .iter()
        .map(|signal| generate_try_from_event_body(iface, signal))
        .collect::<Vec<String>>()
        .join("\n");
    let registry_event_enum_impl = generate_registry_event_enum_impl(iface);
    let registry_event_impls = iface
        .signals()
        .iter()
        .map(|signal| generate_registry_event_impl(signal, iface))
        .collect::<Vec<String>>()
        .join("\n");
    let match_rule_impls = iface
        .signals()
        .iter()
        .map(|signal| generate_match_rule_impl(signal, iface))
        .collect::<Vec<String>>()
        .join("\n");
    let match_rule_vec_impl = generate_match_rule_vec_impl(iface);
    format!(
        "
#[allow(clippy::module_name_repetitions)]
{STRIPPER_IGNORE_START}
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
{STRIPPER_IGNORE_STOP}
pub mod {mod_name} {{
	use crate::{{
        Event,
		error::AtspiError,
		events::{{GenericEvent, HasMatchRule, HasRegistryEventString, EventBodyOwned}},
	}};
	use zbus;
	use zbus::zvariant::ObjectPath;
  use zbus::names::UniqueName;
	{enums}
	{match_rule_vec_impl}
	{structs}
	{impls}
	{try_from_atspi}
  {from_event_body}
	{match_rule_impls}
  {registry_event_impls}
  {registry_event_enum_impl}
}}
	"
    )
}

fn generate_enum_associated_example(mod_name: &str, signal_event_name: &str, signal_name: &str, interface: &str, iface_name: &str) -> String {
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
    /// use atspi::Event;
    /// use atspi::identify::{mod_name}::{signal_event_name};
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {{
    ///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
    ///     let mut events = atspi.event_stream();
		/// #   atspi.register_event::<{signal_event_name}>().await.unwrap();
    ///     std::pin::pin!(&mut events);
    /// #   let output = std::process::Command::new(\"busctl\")
    /// #       .arg(\"--user\")
    /// #       .arg(\"call\")
    /// #       .arg(\"org.a11y.Bus\")
    /// #       .arg(\"/org/a11y/bus\")
    /// #       .arg(\"org.a11y.Bus\")
    /// #       .arg(\"GetAddress\")
    /// #       .output()
    /// #       .unwrap();
    /// #    let addr_string = String::from_utf8(output.stdout).unwrap();
    /// #    let addr_str = addr_string
    /// #        .strip_prefix(\"s \\\"\")
    /// #        .unwrap()
    /// #        .trim()
    /// #        .strip_suffix('\"')
    /// #        .unwrap();
    /// #   let mut base_cmd = std::process::Command::new(\"busctl\");
    /// #   let thing = base_cmd
    /// #       .arg(\"--address\")
    /// #       .arg(addr_str)
    /// #       .arg(\"emit\")
    /// #       .arg(\"/org/a11y/atspi/accessible/null\")
    /// #       .arg(\"{interface}\")
    /// #       .arg(\"{signal_name}\")
    /// #       .arg(\"siiva{{sv}}\")
    /// #       .arg(\"\")
    /// #       .arg(\"0\")
    /// #       .arg(\"0\")
    /// #       .arg(\"i\")
    /// #       .arg(\"0\")
    /// #       .arg(\"0\")
    /// #       .output()
    /// #       .unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {{
    ///          if let Ok(event) = {iface_name}::try_from(ev) {{
		/// #            break;
		///              // do things with your event here
		///          }}  else {{ continue }};
    ///     }}
    /// }}
    /// ```
    {STRIPPER_IGNORE_STOP}"
    )
}

fn generate_enum_from_iface(iface: &Interface) -> String {
    let mod_name = iface_name(iface).to_lowercase();
    let name_ident = iface_to_enum_name(iface);
		let signal = iface.signals().into_iter().next().expect("Could not get a signal to create example code.");
		let sig_name_event = event_ident(signal.name());
		let interface_name = iface.name();
    let example = generate_enum_associated_example(&mod_name, &sig_name_event, &signal.name(), &interface_name, &name_ident);
    let name_ident_plural = events_ident(name_ident);
    let signal_quotes = iface
        .signals()
        .into_iter()
        .map(generate_variant_from_signal)
        .collect::<Vec<String>>()
        .join("");
    format!(
        "
    {example}
	#[derive(Clone, Debug)]
	pub enum {name_ident_plural} {{
{signal_quotes}
	}}
	"
    )
}

pub fn get_signal_names_from_interfaces(interfaces: Vec<&Interface>) -> String {
    interfaces
        .iter()
        .map(|iface| {
            let mut signal_events_names = iface
                .signals()
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

pub fn create_try_from_event_impl_from_xml(file_name: &str) -> String {
    let xml_file = std::fs::File::open(file_name).expect("Cannot read file");
    let data: Node = Node::from_reader(&xml_file).expect("Cannot deserialize file");
    let event_imports = get_signal_names_from_interfaces(data.interfaces());
    let iface_data = data
        .interfaces()
        .iter()
        .map(|iface| {
            iface
                .signals()
                .iter()
                .map(|signal| generate_try_from_event_impl(signal, iface))
                .collect::<Vec<String>>()
                .join("\n")
        })
        .collect::<Vec<String>>()
        .join("\n");
    format!("use crate::events::{{{event_imports}}};\n{iface_data}\n")
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
        .join("\n\n");
    format!(
        "
    use crate::AtspiError;
    {module_level_doc}\n
    {iface_data}"
    )
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
struct CmtOrItem {
    // distance to next 'identifier' / string we can associate the docs with
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
    CmtOrItem(CmtOrItem),
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
enum ParseState {
    #[default]
    None,
    CmtOrItem,
    ModuleLevel,
    IgnoreBlock(Box<ParseState>),
}

/// Collects from the source file into a Vec.
/// HashMap does not (necessarilly) preserve order of insertion.  Hence Vec.
fn read_file_to_vec(src: &Path) -> Vec<(Option<String>, DocType)> {
    let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
    let mut src = OpenOptions::new()
        .read(true)
        .open(src)
        .expect("could not open save file");

    let mut buf = String::new();
    let n = src.read_to_string(&mut buf).expect("could not read source to buf");
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
                line if line.trim().starts_with("///") | line.trim().starts_with("//") => {
                    if line.contains(STRIPPER_IGNORE_START) {
                        docstate = ParseState::IgnoreBlock(Box::new(ParseState::None));
                        continue;
                    }
                    docstate = ParseState::CmtOrItem;
                    docblock.push(line.into());
                    continue;
                }
                _ => continue,
            },

            ParseState::ModuleLevel => {
                if line.contains(STRIPPER_IGNORE_START) {
                    docstate = ParseState::IgnoreBlock(Box::new(ParseState::ModuleLevel));
                    continue;
                }
                gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
                continue;
            }

            ParseState::CmtOrItem => {
                if line.contains(STRIPPER_IGNORE_START) {
                    docstate = ParseState::IgnoreBlock(Box::new(ParseState::CmtOrItem));
                    counter += 1;
                    continue;
                }
                gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
                continue;
            }

            ParseState::IgnoreBlock(ref origin) => {
                match **origin {
                    ParseState::None => {
                        if line.contains(STRIPPER_IGNORE_STOP) {
                            docstate = (**origin).clone();
                        }
                    }
                    ParseState::CmtOrItem | ParseState::ModuleLevel => {
                        counter += 1;
                        if line.contains(STRIPPER_IGNORE_STOP) {
                            docstate = (**origin).clone();
                        }
                    }
                    _ => unreachable!(),
                }
                continue;
            }
        }
    }
    docvec
}

fn gather_module_level_doc_line(
    line: &str,
    docblock: &mut Vec<String>,
    docstate: &mut ParseState,
    docvec: &mut Vec<(Option<String>, DocType)>,
) {
    // As long as `line` starts with '//' it is still comment. a mixed block is also a block.
    if line.trim().starts_with("//") {
        docblock.push(line.into());
    } else {
        *docstate = ParseState::None;
        let dt = DocType::Module(ModuleLevel { doc: docblock.clone() });
        docblock.clear();
        docvec.push((None, dt));
    }
}

fn gather_doc_or_cmt(
    line: &str,
    counter: &mut u8,
    docblock: &mut Vec<String>,
    docstate: &mut ParseState,
    docvec: &mut Vec<(Option<String>, DocType)>,
) {
    if line.trim().starts_with("//") {
        docblock.push(line.into());
    } else if line.trim().starts_with("#[") || line.trim().is_empty() {
        *counter += 1;
        return;
    } else if line.trim() == "{" || line.trim() == "}" {
        // A single curly brace is too common to uniquely reference to as a position.
        *docstate = ParseState::None;
        docblock.clear();
        *counter = 0;
        return;
    } else if !line.trim().is_empty() {
        let docitem = CmtOrItem { dist: *counter, doc: docblock.clone() };
        let dt = DocType::CmtOrItem(docitem);
        docvec.push((Some(line.trim().into()), dt));

        docblock.clear();
        *counter = 0;
        *docstate = ParseState::None;
    }
}

fn reinstate_docs(path: &Path, docvec: Vec<(Option<String>, DocType)>) {
    let mut source_string = String::new();
    let mut remains = docvec.clone();

    OpenOptions::new()
        .read(true)
        .open(path)
        .expect("could not open sources")
        .read_to_string(&mut source_string)
        .expect("could not read source file to string");

    // Create Vec<String>s from single String.
    let source_lines: Vec<String> = source_string.lines().map(|s| s.to_string()).collect();
    let mut source_and_doc_lines: Vec<String> = source_lines.clone();

    // For each key in `docvec`, look in `source_lines` for a line that contain that key.
    // if so, insert docs that point, honoring distance,
    for (k, v) in docvec {
        if k.is_none() {
            if let DocType::Module(ModuleLevel { ref doc }) = v {
                source_and_doc_lines.splice(0..0, doc.iter().cloned());
                remains.retain(|tup| *tup != (k.clone(), v.clone()));
                continue;
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
                    DocType::CmtOrItem(CmtOrItem { dist, ref doc }) => {
                        let i = idx - dist as usize;
                        source_and_doc_lines.splice(i..i, doc.iter().cloned());
                        remains.retain(|tup| *tup != (k.clone(), v.clone()));
                    }
                    _ => unreachable!("k == None implies ModuleLevel docs."),
                }
            }
        }
    }

    // collect all strings in vec, adding a newline to each but the last.
    let last = source_and_doc_lines.last().unwrap().clone();
    let len = source_and_doc_lines.len();
    let mut new_source: String = source_and_doc_lines[..len - 1]
        .iter()
        .map(|line| line.to_owned() + "\n")
        .collect();
    new_source += &last;

    // write string to source
    std::fs::write(path, new_source).expect("Unable to write file");

    if !remains.is_empty() {
        println!("The following items could not be reinstated:");
        println!("{remains:#?}");
        println!("Number of items not reinstated: {}", remains.len());
    }
}

/// Writes the serialized docs to the path
fn write_serialized_docs_to_file(docvec: &Vec<(Option<String>, DocType)>, path: &Path) {
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

fn generate_new_sources_main() -> String {
    let mut generated = String::new();
    generated.push_str(&create_events_from_xml("xml/Event.xml"));
    generated.push_str("use crate::Event;\n");
    generated.push_str(&create_try_from_event_impl_from_xml("xml/Cache.xml"));
    generated.push_str(&create_try_from_event_impl_from_xml("xml/Registry.xml"));
    generated.push_str(&create_try_from_event_impl_from_xml("xml/Socket.xml"));
    generated
}

fn xml_to_src_file(path: &Path) {
    let generated = generate_new_sources_main();
    let buf = generated.as_bytes();

    let mut source_file = File::create(path).expect("error opening source file");
    source_file
        .write_all(buf)
        .expect("error while writing to source file");
}

fn xml_to_src_stdout() {
    let generated = generate_new_sources_main();
    let buf = generated.as_bytes();

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
            write_serialized_docs_to_file(&docvec, &path);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test line parsing of module level docs, per line.
    #[test]
    fn module_level_space() {
        let line = "//! ";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::None;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();

        gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn module_level_preceding_spaces() {
        let line = "    //! ";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::ModuleLevel;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();

        gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn module_level_preceding_tab() {
        let line = "\t//! ";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::ModuleLevel;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();

        gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn module_level_preceding_characters() {
        let line = "shouldnotparse//! ";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::ModuleLevel;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();

        gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_ne!(docblock, v);
        assert_eq!(docblock, Vec::<String>::new())
    }

    #[test]
    fn module_level_heading() {
        let line = "//! # Heading";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::ModuleLevel;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();

        gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn module_level_comment() {
        let line = "//! // comment";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::ModuleLevel;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();

        gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn module_level_nospace() {
        let line = "//!nospace";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::ModuleLevel;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();

        gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn module_level_accept_comments() {
        let line = "// TODO";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::ModuleLevel;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();

        gather_module_level_doc_line(line, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    /// Test line parsing of comment level docs, per line.
    #[test]
    fn comment_level_empty_comment() {
        let line = "//";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn comment_level_empty_preceding_spaces() {
        let line = "      //";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn comment_level_empty_preceding_tab() {
        let line = "\t//";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn comment_level_empty_repeat() {
        let line = "//////////////"; // still a valid comment

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        let v: Vec<String> = vec![String::from(line)];
        assert_eq!(docblock, v);
    }

    #[test]
    fn comment_level_attribute() {
        let line = "#[SomeAttribute(attribute_param)]";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        assert_eq!(docblock, Vec::<String>::new());
        assert_eq!(counter, 1);
    }

    #[test]
    fn comment_level_newline() {
        let line = "\n";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        assert_eq!(docblock, Vec::<String>::new());
        assert_eq!(counter, 1);
    }

    #[test]
    fn comment_level_single_open_curly_brace() {
        let line = "{";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        assert_eq!(docblock, Vec::<String>::new());
        assert_eq!(counter, 0);
        assert_eq!(docstate, ParseState::None);
        assert!(docvec.is_empty());
    }

    #[test]
    fn comment_level_single_closing_curly_brace() {
        let line = "}";

        let mut docblock: Vec<String> = Vec::new();
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        assert_eq!(docblock, Vec::<String>::new());
        assert_eq!(counter, 0);
        assert_eq!(docstate, ParseState::None);
        assert!(docvec.is_empty());
    }

    #[test]
    fn comment_level_single_item() {
        let line = "pub struct Foo";

        // supposedly previously gathered comments
        let mut docblock: Vec<String> =
            vec![String::from("// Foobar"), String::from("// Touxdoux")];
        let mut docstate = ParseState::CmtOrItem;
        let mut docvec: Vec<(Option<String>, DocType)> = Vec::new();
        let mut counter = 0;

        gather_doc_or_cmt(line, &mut counter, &mut docblock, &mut docstate, &mut docvec);
        assert_eq!(docblock, Vec::<String>::new());
        assert_eq!(counter, 0);
        assert_eq!(docstate, ParseState::None);

        let docitem = CmtOrItem {
            dist: counter,
            doc: vec![String::from("// Foobar"), String::from("// Touxdoux")],
        };
        let dt = DocType::CmtOrItem(docitem);
        let dv: Vec<(Option<String>, DocType)> = vec![(Some(line.to_owned()), dt)];

        assert_eq!(docvec, dv);
    }

    #[test]
    fn ignore_block_gather_nothing() {
        let t = temp_file::with_contents(
            br#"
        // IgnoreBlock start
        /// # Examples
        ///
        /// ```
        /// use atspi::Event;
        /// # use std::time::Duration;
        /// use tokio_stream::StreamExt;
        ///
        /// #[tokio::main]
        /// async fn main() {}
        /// ```
        // IgnoreBlock stop  
        #[derive(Clone, Debug)]
        pub enum ObjectEvents {
        "#,
        );

        let empty: Vec<(Option<String>, DocType)> = Vec::new();

        let gathered = read_file_to_vec(t.path());
        assert_eq!(gathered, empty);
    }

    #[test]
    fn item_level_single_line_before_ignores() {
        let t = temp_file::with_contents(
            br#"
        /// Single line doc comment
        // IgnoreBlock start
        /// # Examples
        // IgnoreBlock stop  
        #[derive(Clone, Debug)]
        pub enum ObjectEvents {
        "#,
        );

        let line: Vec<String> = vec!["        /// Single line doc comment".to_string()];
        let dt: DocType = DocType::CmtOrItem(CmtOrItem { dist: 4, doc: line });
        let dt_single_line: Vec<(Option<String>, DocType)> =
            vec![(Some("pub enum ObjectEvents {".to_string()), dt)];

        let gathered = read_file_to_vec(t.path());
        assert_eq!(gathered, dt_single_line);
    }

    #[test]
    fn reinstale_single_line_before_ignores() {
        let original = temp_file::with_contents(
            br#"
        /// Single line doc comment
        // IgnoreBlock start
        /// # Examples
        // IgnoreBlock stop  
        #[derive(Clone, Debug)]
        pub enum ObjectEvents {
            "#,
        );

        let generated = temp_file::with_contents(
            br#"
        // IgnoreBlock start
        /// # Examples
        // IgnoreBlock stop  
        #[derive(Clone, Debug)]
        pub enum ObjectEvents {
            "#,
        );

        let gathered = read_file_to_vec(original.path());

        reinstate_docs(generated.path(), gathered);
        assert_eq!(
            std::fs::read_to_string(original.path()).unwrap(),
            std::fs::read_to_string(generated.path()).unwrap()
        );
    }

    #[test]
    fn reinstale_multiple_lines() {
        let original = temp_file::with_contents(
            br#"
        /// first line of item level docs
        /// second
        /// third
        pub enum ObjectEvents {
            "#,
        );

        let generated = temp_file::with_contents(
            br#"
        pub enum ObjectEvents {
            "#,
        );

        let gathered = read_file_to_vec(original.path());

        reinstate_docs(generated.path(), gathered);
        assert_eq!(
            std::fs::read_to_string(original.path()).unwrap(),
            std::fs::read_to_string(generated.path()).unwrap()
        );
    }

    #[test]
    fn reinstale_two_blocks_multiple_lines() {
        let original = temp_file::with_contents(
            br#"
        /// first line of item level docs
        /// second
        /// third
        pub enum ObjectEvents {

        /// first line of item level docs
        /// second
        /// third
        pub enum KeyboardEvents {
            "#,
        );

        let generated = temp_file::with_contents(
            br#"
        pub enum ObjectEvents {

        pub enum KeyboardEvents {
            "#,
        );

        let gathered = read_file_to_vec(original.path());

        reinstate_docs(generated.path(), gathered);
        assert_eq!(
            std::fs::read_to_string(original.path()).unwrap(),
            std::fs::read_to_string(generated.path()).unwrap()
        );
    }

    #[test]
    fn dont_reinstale_at_common_curly() {
        let original = temp_file::with_contents(
            br#"
        /// first line of item level docs
        /// second
        /// third
                 {
            "#,
        );

        let generated = temp_file::with_contents(
            br#"

                 {
            "#,
        );

        let gathered = read_file_to_vec(original.path());

        reinstate_docs(generated.path(), gathered);
        assert_eq!(
            r#"

                 {
            "#
            .to_owned(),
            std::fs::read_to_string(generated.path()).unwrap()
        );
    }

    #[test]
    fn reinstale_item_level() {
        let original = temp_file::with_contents(
            br#"        /// Important item level docs
        /// describing the item
        /// what it is, when to use, how to use
        
        pub struct PeculiarItem
            "#,
        );

        let generated = temp_file::with_contents(
            br#"        
        pub struct PeculiarItem
            "#,
        );

        let gathered = read_file_to_vec(original.path());
        let dt: DocType = DocType::CmtOrItem(CmtOrItem {
            dist: 1,
            doc: vec![
                "        /// Important item level docs".to_string(),
                "        /// describing the item".to_string(),
                "        /// what it is, when to use, how to use".to_string(),
            ],
        });
        let docvec: Vec<(Option<String>, DocType)> =
            vec![(Some("pub struct PeculiarItem".to_owned()), dt)];
        assert_eq!(gathered, docvec);

        reinstate_docs(generated.path(), gathered);
        assert_eq!(
            std::fs::read_to_string(original.path()).unwrap(),
            std::fs::read_to_string(generated.path()).unwrap()
        );
    }

    #[test]
    fn reinstale_module_level() {
        let original = temp_file::with_contents(
            b"\t//! Important module level docs\n\t//! describing the module\n\t//! how it works and what is in it\n\n\tuse std::collections::SomeSet;", 
        );
        let generated = temp_file::with_contents(b"\n\tuse std::collections::SomeSet;");

        let gathered = read_file_to_vec(original.path());
        reinstate_docs(generated.path(), gathered);
        assert_eq!(
            std::fs::read_to_string(original.path()).unwrap(),
            std::fs::read_to_string(generated.path()).unwrap()
        );
    }

    #[test]
    fn reinstale_nothing() {
        let original = temp_file::with_contents(
            br#"
            
            use std::collections::SomeSet;
            "#,
        );

        let generated = temp_file::with_contents(
            br#"
            
            use std::collections::SomeSet;
            "#,
        );

        let gathered = read_file_to_vec(original.path());
        reinstate_docs(generated.path(), gathered);
        assert_eq!(
            std::fs::read_to_string(original.path()).unwrap(),
            std::fs::read_to_string(generated.path()).unwrap()
        );
    }
}
