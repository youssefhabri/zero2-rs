use serde::Serialize;

use crate::models::{AniListID, MediaType};

pub trait Variables: Send + Sync + erased_serde::Serialize {}

erased_serde::serialize_trait_object!(Variables);

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaVariables {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<AniListID>,

    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<MediaType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    is_adult: Option<bool>,
}

impl Variables for MediaVariables {}

impl MediaVariables {
    pub fn id(&mut self, id: AniListID) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn search(&mut self, keyword: impl ToString) -> &mut Self {
        self.search = Some(keyword.to_string());
        self
    }

    pub fn r#type(&mut self, r#type: MediaType) -> &mut Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn is_adult(&mut self, is_adult: bool) -> &mut Self {
        self.is_adult = Some(is_adult);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardVariables {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<AniListID>,

    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
}

impl Variables for StandardVariables {}

impl StandardVariables {
    pub fn id(&mut self, id: AniListID) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn search(&mut self, keyword: impl ToString) -> &mut Self {
        self.search = Some(keyword.to_string());
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringScheduleVariables {
    start_date: u64,
    end_date: u64,
}

impl Variables for AiringScheduleVariables {}

impl AiringScheduleVariables {
    pub fn new(start_date: u64, end_date: u64) -> Self {
        AiringScheduleVariables {
            start_date,
            end_date,
        }
    }
}
