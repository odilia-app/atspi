use serde::{Serialize, Deserialize};
use zvairant::Type;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, Hash)]
pub enum RelationType {
	Null = 0,
	LabelFor,
	LabelledBy,
	ControllerFor,
	ControlledBy,
	MemberOf,
	TooltipFor,
	NodeChildOf,
	NodeParentOf,
	Extended,
	FlowsTo,
	FlowsFrom,
	SubwindowOf,
	Embeds,
	EmbeddedBy,
	PopupFor,
	ParentWindowOf,
	DescriptionFor,
	DescribedBy,
	Details,
	DetailsFor,
	ErrorMessage,
	ErrorFor,
}
