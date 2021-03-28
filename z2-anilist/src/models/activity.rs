use serde::Deserialize;

use super::media::MediaBase;
use super::user::UserBase;
use crate::utils::{na_long_str, synopsis};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    Text,
    AnimeList,
    MangaList,
    Message,
    MediaList,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ActivityUnion {
    TextActivity,
    ListActivity,
    MessageActivity,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(rename = "__typename")]
    pub __typename: ActivityUnion,
    pub id: u32,
    pub r#type: ActivityType,
    pub created_at: u64,
    pub site_url: String,
    pub user: Option<UserBase>,

    // MessageActivity fields
    pub text: Option<String>,

    // ListActivity fields
    status: Option<String>,
    progress: Option<String>,
    pub media: Option<MediaBase>,

    // MessageActivity fields
    pub recipient: Option<UserBase>,
    pub messenger: Option<UserBase>,
    pub message: Option<String>,
}

impl Activity {
    pub fn status(&self) -> String {
        let status = self.status.clone().unwrap_or_else(String::new);
        let progress = self.progress.clone().unwrap_or_else(String::new);

        format!("{} {}", status, progress)
    }

    pub fn description(&self) -> String {
        match self.__typename {
            ActivityUnion::TextActivity => {
                let text = self.text.clone();
                text.map_or_else(na_long_str, |text| synopsis(text, 1000))
            }
            ActivityUnion::ListActivity => {
                let media = self.media.clone().unwrap();
                format!(
                    "**{} [{}]({})**",
                    self.status().trim(),
                    media.title.user_preferred,
                    media.site_url
                )
            }
            ActivityUnion::MessageActivity => {
                let recipient = self.recipient.clone().unwrap();
                let message = self.message.clone().unwrap_or_else(na_long_str);
                format!(
                    "**Sent a message to [{}]({})**\n\n{}",
                    recipient.name, recipient.site_url, message
                )
            }
        }
    }
}
