use crate::{
    accessible::{RelationType, Role, Accessible},
    collection::MatchType,
    convertable::Convertable,
		text::Text,
		hyperlink::Hyperlink,
		AtspiError,
    InterfaceSet,
};
use async_trait::async_trait;
use std::collections::HashMap;

pub type MatcherArgs =
    (Vec<Role>, MatchType, HashMap<String, String>, MatchType, InterfaceSet, MatchType);

#[async_trait]
pub trait AccessibleExt {
	type Error: std::error::Error;
    async fn get_parent_ext<'a>(&self) -> Result<Self, Self::Error> where Self: Sized;
    async fn get_children_ext<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized;
    async fn get_siblings<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized;
    async fn get_children_indexes<'a>(&self) -> Result<Vec<i32>, Self::Error>;
    async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized;
    async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized;
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

#[async_trait]
impl<T: Accessible + Convertable + Sync + Send> AccessibleExt for T where 
	<T as Convertable>::Error: Into<AtspiError>,
	<T as Accessible>::Error: Into<AtspiError>,
	AtspiError:
			From<<T as Accessible>::Error>
		+ From<<T as Convertable>::Error>
		+ From<<<T as Convertable>::Text as Text>::Error> {
		type Error = AtspiError;
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
    async fn get_siblings<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized {
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
    async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized {
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
    async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Self::Error> where Self: Sized {
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
