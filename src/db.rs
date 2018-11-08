use lastfm;

use failure::Fail;

const SCHEMA: &'static str = "
    CREATE TABLE `tracks` (
        artist varchar(1024) NOT NULL,
        album varchar(1024) DEFAULT NULL,
        name varchar(1024) NOT NULL,
        timestamp integer(11) NOT NULL
    );
";

pub struct DB {
    conn: rusqlite::Connection,
}

impl DB {
    pub fn new<P: AsRef<std::path::Path>>(path: &P) -> failure::Fallible<DB> {
        let conn = if path.as_ref().exists() {
            rusqlite::Connection::open(path)
                .map_err(|e| {
                    let msg = format!(
                        "couldn't open db at {}",
                        path.as_ref().display()
                    );
                    e.context(msg)
                })?
        }
        else {
            Self::create(path)?
        };

        return Ok(DB { conn })
    }

    fn create<P: AsRef<std::path::Path>>(
        path: &P
    ) -> failure::Fallible<rusqlite::Connection> {
        eprintln!(
            "Initializing database at {}",
            path.as_ref().to_string_lossy(),
        );

        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
            let conn = rusqlite::Connection::open(path)
                .map_err(|e| {
                    let msg = format!(
                        "couldn't create db at {}",
                        path.as_ref().display()
                    );
                    e.context(msg)
                })?;
            conn.execute(SCHEMA, rusqlite::NO_PARAMS)
                .map_err(|e| e.context("failed to execute schema"))?;
            Ok(conn)
        }
        else {
            unimplemented!();
        }
    }

    pub fn most_recent_timestamp(&self) -> failure::Fallible<Option<i64>> {
        Ok(self.conn.query_row(
            "SELECT timestamp FROM tracks ORDER BY timestamp DESC LIMIT 1",
            rusqlite::NO_PARAMS,
            |row| Some(row.get(0))
        ).or_else(|e| {
            match e {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                _ => Err(e),
            }
        })?)
    }

    pub fn insert_tracks<F: FnMut(lastfm::Track)>(
        &self,
        tracks: impl Iterator<Item=lastfm::Track>,
        mut f: F
    ) -> failure::Fallible<()> {
        let mut sth = self.conn.prepare("INSERT INTO tracks VALUES (?, ?, ?, ?)")?;
        for track in tracks {
            sth.execute(
            &[
                &track.artist as &rusqlite::types::ToSql,
                &track.album,
                &track.name,
                &track.timestamp,
            ]).map(|_| ())?;
            f(track);
        }
        Ok(())
    }
}
