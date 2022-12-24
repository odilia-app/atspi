import xml.etree.ElementTree as ET
tree = ET.parse("../xml/Event.xml")
root = tree.getroot()

RUST_TYPE_FROM_DBUS_TYPE = {
  "s": "String",
  "i": "i32",
  "u": "u32",
  "v": "OwnedValue",
  "a{sv}": "HashMap<String, OwnedValue>",
  "o": "OwnedPath"
}
ARG_IDENT_FROM_NUMBER = [
  "kind",
  "detail1",
  "detail2",
  "any_data",
  "properties"
]
# will store lists of members with their various details. Will output a markdown table.
interfaces = []

def print_interface_table(signals):
  print(f"/// Event table for the contained types:\n///")
  print(f"/// Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties")
  print(f"/// ---|---|---|---|---|---|---")
  for signal in signals:
    interface = signal["interface"].replace("Event", "")
    member = signal["member"]
    kind = signal["kind"] or "\t"
    detail1 = signal["detail1"] or "\t"
    detail2 = signal["detail2"] or "\t"
    data = signal["any_data"] or "\t"
    properties = signal["properties"]
    print(f"/// {interface}|{member}|{kind}|{detail1}|{detail2}|{data}|{properties}")

for interface in root:
  enum_name = interface.attrib["name"].split(".")[-1] + "Event"
  enum = f"pub enum {enum_name}s {{\n"
# will contain all the attributes to use for markdown table
  signals = []
  structs = []
  for signal in interface:
    signal_identities = dict()
    variant_name = signal.attrib["name"]
    signal_identities["interface"] = enum_name
    signal_identities["member"] = variant_name
    enum += f"\t{variant_name}({variant_name}Event),\n"
    struct = f"#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\n"
    struct += f"struct {variant_name}Event {{\n"
    for (argi, arg) in enumerate(signal):
# ignore <annotation> tags for QSPI
      if arg.tag != "arg":
        continue
      signal_identities[ARG_IDENT_FROM_NUMBER[argi]] = None
# skip getting additional info if the name of the arg is not specified (trhis means it is not used)
      if "name" not in arg.attrib:
        continue
      field_name = arg.attrib["name"]
      field_type = arg.attrib["type"]
      rust_type = RUST_TYPE_FROM_DBUS_TYPE[field_type]
      signal_identities[ARG_IDENT_FROM_NUMBER[argi]] = field_name
      struct += f"\t{field_name}: {rust_type},\n"
    signals.append(signal_identities)
    struct += "}"
    structs.append(struct)
  enum += "}"
  interfaces.append(signals)
  for struct in structs:
    print(struct)
  print_interface_table(signals)
  print(enum)


