use lastfm;

use failure::Fail;

const SCHEMA: &'static str = "
    CREATE TABLE `tracks` (
        artist varchar(1024) NOT NULL,
        album varchar(1024) DEFAULT NULL,
        name varchar(1024) NOT NULL,
        timestamp integer(11) NOT NULL
    );
    CREATE VIEW `yearly_tracks` as
        SELECT *
        FROM tracks
        WHERE strftime('%s') - timestamp < 60*60*24*365;
    CREATE VIEW `monthly_tracks` as
        SELECT *
        FROM tracks
        WHERE strftime('%s') - timestamp < 60*60*24*30;
    CREATE VIEW `weekly_tracks` as
        SELECT *
        FROM tracks
        WHERE strftime('%s') - timestamp < 60*60*24*7;
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

    pub fn insert_tracks(
        &self,
        tracks: impl Iterator<Item=lastfm::Track>,
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
        }
        Ok(())
    }

    pub fn query<F: Fn(&rusqlite::Row)>(
        &self,
        query: &str,
        f: F
    ) -> failure::Fallible<Vec<String>> {
        let mut sth = self.conn.prepare(query)?;

        let cols = sth.column_names()
            .iter()
            .map(|s| s.to_string())
            .collect();

        let rows = sth.query_and_then(
            rusqlite::NO_PARAMS,
            |row| { f(row); Ok(()) },
        )?;
        // this call to collect() forces it to actually consume the iterator
        // (and therefore call the callbacks). what i really want here is for
        // there to be a query_for_each or something like that, but the weird
        // way lifetimes work for rows makes it difficult to emulate this any
        // other way
        let errs: failure::Fallible<Vec<()>> = rows.collect();

        errs.map(|_| cols)
    }
}
