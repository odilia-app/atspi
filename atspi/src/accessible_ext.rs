use crate::{
	accessible::{
		Accessible, AccessibleBlocking, AccessibleProxy, AccessibleProxyBlocking, ObjectPair,
		RelationType, Role,
	},
	collection::MatchType,
	convertable::{Convertable, ConvertableBlocking},
	hyperlink::Hyperlink,
	text::{Text, TextBlocking},
	InterfaceSet,
};
use async_trait::async_trait;
use std::collections::HashMap;

pub type MatcherArgs =
	(Vec<Role>, MatchType, HashMap<String, String>, MatchType, InterfaceSet, MatchType);

#[async_trait]
pub trait AccessibleExt {
	type Error: std::error::Error;
	async fn get_application_ext<'a>(&self) -> Result<Self, Self::Error>
	where
		Self: Sized;
	async fn get_parent_ext<'a>(&self) -> Result<Self, Self::Error>
	where
		Self: Sized;
	async fn get_children_ext<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_siblings<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_children_indexes<'a>(&self) -> Result<Vec<i32>, Self::Error>;
	async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_children_caret<'a>(&self, after: bool) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	/* TODO: not sure where these should go since it requires both Text as a self interface and
	 * Hyperlink as children interfaces. */
	async fn get_next<'a>(
		&self,
		matcher_args: &MatcherArgs,
		backward: bool,
		already_visited: &'a mut Vec<ObjectPair>,
	) -> Result<Option<Self>, Self::Error>
	where
		Self: Sized;
	/// Get all edges for a given accessible object.
	/// This means: all children, siblings, and parent, in that order.
	/// If a direction is specified, then it will only get the appicable matching siblings/children.
	/// This also checks if the element supports the text interface, and then checks if the caret position is contained within the string, if it is, then children are also handled by direction.
	async fn edges<'a>(&self, backward: Option<bool>) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;
	async fn get_relation_set_ext<'a>(
		&self,
	) -> Result<HashMap<RelationType, Vec<Self>>, Self::Error>
	where
		Self: Sized;
	async fn match_(
		&self,
		matcher_args: &MatcherArgs,
	) -> Result<bool, <Self as AccessibleExt>::Error>;
}
// TODO: implement AccessibleExt
pub trait AccessibleBlockingExt {}

#[allow(clippy::module_name_repetitions)]
pub trait AccessibleExtError: Accessible + Convertable {
	type Error: std::error::Error
		+ From<<Self as Accessible>::Error>
		+ From<<Self as Convertable>::Error>
		// TODO: add all convertable error types
		+ From<<<Self as Convertable>::Text as Text>::Error>
		+ From<std::num::TryFromIntError>
		+ Send;
}

#[allow(clippy::module_name_repetitions)]
pub trait AccessibleBlockingExtError: AccessibleBlocking + ConvertableBlocking {
	type Error: std::error::Error
		+ From<<Self as AccessibleBlocking>::Error>
		+ From<<Self as ConvertableBlocking>::Error>
		// TODO: add all convertable error types
		+ From<<<Self as ConvertableBlocking>::Text as TextBlocking>::Error>
		+ From<std::num::TryFromIntError>;
}

#[async_trait]
impl<T: Accessible + Convertable + AccessibleExtError + Send + Sync + Clone> AccessibleExt for T
where
	ObjectPair: for<'c> TryFrom<&'c T>,
{
	type Error = <T as AccessibleExtError>::Error;
	async fn get_application_ext<'a>(&self) -> Result<Self, Self::Error>
	where
		Self: Sized,
	{
		Ok(self.get_application().await?)
	}
	async fn get_parent_ext<'a>(&self) -> Result<Self, Self::Error>
	where
		Self: Sized,
	{
		Ok(self.parent().await?)
	}
	async fn get_children_indexes<'a>(&self) -> Result<Vec<i32>, Self::Error> {
		let mut indexes = Vec::new();
		for child in self.get_children_ext().await? {
			indexes.push(child.get_index_in_parent().await?);
		}
		Ok(indexes)
	}
	async fn get_children_ext<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		Ok(self.get_children().await?)
		/*
		let children_parts = self.get_children().await?;
		let mut children = Vec::new();
		for child_parts in children_parts {
			let acc = AccessibleProxy::builder(self.connection())
				.destination(child_parts.0)?
				.cache_properties(CacheProperties::No)
				.path(child_parts.1)?
				.build()
				.await?;
			children.push(acc);
		}
		Ok(children)
				*/
	}
	async fn get_siblings<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let parent = self.parent().await?;
		let pin = self.get_index_in_parent().await?;
		let index = pin.try_into()?;
		// Clippy false positive: Standard pattern for excluding index item from list.
		#[allow(clippy::if_not_else)]
		let children: Vec<Self> = parent
			.get_children()
			.await?
			.into_iter()
			.enumerate()
			.filter_map(|(i, a)| if i != index { Some(a) } else { None })
			.collect();
		Ok(children)
	}
	async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let parent = self.parent().await?;
		let index = self.get_index_in_parent().await?.try_into()?;
		let children: Vec<Self> = parent
			.get_children_ext()
			.await?
			.into_iter()
			.enumerate()
			.filter_map(|(i, a)| if i < index { Some(a) } else { None })
			.collect();
		Ok(children)
	}
	async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let parent = self.parent().await?;
		let index = self.get_index_in_parent().await?.try_into()?;
		let children: Vec<Self> = parent
			.get_children_ext()
			.await?
			.into_iter()
			.enumerate()
			.filter_map(|(i, a)| if i > index { Some(a) } else { None })
			.collect();
		Ok(children)
	}
	async fn get_children_caret<'a>(&self, backward: bool) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let mut children_after_before = Vec::new();
		let text_iface = self.to_text().await?;
		let caret_pos = text_iface.caret_offset().await?;
		let children_hyperlink = self.get_children_ext().await?;
		for child in children_hyperlink {
			let hyperlink = child.to_hyperlink().await?;
			if let Ok(start_index) = hyperlink.start_index().await {
				if (start_index <= caret_pos && backward) || (start_index >= caret_pos && !backward)
				{
					children_after_before.push(child);
				}
			// include all children which do not identify their positions, for some reason
			} else {
				children_after_before.push(child);
			}
		}
		Ok(children_after_before)
	}
	async fn edges<'a>(&self, backward: Option<bool>) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let mut edge_elements = Vec::new();
		let children = match backward {
			Some(backward) => {
				if let Ok(caret_children) = self.get_children_caret(backward).await {
					caret_children
				} else {
					self.get_children().await?
				}
			}
			None => self.get_children().await?,
		};
		children.into_iter().for_each(|child| edge_elements.push(child));
		let siblings = match backward {
			Some(false) => self.get_siblings_before().await?,
			Some(true) => self.get_siblings_after().await?,
			None => self.get_siblings().await?,
		};
		siblings.into_iter().for_each(|sibling| edge_elements.push(sibling));
		let parent = self.get_parent_ext().await?;
		edge_elements.push(parent);
		Ok(edge_elements)
	}
	async fn get_next<'a>(
		&self,
		matcher_args: &MatcherArgs,
		backward: bool,
		visited: &'a mut Vec<ObjectPair>,
	) -> Result<Option<Self>, Self::Error>
	where
		Self: Sized,
	{
		let mut stack: Vec<T> = Vec::new();
		let edges = self.edges(Some(backward)).await?;
		edges.into_iter().for_each(|edge| stack.push(edge));
		while let Some(item) = stack.pop() {
			// TODO: properly bubble up error
			let Ok(identifier) = ObjectPair::try_from(&item) else {
				return Ok(None);
			};
			// the top of the hirearchy for strctural navigation.
			if visited.contains(&identifier) {
				continue;
			}
			visited.push(identifier);
			if item.get_role().await? == Role::InternalFrame {
				return Ok(None);
			}
			// if it matches, then return it
			if item.match_(matcher_args).await? {
				return Ok(Some(item));
			}
			// if it doesnt match, add all edges
			self.edges(Some(backward))
				.await?
				.into_iter()
				.for_each(|edge| stack.push(edge));
		}
		return Ok(None);
	}
	async fn get_relation_set_ext<'a>(
		&self,
	) -> Result<HashMap<RelationType, Vec<Self>>, Self::Error>
	where
		Self: Sized,
	{
		let raw_relations = self.get_relation_set().await?;
		let mut relations = HashMap::new();
		for relation in raw_relations {
			let mut related_vec = Vec::new();
			for related in relation.1 {
				related_vec.push(related);
			}
			relations.insert(relation.0, related_vec);
		}
		Ok(relations)
	}
	// TODO: make match more broad, allow use of other parameters; also, support multiple roles, since right now, multiple will just exit immediately with false
	async fn match_(
		&self,
		matcher_args: &MatcherArgs,
	) -> Result<bool, <Self as AccessibleExt>::Error> {
		let roles = &matcher_args.0;
		if roles.len() != 1 {
			return Ok(false);
		}
		// our unwrap is protected from panicing with the above check
		Ok(self.get_role().await? == *roles.get(0).unwrap())
	}
}

impl<T: AccessibleBlocking + ConvertableBlocking + AccessibleBlockingExtError> AccessibleBlockingExt
	for T
{
}

assert_impl_all!(AccessibleProxy: Accessible, AccessibleExt);
assert_impl_all!(AccessibleProxyBlocking: AccessibleBlocking, AccessibleBlockingExt);
