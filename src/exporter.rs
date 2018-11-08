use db;
use lastfm;

pub struct Exporter<'d, 'l> {
    db: &'d db::DB,
    lastfm: &'l lastfm::LastFMClient,
}

impl<'d, 'l> Exporter<'d, 'l> {
    pub fn new(
        db: &'d db::DB,
        lastfm: &'l lastfm::LastFMClient
    ) -> Exporter<'d, 'l> {
        Exporter {
            db,
            lastfm,
        }
    }

    pub fn tracks_to_sync(&self) -> failure::Fallible<u64> {
        let ts = self.db.most_recent_timestamp()?;
        Ok(self.lastfm.track_count(ts.map(|x| x + 1))?)
    }

    pub fn sync<F: FnMut(lastfm::Track)>(
        &self,
        track_cb: F,
    ) -> failure::Fallible<()> {
        let ts = self.db.most_recent_timestamp()?;
        self.db.insert_tracks(
            self.lastfm.tracks(ts.map(|x| x + 1)),
            track_cb
        )?;

        Ok(())
    }
}
