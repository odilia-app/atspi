use crate::{
    accessible::{AccessibleProxy, RelationType, Role, Accessible},
    collection::MatchType,
    convertable::Convertable,
    error::ObjectPathConversionError,
		AtspiError,
    InterfaceSet,
};
use async_recursion::async_recursion;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use zbus::{
    zvariant::{ObjectPath, OwnedObjectPath},
    CacheProperties,
};

pub type MatcherArgs =
    (Vec<Role>, MatchType, HashMap<String, String>, MatchType, InterfaceSet, MatchType);

#[async_trait]
pub trait AccessibleExt {
	type Error: std::error::Error;
    // Assumes that an accessible can be made from the component parts
    fn get_id(&self) -> Option<AccessibleId>;
    async fn get_parent_ext<'a>(&self) -> Result<Self, Self::Error> where Self: Sized;
    async fn get_children_ext<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized;
    async fn get_siblings<'a>(&self) -> Result<Vec<Self>, Box<dyn Error>> where Self: Sized;
    async fn get_children_indexes<'a>(&self) -> Result<Vec<i32>, Self::Error>;
    async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Box<dyn Error>> where Self: Sized;
    async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Box<dyn Error>> where Self: Sized;
    async fn get_ancestors(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized;
    async fn get_ancestor_with_role<'a>(&self, role: Role) -> Result<Self, Self::Error> where Self: Sized;
    /* TODO: not sure where these should go since it requires both Text as a self interface and
     * Hyperlink as children interfaces. */
    async fn get_children_caret<'a>(&self, after: bool) -> Result<Vec<Self>, Self::Error> where Self: Sized;
    async fn get_next<'a>(
        &self,
        matcher_args: &MatcherArgs,
        backward: bool,
    ) -> Result<Option<Self>, Self::Error> where Self: Sized;
    async fn get_relation_set_ext<'a>(
        &self,
    ) -> Result<HashMap<RelationType, Vec<Self>>, Self::Error> where Self: Sized;
    async fn find_inner<'a>(
        &self,
        after_or_before: i32,
        matcher_args: &MatcherArgs,
        backward: bool,
        recur: bool,
    ) -> Result<Option<Self>, <Self as AccessibleExt>::Error> where Self: Sized;
	async fn match_(
			&self,
			matcher_args: &MatcherArgs,
	) -> Result<bool, <Self as AccessibleExt>::Error>;
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AccessibleId {
    Null,
    Root,
    Number(i64),
}
impl ToString for AccessibleId {
    fn to_string(&self) -> String {
        let ending = match self {
            Self::Null => "null".to_string(),
            Self::Root => "root".to_string(),
            Self::Number(int) => int.to_string(),
        };
        format!("/org/a11y/atspi/accessible/{ending}")
    }
}
impl<'a> TryInto<ObjectPath<'a>> for AccessibleId {
    type Error = zbus::zvariant::Error;

    fn try_into(self) -> Result<ObjectPath<'a>, Self::Error> {
        ObjectPath::try_from(self.to_string())
    }
}

impl TryFrom<OwnedObjectPath> for AccessibleId {
    type Error = ObjectPathConversionError;

    fn try_from(path: OwnedObjectPath) -> Result<Self, Self::Error> {
        match path.split('/').next_back() {
            Some("null") => Ok(AccessibleId::Null),
            Some("root") => Ok(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Ok(AccessibleId::Number(uid)),
                Err(e) => Err(Self::Error::ParseError(e)),
            },
            None => Err(Self::Error::NoIdAvailable),
        }
    }
}
impl<'a> TryFrom<ObjectPath<'a>> for AccessibleId {
    type Error = ObjectPathConversionError;

    fn try_from(path: ObjectPath<'a>) -> Result<Self, Self::Error> {
        match path.split('/').next_back() {
            Some("null") => Ok(AccessibleId::Null),
            Some("root") => Ok(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Ok(AccessibleId::Number(uid)),
                Err(e) => Err(Self::Error::ParseError(e)),
            },
            None => Err(Self::Error::NoIdAvailable),
        }
    }
}
impl<'a> TryFrom<&ObjectPath<'a>> for AccessibleId {
    type Error = ObjectPathConversionError;

    fn try_from(path: &ObjectPath<'a>) -> Result<Self, Self::Error> {
        match path.split('/').next_back() {
            Some("null") => Ok(AccessibleId::Null),
            Some("root") => Ok(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Ok(AccessibleId::Number(uid)),
                Err(e) => Err(Self::Error::ParseError(e)),
            },
            None => Err(Self::Error::NoIdAvailable),
        }
    }
}

#[async_trait]
impl<'c, T: Accessible + Convertable + Sync + std::ops::Deref<Target = zbus::Proxy<'c>> + Send> AccessibleExt for T where 
	<T as Convertable>::Error: Into<AtspiError>,
	<T as Accessible>::Error: Into<AtspiError>,
	AtspiError: From<<T as Accessible>::Error>,
	AtspiError: From<<T as Convertable>::Error> {
		type Error = AtspiError;
    /// get_id gets the id (if available) for any accessible.
    /// This *should* always return a Some(i32) and never None, but you never know.
    /// Sometimes, a path (`/org/a11y/atspi/accessible/XYZ`) may contain a special value for `XYZ`.
    /// For example: "null" (invalid item), or "root" (the ancestor of all accessibles).
    /// It *should* be safe to `.expect()` the return type.
    fn get_id(&self) -> Option<AccessibleId> {
        let path = self.path();
        match path.split('/').next_back() {
            Some("null") => Some(AccessibleId::Null),
            Some("root") => Some(AccessibleId::Root),
            Some(id) => match id.parse::<i64>() {
                Ok(uid) => Some(AccessibleId::Number(uid)),
                _ => None,
            },
            _ => None,
        }
    }
    async fn get_parent_ext<'a>(&self) -> Result<Self, Self::Error> where Self: Sized {
        Ok(self.parent().await?)
    }
    async fn get_children_indexes<'a>(&self) -> Result<Vec<i32>, Self::Error> {
        let mut indexes = Vec::new();
        for child in self.get_children_ext().await? {
            indexes.push(child.get_index_in_parent().await?);
        }
        Ok(indexes)
    }
    async fn get_children_ext<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized{
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
    async fn get_siblings<'a>(&self) -> Result<Vec<Self>, Box<dyn Error>> where Self: Sized {
        let parent = self.parent().await?;
        let index = self.get_index_in_parent().await?.try_into()?;
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
    async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Box<dyn Error>> where Self: Sized {
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
    async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Box<dyn Error>> where Self: Sized {
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
    async fn get_ancestors(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized {
        let mut ancestors = Vec::new();
        let mut ancestor = self.get_parent_ext().await?;
        while ancestor.get_role().await? != Role::Frame {
            ancestors.push(ancestor.clone());
            ancestor = ancestor.get_parent_ext().await?;
        }
        Ok(ancestors)
    }
    async fn get_ancestor_with_role<'a>(&self, role: Role) -> Result<Self, Self::Error> where Self: Sized {
        let mut ancestor = self.get_parent_ext().await?;
				let mut ancestor_role = ancestor.get_role().await?;
        while ancestor_role != role && ancestor_role != Role::Frame {
            ancestor = ancestor.get_parent_ext().await?;
						ancestor_role = ancestor.get_role().await?;
        }
        Ok(ancestor)
    }
    async fn get_children_caret<'a>(
        &self,
        backward: bool,
    ) -> Result<Vec<Self>, Self::Error> where Self: Sized {
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
    async fn get_next<'a>(
        &self,
        matcher_args: &MatcherArgs,
        backward: bool,
    ) -> Result<Option<Self>, Self::Error> where Self: Sized {
        // TODO if backwards, check here
        let caret_children = self.get_children_caret(backward).await?;
        for child in caret_children {
            if child.match_(matcher_args).await? {
                return Ok(Some(child));
            } else if let Some(found_sub) =
                child.find_inner(0, matcher_args, backward, true).await?
            {
                return Ok(Some(found_sub));
            }
        }
        let mut last_parent_index = self.get_index_in_parent().await?;
        if let Ok(mut parent) = self.get_parent_ext().await {
            while parent.get_role().await? != Role::InternalFrame {
                let found_inner_child = parent
                    .find_inner(last_parent_index, matcher_args, backward, false)
                    .await?;
                if found_inner_child.is_some() {
                    return Ok(found_inner_child);
                }
                last_parent_index = parent.get_index_in_parent().await?;
                parent = parent.get_parent_ext().await?;
            }
        }
        Ok(None)
    }
    async fn get_relation_set_ext<'a>(
        &self,
    ) -> Result<HashMap<RelationType, Vec<Self>>, Self::Error> where Self: Sized {
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
    async fn find_inner<'a>(
        &self,
        after_or_before: i32,
        matcher_args: &MatcherArgs,
        backward: bool,
        recur: bool,
    ) -> Result<Option<Self>, <Self as AccessibleExt>::Error> where Self: Sized {
        let children = if backward {
            let mut vec = self.get_children_ext().await?;
            vec.reverse();
            vec
        } else {
            self.get_children_ext().await?
        };
        for child in children {
            let child_index = child.get_index_in_parent().await?;
            if !recur
                && ((child_index <= after_or_before && !backward)
                    || (child_index >= after_or_before && backward))
            {
                continue;
            }
            if child.match_(matcher_args).await? {
                return Ok(Some(child));
            }
            /* 0 here is ignored because we are recursive; see the line starting with if !recur */
            if let Some(found_decendant) = child.find_inner(0, matcher_args, backward, true).await?
            {
                return Ok(Some(found_decendant));
            }
        }
        Ok(None)
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
