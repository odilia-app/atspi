generator = find_program('generate-introspection.py')

introspection_sources = [
  'Accessible.xml',
  'Action.xml',
  'Application.xml',
  'Cache.xml',
  'Collection.xml',
  'Component.xml',
  'DeviceEventController.xml',
  'DeviceEventListener.xml',
  'Document.xml',
  'EditableText.xml',
  'Event.xml',
  'Hyperlink.xml',
  'Hypertext.xml',
  'Image.xml',
  'Registry.xml',
  'Selection.xml',
  'Socket.xml',
  'Table.xml',
  'TableCell.xml',
  'Text.xml',
  'Value.xml',
]

introspection_generated = custom_target(
  'introspection_generated',
  input: introspection_sources,
  output: [ 'introspection.c', 'introspection.h' ],
  command: [ generator, '@INPUT@', '--c-output=@OUTPUT0@', '--h-output=@OUTPUT1@' ],
)
