use crate::commands::anilist::utils::synopsis;
use crate::models::anilist::connection::{CharacterConnection, MediaConnection};
use crate::models::anilist::media::MediaType;

// TODO unify all image structs
#[derive(Clone, Deserialize, Debug)]
pub struct StaffImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct StaffName {
    pub first: Option<String>,
    pub last: Option<String>,
    pub native: Option<String>,
    pub alternative: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Staff {
    pub id: u32,
    pub site_url: String,
    pub name: StaffName,
    pub image: StaffImage,
    pub description: Option<String>,
    pub characters: CharacterConnection,
    pub staff_media: MediaConnection,
}

impl Staff {
    pub fn full_name(&self) -> String {
        let mut name_list = vec![];

        match &self.name.first {
            Some(first) => name_list.push(first.clone()),
            None => {}
        }

        match &self.name.last {
            Some(last) => name_list.push(last.clone()),
            None => {}
        }

        name_list.join(" ")
    }

    pub fn about(&self) -> String {
        match &self.description {
            Some(description) => synopsis(description, 300),
            None => String::new(),
        }
    }

    pub fn image(&self) -> String {
        match &self.image.large {
            Some(image) => image.to_string(),
            None => String::new(),
        }
    }

    pub fn media_list(&self, media_type: MediaType) -> String {
        let media_list = &self.staff_media.nodes;

        let mut fav_list: Vec<String> = vec![];

        if !media_list.is_empty() {
            let mut count = 0;
            for media in media_list {
                if media.media_type == media_type {
                    fav_list.push(format!(
                        "[{}]({})",
                        media.title.user_preferred, media.site_url
                    ));
                    count += 1;
                }
                if count >= 5 {
                    break;
                }
            }
        }

        if !fav_list.is_empty() {
            if media_list
                .iter()
                .filter(|media| media.media_type == media_type)
                .count()
                > 5
            {
                return format!("{}\n + {} more", fav_list.join("\n"), media_list.len() - 5);
            }

            return fav_list.join("\n");
        }

        String::from("N/A")
    }
}
