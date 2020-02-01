use super::{MediaBase, UserAvatar};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserBase {
    pub id: u32,
    pub name: String,
    pub site_url: String,
    pub avatar: UserAvatar,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub __typename: String,
    pub id: u32,
    pub created_at: u64,
    pub activity_type: String,
    pub site_url: String,
    pub user: Option<UserBase>,
    pub text: Option<String>,
    pub status: Option<String>,
    pub progress: Option<String>,
    pub media: Option<MediaBase>,
    pub recipient: Option<UserBase>,
    pub messenger: Option<UserBase>,
    pub message: Option<String>,
}

impl Default for Activity {
    fn default() -> Self {
        Activity {
            __typename: String::new(),
            id: 0,
            created_at: 0,
            activity_type: String::new(),
            site_url: String::new(),
            user: None,
            text: None,
            status: None,
            progress: None,
            media: None,
            recipient: None,
            messenger: None,
            message: None,
        }
    }
}

impl Activity {
    pub fn status(&self) -> String {
        let status = match &self.status {
            Some(status) => status.clone(),
            None => String::new(),
        };

        let progress = match &self.progress {
            Some(progress) => format!("{} of", progress.clone()),
            None => String::new(),
        };

        format!("{} {}", status, progress)
    }
}
