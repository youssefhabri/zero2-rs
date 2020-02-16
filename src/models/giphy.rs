#[derive(Deserialize, Serialize, Debug)]
pub struct GiphyImageOriginal {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GiphyImages {
    pub original: GiphyImageOriginal,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Giphy {
    pub url: String,
    pub title: String,
    pub images: GiphyImages,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GiphyResponse {
    pub data: Vec<Giphy>,
}
