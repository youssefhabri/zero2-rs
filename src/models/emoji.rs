#[derive(Debug, Deserialize)]
pub struct Emoji {
    id: i32,
    pub title: String,
    slug: String,
    pub image: String,
    pub description: String,
    category: i32,
    license: String,
    source: String,
    faves: i32,
    pub submitted_by: String,
    width: i32,
    height: i32,
    filesize: i32,
}
