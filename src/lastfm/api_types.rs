#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[derive(Deserialize)]
pub struct track_artist {
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Deserialize)]
pub struct track_album {
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Deserialize)]
pub struct track_date {
    pub uts: String, // no idea why this is a string either
}

#[derive(Deserialize)]
pub struct track {
    pub artist: track_artist,
    pub name: String,
    pub album: track_album,
    pub date: track_date,
}

#[derive(Deserialize)]
pub struct recent_tracks_recenttracks_attr {
    pub total: String, // no idea why this is a string
    pub totalPages: String,
}

#[derive(Deserialize)]
pub struct recent_tracks_recenttracks {
    pub track: Vec<track>,
    #[serde(rename = "@attr")]
    pub attr: recent_tracks_recenttracks_attr,
}

#[derive(Deserialize)]
pub struct recent_tracks {
    pub recenttracks: recent_tracks_recenttracks,
}

