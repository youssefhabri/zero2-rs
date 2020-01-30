use crate::models::anilist::connection::MediaConnection;

#[derive(Deserialize, Debug)]
pub struct Studio {
    id: u32,

    name: String,

    #[serde(rename = "siteUrl")]
    site_url: String,

    media: MediaConnection,
}

impl Studio {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn site_url(&self) -> String {
        self.site_url.to_owned()
    }

    pub fn media(&self) -> String {
        let mut media_list: Vec<String> = vec![];

        let unwrap_score = |score: Option<u32>| -> String {
            match score {
                Some(score) => format!("{}", score),
                None => "N/A".to_string(),
            }
        };

        for media in self.media.nodes.iter().take(10) {
            media_list.push(format!(
                "[{}]({}) [Score: {}]",
                media.title.user_preferred,
                media.site_url,
                unwrap_score(media.mean_score)
            ))
        }

        media_list.join("\n")
    }
}
