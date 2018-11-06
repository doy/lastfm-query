use error::Result;

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
    pub fn new<P: AsRef<std::path::Path>>(path: &P) -> Result<DB> {
        let conn = if path.as_ref().exists() {
            rusqlite::Connection::open(path)?
        }
        else {
            Self::create(path)?
        };

        return Ok(DB { conn })
    }

    fn create<P: AsRef<std::path::Path>>(
        path: &P
    ) -> Result<rusqlite::Connection> {
        println!(
            "Initializing database at {}",
            path.as_ref().to_string_lossy(),
        );

        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
            let conn = rusqlite::Connection::open(path)?;
            conn.execute(SCHEMA, rusqlite::NO_PARAMS)?;
            Ok(conn)
        }
        else {
            unimplemented!();
        }
    }
}
