use failure;
use reqwest;

use error::Result;

mod api_types;

const API_ROOT: &'static str = "https://ws.audioscrobbler.com/2.0/";

pub struct LastFMClient {
    client: reqwest::Client,
    api_key: String,
    user: String,
}

pub struct Track {
    pub artist: String,
    pub album: String,
    pub name: String,
    pub timestamp: i64,
}

pub struct Tracks<'a> {
    client: &'a LastFMClient,
    page: Option<u64>,
    buf: Vec<Track>,
    from: Option<i64>,
}

impl<'a> Tracks<'a> {
    fn new(client: &LastFMClient, from: Option<i64>) -> Tracks {
        Tracks {
            client,
            page: None,
            buf: vec![],
            from,
        }
    }

    fn get_next_page(&mut self) -> Result<()> {
        if !self.page.is_some() {
            self.page = Some(self.client.get_total_pages(self.from)?);
        }
        let page = self.page.unwrap();
        if page < 1 {
            return Ok(())
        }

        let req = self.client.client
            .get(API_ROOT)
            .query(&[
                ("method", "user.getrecenttracks"),
                ("api_key", &self.client.api_key),
                ("user", &self.client.user),
                ("format", "json"),
                ("page", &format!("{}", page)),
                ("limit", "200"),
            ]);
        let req = if let Some(from) = self.from {
            req.query(&[("from", &format!("{}", from))])
        }
        else {
            req
        };

        let mut res = req.send()?;

        if res.status().is_success() {
            let data: api_types::recent_tracks = res.json()?;
            self.buf = data.recenttracks.track
                .iter()
                .filter(|t| { t.date.is_some() })
                .map(|t| {
                    Ok(Track {
                        artist: t.artist.text.clone(),
                        album: t.album.text.clone(),
                        name: t.name.clone(),
                        timestamp: t.date.as_ref().unwrap().uts.parse()?,
                    })
                })
                .collect::<Result<Vec<Track>>>()?;
            self.page = Some(page - 1);
            Ok(())
        }
        else {
            Err(failure::err_msg(res.status().as_str().to_string()))
        }
    }
}

impl<'a> Iterator for Tracks<'a> {
    type Item = Track;

    fn next(&mut self) -> Option<Track> {
        if self.buf.len() == 0 {
            let result = self.get_next_page();
            if result.is_err() {
                return None;
            }
        }
        self.buf.pop()
    }
}

impl LastFMClient {
    pub fn new(api_key: &str, user: &str) -> LastFMClient {
        LastFMClient {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            user: user.to_string(),
        }
    }

    pub fn track_count(&self, from: Option<i64>) -> Result<u64> {
        let data = self.recent_tracks(from)?;
        Ok(data.recenttracks.attr.total.parse()?)
    }

    pub fn tracks(&self, from: Option<i64>) -> Tracks {
        Tracks::new(&self, from)
    }

    fn get_total_pages(&self, from: Option<i64>) -> Result<u64> {
        let data = self.recent_tracks(from)?;
        Ok(data.recenttracks.attr.totalPages.parse()?)
    }

    fn recent_tracks(
        &self,
        from: Option<i64>,
    ) -> Result<api_types::recent_tracks> {
        let req = self.client
            .get(API_ROOT)
            .query(&[
                ("method", "user.getrecenttracks"),
                ("api_key", &self.api_key),
                ("user", &self.user),
                ("format", "json"),
                ("limit", "200"),
            ]);
        let req = if let Some(from) = from {
            req.query(&[("from", &format!("{}", from))])
        }
        else {
            req
        };

        let mut res = req.send()?;

        if res.status().is_success() {
            let data: api_types::recent_tracks = res.json()?;
            Ok(data)
        }
        else {
            Err(failure::err_msg(res.status().as_str().to_string()))
        }
    }
}
