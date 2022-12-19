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

for interface in root:
  enum_name = interface.attrib["name"].split(".")[-1] + "Event"
  enum = f"pub enum {enum_name}\n"
  structs = []
  for signal in interface:
    variant_name = signal.attrib["name"]
    enum += f"\t{variant_name}({variant_name}Event),\n"
    struct = f"#[derive(Debug, PartialEq, Serialize, Deserialize)]\n"
    struct += f"struct {variant_name}Event {{\n"
    for arg in signal:
      if "name" in arg.attrib and arg.tag == "arg":
        field_name = arg.attrib["name"]
        field_type = arg.attrib["type"]
        rust_type = RUST_TYPE_FROM_DBUS_TYPE[field_type]
        struct += f"\t{field_name}: {rust_type},\n"
    struct += "}"
    structs.append(struct)
  enum += "}"
  for struct in structs:
    print(struct)
  print(enum)

