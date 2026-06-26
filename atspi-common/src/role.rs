use serde::{
	de::{Deserializer, Visitor},
	ser::{SerializeSeq, Serializer},
	Deserialize, Serialize,
};
use std::{fmt, iter::FusedIterator};
use zvariant::Type;

use crate::AtspiError;

macro_rules! role_enum {
	($($(#[$doc:meta])* ($variant:ident, $human_name:literal)),+) => {
		/// An accessible object role.
		///
		/// To think of it in terms of HTML, any semantic element likely has a corollary in this enum.
		/// For example: `<button>`, `<input>`, `<form>` or `<h4>`.
		/// Non-semantic elements like `<span>`, `<div>` and `<b>` will not be represented here, and this information is not passed through via the atspi library.
		/// TODO: add examples for GTK/Qt libraries in addition to HTML examples.
		#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, Hash)]
		#[repr(u32)]
		pub enum Role {
			$(
				$(#[$doc])*
				$variant,
			)*
		}

		impl TryFrom<u32> for Role {
			type Error = AtspiError;

			#[allow(clippy::too_many_lines)]
			fn try_from(value: u32) -> Result<Self, Self::Error> {
				match value {
					$(
						_ if value == Self::$variant as u32 => Ok(Self::$variant),
					)*
					_ => Err(AtspiError::UnknownRole(value)),
				}
			}
		}

		impl Role {
			/// Get a readable, English name from the role.
			#[must_use]
			pub fn name(&self) -> &'static str {
				match self {
					$(
						Self::$variant => $human_name,
					)*
				}
			}

			const fn last_variant() -> Self {
				$(
					#[allow(unused_variables)]
					let variant = Self::$variant;
				)*
				variant
			}

			pub const LAST_VARIANT: Self = Self::last_variant();
		}

		impl std::fmt::Display for Role {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(self.name())
			}
		}
	}
}

role_enum! {
	/// A role indicating an error condition, such as uninitialized Role data, or an error deserializing.
	(Invalid, "invalid"),
	/// Object is a label indicating the keyboard accelerators for the parent.
	(AcceleratorLabel, "accelerator label"),
	/// Object is used to alert the user about something.
	(Alert, "alert"),
	/// Object contains a dynamic or moving image of some kind.
	(Animation, "animation"),
	/// Object is a 2d directional indicator.
	(Arrow, "arrow"),
	/// Object contains one or more dates, usually arranged into a 2d list.
	(Calendar, "calendar"),
	/// Object that can be drawn into and is used to trap events.
	(Canvas, "canvas"),
	/// A choice that can be checked or unchecked and provides a separate indicator for the current state.
	(CheckBox, "check box"),
	/// A menu item that behaves like a check box. See [`Self::CheckBox`].
	(CheckMenuItem, "check menu item"),
	/// A specialized dialog that lets the user choose a color.
	(ColorChooser, "color chooser"),
	/// The header for a column of data.
	(ColumnHeader, "column header"),
	/// A list of choices the user can select from.
	(ComboBox, "combo box"),
	/// An object which allows entry of a date.
	(DateEditor, "date editor"),
	/// An inconifed internal frame within a [`Role::DesktopFrame`].
	(DesktopIcon, "desktop icon"),
	/// A pane that supports internal frames and iconified versions of those internal frames.
	(DesktopFrame, "desktop frame"),
	/// An object that allows a value to be changed via rotating a visual element, or which displays a value via such a rotating element.
	(Dial, "dial"),
	/// A top level window with title bar and a border.
	(Dialog, "dialog"),
	/// A pane that allows the user to navigate through and select the contents of a directory.
	(DirectoryPane, "directory pane"),
	/// An object used for drawing custom user interface elements.
	(DrawingArea, "drawing area"),
	/// A specialized dialog that displays the files in the directory and lets the user select a file, browse a different directory, or specify a filename.
	(FileChooser, "file chooser"),
	/// A object that fills up space in a user interface.
	(Filler, "filler"),
	/// Don't use, reserved for future use.
	(FocusTraversable, "focus traversable"),
	/// Allows selection of a display font.
	(FontChooser, "font chooser"),
	/// A top level window with a title bar, border, menubar, etc.
	(Frame, "frame"),
	/// A pane that is guaranteed to be painted on top of all panes beneath it.
	(GlassPane, "glass pane"),
	/// A document container for HTML, whose children represent the document content.
	(HTMLContainer, "html container"),
	/// A small fixed size picture, typically used to decorate components.
	(Icon, "icon"),
	/// An image, typically static.
	(Image, "image"),
	/// A frame-like object that is clipped by a desktop pane.
	(InternalFrame, "internal frame"),
	/// An object used to present an icon or short string in an interface.
	(Label, "label"),
	/// A specialized pane that allows its children to be drawn in layers, providing a form of stacking order.
	(LayeredPane, "layered pane"),
	/// An object that presents a list of objects to the user and * allows the user to select one or more of them.
	(List, "list"),
	/// An object that represents an element of a list.
	(ListItem, "list item"),
	/// An object usually found inside a menu bar that contains a list of actions the user can choose from.
	(Menu, "menu"),
	/// An object usually drawn at the top of the primary dialog box of an application that contains a list of menus the user can choose from.
	(MenuBar, "menu bar"),
	/// An object usually contained in a menu that presents an action the user can choose.
	(MenuItem, "menu item"),
	/// A specialized pane whose primary use is inside a dialog.
	(OptionPane, "option pane"),
	/// An object that is a child of a page tab list.
	(PageTab, "page tab"),
	/// An object that presents a series of panels (or page tabs), one at a time,through some mechanism provided by the object.
	(PageTabList, "page tab list"),
	/// A generic container that is often used to group objects.
	(Panel, "panel"),
	/// A text object uses for passwords, or other places where the text content is not shown visibly to the user.
	(PasswordText, "password text"),
	/// A temporary window that is usually used to offer the user a list of choices, and then hides when the user selects one of those choices.
	(PopupMenu, "popup menu"),
	/// An object used to indicate how much of a task has been completed.
	(ProgressBar, "progress bar"),
	/// An object the user can manipulate to tell the application to do something.
	(Button, "button"),
	/// A specialized check box that will cause other radio buttons in the same group to become unchecked when this one is checked.
	(RadioButton, "radio button"),
	/// Object is both a menu item and a "radio button". See [`Self::RadioButton`].
	(RadioMenuItem, "radio menu item"),
	/// A specialized pane that has a glass pane and a layered pane as its children.
	(RootPane, "root pane"),
	/// The header for a row of data.
	(RowHeader, "row header"),
	/// An object usually used to allow a user to incrementally view a large amount of data by moving the bounds of a viewport along a one-dimensional axis.
	(ScrollBar, "scroll bar"),
	/// A scroll pane: the pane in which the scrollable content is contained within.
	/// An object that allows a user to incrementally view a large amount of information.
	/// [`Self::ScrollPane`] objects are usually accompanied by [`Self::ScrollBar`] controllers,
	/// on which the [`crate::RelationType::ControllerFor`] and [`crate::RelationType::ControlledBy`] reciprocal relations are set.
	(ScrollPane, "scroll pane"),
	/// An object usually contained in a menu to provide a visible and logical separation of the contents in a menu.
	(Separator, "separator"),
	/// An object that allows the user to select from a bounded range.
	/// Unlike [`Self::ScrollBar`], [`Self::Slider`] objects need not control 'viewport'-like objects.
	(Slider, "slider"),
	/// An object which allows one of a set of choices to be selected, and which displays the current choice.
	(SpinButton, "spin button"),
	/// A specialized panel that presents two other panels at the same time.
	(SplitPane, "split pane"),
	/// Object displays non-quantitative status information (c.f. [`Self::ProgressBar`])
	(StatusBar, "status bar"),
	/// An object used to represent information in terms of rows and columns.
	(Table, "table"),
	/// A 'cell' or discrete child within a Table.
	/// Note: Table cells need not have [`Self::TableCell`], other [`crate::Role`] values are valid as well.
	(TableCell, "table cell"),
	/// An object which labels a particular column in an [`Self::Table`].
	(TableColumnHeader, "table column header"),
	/// An object which labels a particular row in a [`Self::Table`].
	/// `TableProxy` rows and columns may also be labelled via the
	/// [`crate::RelationType::LabelFor`]/[`crate::RelationType::LabelledBy`] relationships.
	/// See: `AccessibleProxy::get_relation_type`.
	(TableRowHeader, "table row header"),
	/// Object allows menu to be removed from menubar and shown in its own window.
	(TearoffMenuItem, "tearoff menu item"),
	/// An object that emulates a terminal.
	(Terminal, "terminal"),
	/// An interactive widget that supports multiple lines of text and optionally accepts user input,
	/// but whose purpose is not to solicit user input.
	/// Thus [`Self::Text`] is appropriate for the text view in a plain text editor but inappropriate for an input field in a dialog box or web form.
	/// For widgets whose purpose is to solicit input from the user, see [`Self::Entry`] and [`Self::PasswordText`].
	/// For generic objects which display a brief amount of textual information, see [`Self::Static`].
	(Text, "text"),
	/// A specialized push button that can be checked or unchecked, but does not provide a separate indicator for the current state.
	(ToggleButton, "toggle button"),
	/// A bar or palette usually composed of push buttons or toggle buttons.
	(ToolBar, "tool bar"),
	/// An object that provides information about another object.
	(ToolTip, "tool tip"),
	/// An object used to repsent hierarchical information to the user.
	(Tree, "tree"),
	/// An object that presents both tabular and hierarchical info to the user.
	(TreeTable, "tree table"),
	/// When the role cannot be accurately reported, this role will be set.
	(Unknown, "unknown"),
	/// An object usually used in a scroll pane, or to otherwise clip a larger object or content renderer to a specific onscreen viewport.
	(Viewport, "viewport"),
	/// A top level window with no title or border.
	(Window, "window"),
	/// means that the role for this item is known, but not included in the core enumeration.
	(Extended, "extended"),
	/// An object that serves as a document header.
	(Header, "header"),
	/// An object that serves as a document footer.
	(Footer, "footer"),
	/// An object which is contains a single paragraph of text content. See also [`Self::Text`].
	(Paragraph, "paragraph"),
	/// An object which describes margins and tab stops, etc. for text objects which it controls (should have [`crate::RelationType::ControllerFor`] relation to such).
	(Ruler, "ruler"),
	/// An object corresponding to the toplevel accessible of an application, which may contain [`Self::Frame`] objects or other accessible objects.
	/// Children of objects with the [`Self::DesktopFrame`] role are generally [`Self::Application`] objects.
	(Application, "application"),
	/// The object is a dialog or list containing items for insertion into an entry widget, for instance a list of words for completion of a text entry.
	(Autocomplete, "autocomplete"),
	/// The object is an editable text object in a toolbar.
	(Editbar, "editbar"),
	/// The object is an embedded component container.
	/// This role is a "grouping" hint that the contained objects share a context which is different from the container in which this accessible is embedded.
	/// In particular, it is used for some kinds of document embedding, and for embedding of out-of-process component, "panel applets", etc.
	(Embedded, "embedded"),
	/// The object is a component whose textual content may be entered or modified by the user, provided [`crate::State::Editable`] is present.
	/// A readonly [`Self::Entry`] object (i.e. where [`crate::State::Editable`] is not present) implies a read-only 'text field' in a form, as opposed to a title, label, or caption.
	(Entry, "entry"),
	/// The object is a graphical depiction of quantitative data.
	/// It may contain multiple subelements whose attributes and/or description may be queried to obtain both the  quantitative data and information about how the data is being presented.
	/// The [`crate::RelationType::LabelledBy`] relation is particularly important in interpreting objects of this type, as is the accessible description property.
	/// See [`Self::Caption`].
	(CHART, "chart"),
	/// The object contains descriptive information, usually textual, about another user interface element such as a table, chart, or image.
	(Caption, "caption"),
	/// The object is a visual frame or container which
	/// contains a view of document content. [`Self::DocumentFrame`]s may occur within
	/// another `DocumentProxy` instance, in which case the second  document may be
	/// said to be embedded in the containing instance.
	/// HTML frames are often [`Self::DocumentFrame`]:  Either this object, or a singleton descendant,
	/// should implement the [`crate::Interface::Document`] interface.
	(DocumentFrame, "document frame"),
	/// Heading: this is a heading with a level (usually 1-6). This is represented by `<h1>` through `<h6>` in HTML.
	/// The object serves as a heading for content which follows it in a document.
	/// The 'heading level' of the heading, if available, may be obtained by querying the object's attributes.
	(Heading, "heading"),
	/// The object is a containing instance which encapsulates a page of information.
	/// [`Self::Page`] is used in documents and content which support a paginated navigation model.
	(Page, "page"),
	/// The object is a containing instance of document content which constitutes a particular 'logical' section of the document.
	/// The type of content within a section, and the nature of the section division itself, may be obtained by querying the object's attributes.
	/// Sections may be nested.
	(Section, "section"),
	/// The object is redundant with another object in the hierarchy, and is exposed for purely technical reasons.
	/// Objects of this role should be ignored by clients, if they are encountered at all.
	(RedundantObject, "redundant object"),
	/// The object is a containing instance of document content which has within it components with which the user can interact in order to input information;
	/// i.e. the object is a container for pushbuttons, comboboxes, text input fields, and other 'GUI' components.
	/// [`Self::Form`] should not, in general, be used for toplevel GUI containers or dialogs, but should be reserved for 'GUI' containers which occur within document content, for instance within Web documents, presentations, or text documents.
	/// Unlike other GUI containers and dialogs which occur inside application instances, [`Self::Form`] containers' components are associated with the current document, rather than the current foreground application or viewer instance.
	(Form, "form"),
	/// The object is a hypertext anchor, i.e. a "link" in a hypertext document.
	/// Such objects are distinct from 'inline' content which may also use the [`crate::Interface::Hypertext`]/[`crate::Interface::Hyperlink`] interfaces to indicate the range/location within a text object where an inline or embedded object lies.
	(Link, "link"),
	/// The object is a window or similar viewport which is used to allow composition or input of a 'complex character', in other words it is an "input method window".
	(InputMethodWindow, "input method window"),
	/// A row in a table.
	(TableRow, "table row"),
	/// An object that represents an element of a tree.
	(TreeItem, "tree item"),
	/// A document frame which contains a spreadsheet.
	(DocumentSpreadsheet, "document spreadsheet"),
	/// A document frame which contains a presentation or slide content.
	(DocumentPresentation, "document presentation"),
	/// A document frame which contains textual content, such as found in a word processing application.
	(DocumentText, "document text"),
	/// A document frame which contains HTML or other markup suitable for display in a web browser.
	(DocumentWeb, "document web"),
	/// A document frame which contains email content to be displayed or composed either in plain text or HTML.
	(DocumentEmail, "document email"),
	/// An object found within a document and designed to present a comment, note, or other annotation.
	/// In some cases, this object might not be visible until activated.
	(Comment, "comment"),
	/// A non-collapsible list of choices the user can select from.
	(ListBox, "list box"),
	/// A group of related widgets. This group typically has a label.
	(Grouping, "grouping"),
	/// An image map object. Usually a graphic with multiple hotspots, where each hotspot can be activated resulting in the loading of another document or section of a document.
	(ImageMap, "image map"),
	/// A transitory object designed to present a message to the user, typically at the desktop level rather than inside a particular application.
	(Notification, "notification"),
	/// An object designed to present a message to the user within an existing window.
	(InfoBar, "info bar"),
	/// A bar that serves as a level indicator to, for instance, show the strength of a password or the state of a battery.
	(LevelBar, "level bar"),
	/// A bar that serves as the title of a window or a dialog.
	(TitleBar, "title bar"),
	/// An object which contains a text section that is quoted from another source.
	(BlockQuote, "block quote"),
	/// An object which represents an audio element.
	(Audio, "audio"),
	/// An object which represents a video element.
	(Video, "video"),
	/// A definition of a term or concept.
	(Definition, "definition"),
	/// A section of a page that consists of a composition that forms an independent part of a document, page, or site.
	/// Examples: A blog entry, a news story, a forum post.
	(Article, "article"),
	/// A region of a web page intended as a navigational landmark. This is designed to allow Assistive Technologies to provide quick navigation among key regions within a document.
	(Landmark, "landmark"),
	/// A text widget or container holding log content, such as chat history and error logs. In this role there is a relationship between the arrival of new items in the log and the reading order.
	/// The log contains a meaningful sequence and new information is added only to the end of the log, not at arbitrary points.
	(Log, "log"),
	/// A container where non-essential information changes frequently.
	/// Common usages of marquee include stock tickers and ad banners.
	/// The primary difference between a marquee and a log is that logs usually have a meaningful order or sequence of important content changes.
	(Marquee, "marquee"),
	/// A text widget or container that holds a mathematical expression.
	(Math, "math"),
	/// A rating system, generally out of five stars, but it does not need to be that way. There is no tag nor role for this in HTML, however.
	/// A widget whose purpose is to display a rating, such as the number of stars associated with a song in a media player.
	/// Objects of this role should also implement [`crate::Interface::Value`].
	(Rating, "rating"),
	/// An object containing a numerical counter which indicates an amount of elapsed time from a start point, or the time remaining until an end point.
	(Timer, "timer"),
	/// A generic non-container object whose purpose is to display a brief amount of information to the user and whose role is known by the implementor but lacks semantic value for the user.
	/// Examples in which [`Self::Static`] is appropriate include the message displayed in a message box and an image used as an alternative means to display text.
	/// [`Self::Static`] should not be applied to widgets which are traditionally interactive, objects which display a significant amount of content, or any object which has an accessible relation pointing to another object.
	/// The displayed information, as a general rule, should be exposed through the accessible name of the object.
	/// For labels which describe another widget, see [`Self::Label`].
	/// For text views, see [`Self::Text`].
	/// For generic containers, see [`Self::Panel`]. For objects whose role is not known by the implementor, see [`Self::Unknown`].
	(Static, "static"),
	/// An object that represents a mathematical fraction.
	(MathFraction, "math fraction"),
	/// An object that represents a mathematical expression displayed with a radical.
	(MathRoot, "math root"),
	/// An object that contains text that is displayed as a subscript.
	(Subscript, "subscript"),
	/// An object that contains text that is displayed as a superscript.
	(Superscript, "superscript"),
	/// An object that represents a list of term-value groups.
	/// A term-value group represents an individual description and consist of one or more names ([`Self::DescriptionTerm`]) followed by one or more values ([`Self::DescriptionValue`]).
	/// For each list, there should not be more than one group with the same term name.
	(DescriptionList, "description list"),
	/// An object that represents a term or phrase with a corresponding definition.
	(DescriptionTerm, "description term"),
	/// An object that represents the description, definition, or value of a term.
	(DescriptionValue, "description value"),
	/// An object that contains the text of a footnote.
	(Footnote, "footnote"),
	/// Content previously deleted or proposed to be deleted, e.g. in revision history or a content view providing suggestions from reviewers.
	(ContentDeletion, "content deletion"),
	/// Content previously inserted or proposed to be inserted, e.g. in revision history or a content view providing suggestions from reviewers.
	(ContentInsertion, "content insertion"),
	/// A run of content that is marked or highlighted, such as for reference purposes, or to call it out as having a special purpose.
	/// If the marked content has an associated section in the document elaborating on the reason for the mark, then [`crate::RelationType::Details`] should be used on the mark to point to that associated section.
	/// In addition, the reciprocal relation [`crate::RelationType::DetailsFor`] should be used on the associated content section to point back to the mark.
	(Mark, "mark"),
	/// A container for content that is called out as a proposed change from the current version of the document, such as by a reviewer of the content.
	/// An object with this role should include children with [`Self::ContentDeletion`] and/or [`Self::ContentInsertion`], in any order, to indicate what the actual change is.
	(Suggestion, "suggestion"),
	/// A specialized push button to open a menu.
	(PushButtonMenu, "push button menu"),
	/// An on/off switch.
	(Switch, "switch")
}

/// The bitflag representation of all roles an object may have.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct RoleSet([i32; 5]);

fn element_idx_and_bit_for(role: Role) -> (usize, usize) {
	let role_val = role as usize;
	let index = role_val / 32;
	debug_assert!(index < RoleSet::empty().0.len());
	let bit = role_val % 32;
	(index, bit)
}

impl RoleSet {
	/// Create a new `RoleSet` from a collection of roles.
	pub fn new<I>(roles: I) -> Self
	where
		I: IntoIterator<Item = Role>,
	{
		Self::from_iter(roles)
	}

	/// Create an empty `RoleSet`.
	#[must_use]
	pub const fn empty() -> Self {
		Self([0; 5])
	}

	/// Checks if all roles are unset.
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.iter().all(|&bits| bits == 0)
	}

	/// Checks if a specific [`Role`] is in the set.
	#[must_use]
	pub fn contains(&self, role: Role) -> bool {
		let (index, bit) = element_idx_and_bit_for(role);
		if let Some(&bits) = self.0.get(index) {
			(bits >> bit) & 0b1 == 0b1
		} else {
			false
		}
	}

	/// Inserts a [`Role`] into the set.
	pub fn insert(&mut self, role: Role) {
		let (index, bit) = element_idx_and_bit_for(role);
		if let Some(bits) = self.0.get_mut(index) {
			*bits |= 1 << bit;
		}
	}

	/// Removes a [`Role`] from the set.
	pub fn remove(&mut self, role: Role) {
		let (index, bit) = element_idx_and_bit_for(role);
		if let Some(bits) = self.0.get_mut(index) {
			*bits &= !(1 << bit);
		}
	}

	/// Toggles a [`Role`] in the set.
	pub fn toggle(&mut self, role: Role) {
		let (index, bit) = element_idx_and_bit_for(role);
		if let Some(bits) = self.0.get_mut(index) {
			*bits ^= 1 << bit;
		}
	}

	/// Returns the raw bits representing the set.
	#[must_use]
	pub fn bits(&self) -> [i32; 5] {
		self.0
	}

	/// Returns an iterator yielding each set [`Role`].
	#[must_use]
	pub fn iter(&self) -> RoleSetIterator {
		RoleSetIterator { set: *self, index: 0, remaining: self.len() }
	}

	/// Returns the number of roles in this set.
	#[must_use]
	pub fn len(&self) -> usize {
		self.0.iter().map(|&bits| (bits).count_ones()).sum::<u32>() as usize
	}
}

impl IntoIterator for RoleSet {
	type IntoIter = RoleSetIterator;
	type Item = Role;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl IntoIterator for &RoleSet {
	type IntoIter = RoleSetIterator;
	type Item = Role;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl FromIterator<Role> for RoleSet {
	fn from_iter<I: IntoIterator<Item = Role>>(iter: I) -> Self {
		let mut set = Self::empty();
		for role in iter {
			set.insert(role);
		}
		set
	}
}

impl<'a> FromIterator<&'a Role> for RoleSet {
	fn from_iter<I: IntoIterator<Item = &'a Role>>(iter: I) -> Self {
		let mut set = Self::empty();
		for &role in iter {
			set.insert(role);
		}
		set
	}
}

impl From<Role> for RoleSet {
	fn from(value: Role) -> Self {
		let mut set = Self::empty();
		set.insert(value);
		set
	}
}

impl std::ops::BitXor for RoleSet {
	type Output = RoleSet;

	fn bitxor(self, other: Self) -> Self::Output {
		RoleSet(std::array::from_fn(|i| self.0[i] ^ other.0[i]))
	}
}

impl std::ops::BitXorAssign for RoleSet {
	fn bitxor_assign(&mut self, other: Self) {
		*self = *self ^ other;
	}
}

impl std::ops::BitOr for RoleSet {
	type Output = RoleSet;

	fn bitor(self, other: Self) -> Self::Output {
		RoleSet(std::array::from_fn(|i| self.0[i] | other.0[i]))
	}
}

impl std::ops::BitOrAssign for RoleSet {
	fn bitor_assign(&mut self, other: Self) {
		*self = *self | other;
	}
}

impl std::ops::BitAnd for RoleSet {
	type Output = RoleSet;

	fn bitand(self, other: Self) -> Self::Output {
		RoleSet(std::array::from_fn(|i| self.0[i] & other.0[i]))
	}
}

impl std::ops::BitAndAssign for RoleSet {
	fn bitand_assign(&mut self, other: Self) {
		*self = *self & other;
	}
}

#[derive(Clone, Debug)]
pub struct RoleSetIterator {
	set: RoleSet,
	index: u32,
	remaining: usize,
}

impl Iterator for RoleSetIterator {
	type Item = Role;

	fn next(&mut self) -> Option<Self::Item> {
		if self.remaining == 0 {
			return None;
		}

		while let Ok(role) = Role::try_from(self.index) {
			self.index += 1;
			if self.set.contains(role) {
				self.remaining -= 1;
				return Some(role);
			}
		}
		None
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.remaining, Some(self.remaining))
	}
}

impl FusedIterator for RoleSetIterator {}

impl ExactSizeIterator for RoleSetIterator {
	fn len(&self) -> usize {
		self.remaining
	}
}

impl<'de> Deserialize<'de> for RoleSet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct RoleSetVisitor;

		impl<'de> Visitor<'de> for RoleSetVisitor {
			type Value = RoleSet;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter
					.write_str("a sequence comprised of five i32 that represents a valid RoleSet")
			}

			fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
			where
				D: Deserializer<'de>,
			{
				let arr =
					<[i32; RoleSet::empty().0.len()] as Deserialize>::deserialize(deserializer)?;
				Ok(RoleSet(arr))
			}
		}

		deserializer.deserialize_newtype_struct("RoleSet", RoleSetVisitor)
	}
}

impl Serialize for RoleSet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut seq = serializer.serialize_seq(Some(5))?;
		for &bits in &self.0 {
			seq.serialize_element(&bits)?;
		}
		seq.end()
	}
}

impl Type for RoleSet {
	const SIGNATURE: &'static zvariant::Signature = <Vec<i32> as Type>::SIGNATURE;
}

#[cfg(test)]
pub mod tests {
	use super::{Role, RoleSet};
	use zvariant::serialized::Context;
	use zvariant::{to_bytes, LE};

	#[test]
	fn test_serialization_matches_from_impl() {
		let ctxt = Context::new_dbus(LE, 0);

		for role_num in 1..=(Role::LAST_VARIANT as u32) {
			let from_role = Role::try_from(role_num)
				.unwrap_or_else(|_| panic!("Unable to convert {role_num} into Role"));
			let encoded = to_bytes(ctxt, &from_role)
				.unwrap_or_else(|_| panic!("Unable to encode {from_role}"));
			println!("ENCODED: {encoded:?}");

			let (zbus_role, _) = encoded
				.deserialize()
				.unwrap_or_else(|_| panic!("Unable to decode {encoded:?}"));

			assert_eq!(from_role, zbus_role, "The serde `Data::deserialize` and `From<u32>` impls produced different results. The number used was {role_num}, it produced a Role of {from_role}, but the from_slice(...) implementation produced {zbus_role}");
			assert_eq!(
				from_role as u32, role_num,
				"The role number {role_num} does not match the representation of the role {}",
				from_role as u32
			);
		}
	}

	#[test]
	fn test_role_set_ops_empty() {
		let set = RoleSet::empty();
		assert!(set.is_empty());
	}

	#[test]
	fn test_role_set_ops_remove() {
		let mut set = RoleSet::new([Role::Arrow]);
		assert!(!set.is_empty());
		set.remove(Role::Arrow);
		assert!(set.is_empty());
	}

	#[test]
	fn test_role_set_ops_insert() {
		let mut set = RoleSet::empty();

		set.insert(Role::Alert);
		set.insert(Role::Animation);
		set.insert(Role::Header);
		assert_eq!(set, RoleSet::new([Role::Alert, Role::Animation, Role::Header]));
	}

	#[test]
	fn test_role_set_ops_contains() {
		let mut set = RoleSet::empty();
		assert!(set.is_empty());
		assert!(!set.contains(Role::Alert));
		assert!(!set.contains(Role::Button));

		set.insert(Role::Alert);
		assert!(!set.is_empty());
		assert!(set.contains(Role::Alert));
		assert!(!set.contains(Role::Button));

		set.insert(Role::Button);
		assert!(set.contains(Role::Button));
	}

	#[test]
	fn test_role_set_ops_toggle() {
		let mut set = RoleSet::empty();
		set.toggle(Role::Alert);
		assert!(set.contains(Role::Alert));
		set.toggle(Role::Alert);
		assert!(!set.contains(Role::Alert));
	}

	#[test]
	fn test_role_set_ops_bits() {
		let mut set = RoleSet::new([Role::Alert, Role::Button]);
		assert_eq!(set.bits(), [4, 2048, 0, 0, 0]);

		set.insert(Role::Window);
		assert_eq!(set.bits(), [4, 2048, 32, 0, 0]);
	}

	#[test]
	fn test_role_set_ops_iter() {
		let mut set = RoleSet::empty();
		set.insert(Role::Alert);
		set.insert(Role::Button);
		assert_eq!(set.iter().collect::<Vec<_>>(), vec![Role::Alert, Role::Button]);
	}

	#[test]
	fn test_role_set_bitops_xor() {
		let set1 = RoleSet::new([Role::Alert, Role::Button]);
		let set2 = RoleSet::new([Role::Button, Role::Window]);
		let result = set1 ^ set2;

		assert!(result.contains(Role::Alert));
		assert!(result.contains(Role::Window));
		assert!(!result.contains(Role::Button));
	}

	#[test]
	fn test_role_set_bitops_and() {
		let set1 = RoleSet::new([Role::Alert, Role::Button]);
		let set2 = RoleSet::new([Role::Button, Role::Window]);
		let result = set1 & set2;

		assert!(!result.contains(Role::Alert));
		assert!(!result.contains(Role::Window));
		assert!(result.contains(Role::Button));
	}

	#[test]
	fn test_role_set_bitops_or() {
		let set1 = RoleSet::new([Role::Alert, Role::Button]);
		let set2 = RoleSet::new([Role::Button, Role::Window]);
		let result = set1 | set2;
		assert!(result.contains(Role::Alert));
		assert!(result.contains(Role::Window));
		assert!(result.contains(Role::Button));
	}

	#[test]
	fn test_role_set_bitops_xor_assign() {
		let set1 = RoleSet::new([Role::Alert, Role::Button]);
		let set2 = RoleSet::new([Role::Button, Role::Window]);
		let mut set1_clone = set1;
		set1_clone ^= set2;
		assert_ne!(set1_clone, set1);
		assert!(set1_clone.contains(Role::Alert));
		assert!(set1_clone.contains(Role::Window));
		assert!(!set1_clone.contains(Role::Button));
	}

	#[test]
	fn test_role_set_bitops_and_assign() {
		let set1 = RoleSet::new([Role::Alert, Role::Button]);
		let set2 = RoleSet::new([Role::Button, Role::Window]);
		let mut set1_clone = set1;
		set1_clone &= set2;
		assert_ne!(set1_clone, set1);
		assert!(!set1_clone.contains(Role::Alert));
		assert!(!set1_clone.contains(Role::Window));
		assert!(set1_clone.contains(Role::Button));
	}

	#[test]
	fn test_role_set_bitops_or_assign() {
		let set1 = RoleSet::new([Role::Alert, Role::Button]);
		let set2 = RoleSet::new([Role::Button, Role::Window]);
		let mut set2_clone = set2;
		set2_clone |= set1;
		assert_ne!(set2_clone, set2);
		assert!(set2_clone.contains(Role::Alert));
		assert!(set2_clone.contains(Role::Window));
		assert!(set2_clone.contains(Role::Button));
	}

	#[test]
	fn test_role_set_serialization_deserialization() {
		use super::RoleSet;
		use zvariant::serialized::Data;

		let mut set = RoleSet::empty();
		set.insert(Role::Alert); // Role::Alert is index 2, bit 2 inside the first i32 element.
		set.insert(Role::Button); // Role::Button is index 43, bit 11 inside the second i32 element (43 - 32 = 11).

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &set).unwrap();

		let expected_bytes = &[
			20, 0, 0, 0, // D-Bus array length header = 20 bytes
			4, 0, 0, 0, // Index 0: 1 << 2 = 4
			0, 8, 0, 0, // Index 1: 1 << 11 = 2048 (0x0800 in Little-Endian)
			0, 0, 0, 0, // Index 2: 0
			0, 0, 0, 0, // Index 3: 0
			0, 0, 0, 0, // Index 4: 0
		];
		assert_eq!(encoded.bytes(), expected_bytes);

		let data = Data::new::<&[u8]>(expected_bytes, ctxt);
		let (decoded, _) = data.deserialize::<RoleSet>().unwrap();
		assert_eq!(decoded, set);
		assert!(decoded.contains(Role::Alert));
		assert!(decoded.contains(Role::Button));
	}

	#[test]
	fn test_role_set_len() {
		let mut set = RoleSet::empty();
		assert_eq!(set.len(), 0);

		set.insert(Role::Alert);
		assert_eq!(set.len(), 1);

		// Duplicate insert must not change the length
		set.insert(Role::Alert);
		assert_eq!(set.len(), 1);

		set.insert(Role::Button);
		set.insert(Role::Window);
		assert_eq!(set.len(), 3);

		set.remove(Role::Button);
		assert_eq!(set.len(), 2);
	}

	#[test]
	fn test_role_set_iterator_exact_size() {
		let set = RoleSet::new([Role::Alert, Role::Button, Role::Window]);
		let mut iter = set.iter();

		assert_eq!(iter.len(), 3);
		assert_eq!(iter.size_hint(), (3, Some(3)));

		assert_eq!(iter.next(), Some(Role::Alert));
		assert_eq!(iter.len(), 2);
		assert_eq!(iter.size_hint(), (2, Some(2)));

		assert_eq!(iter.next(), Some(Role::Button));
		assert_eq!(iter.len(), 1);
		assert_eq!(iter.size_hint(), (1, Some(1)));

		assert_eq!(iter.next(), Some(Role::Window));
		assert_eq!(iter.len(), 0);
		assert_eq!(iter.size_hint(), (0, Some(0)));

		assert_eq!(iter.next(), None);
		assert_eq!(iter.len(), 0);
		assert_eq!(iter.size_hint(), (0, Some(0)));
	}

	#[test]
	fn test_role_set_iterator_fused() {
		let set = RoleSet::new([Role::Alert]);
		let mut iter = set.iter();

		assert_eq!(iter.next(), Some(Role::Alert));

		assert_eq!(iter.next(), None);

		assert_eq!(iter.next(), None, "Fused, must return None");
		assert_eq!(iter.next(), None, "Fused, must return None");
	}
}
