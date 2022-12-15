import xml.etree.ElementTree as ET
tree = ET.parse("../xml/Event.xml")
root = tree.getroot()

RUST_TYPE_FROM_DBUS_TYPE = {
  "s": "String",
  "i": "i32",
  "u": "u32",
  "v": "OwnedValue",
  "a{sv}": "HashMap<String, OwnedValue>"
}

for interface in root:
  enum_name = interface.attrib["name"].split(".")[-1] + "Event"
  print(f"pub enum {enum_name}")
  structs = []
  for signal in interface:
    variant_name = signal.attrib["name"]
    print(f"\t{variant_name}({variant_name}Event),")
    struct = f"struct {variant_name}Event {{\n"
    for arg in signal:
      if "name" in arg.attrib and arg.tag == "arg":
        field_name = arg.attrib["name"]
        field_type = arg.attrib["type"]
        rust_type = RUST_TYPE_FROM_DBUS_TYPE[field_type]
        struct += f"\t{field_name}: {rust_type},\n"
    struct += "}"
    structs.append(struct)
  print("}")
  for struct in structs:
    print(struct)


