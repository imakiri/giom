use std::path::Path;
use rusqlite::Connection;
use log::{info, error};
use crate::repository::Error::{ConnectionError, MigrationError, PathError};

pub struct Repository {
    connection: Connection,
}

pub enum Error {
    PathError(&'static str),
    ConnectionError(rusqlite::Error),
    MigrationError(rusqlite::Error),
}

impl Error {
    pub fn to_string(self) -> String {
        match self {
            PathError(path) => { String::from(path) }
            ConnectionError(error) => { error.to_string() }
            MigrationError(error) => { error.to_string() }
        }
    }
}

impl Repository {
    pub fn new(filename: &Path) -> Result<Box<Repository>, Error> {
        match filename.to_str() {
            None => {
                error!("invalid path");
                return Err(PathError("invalid path"));
            }
            Some(path) => { info!("trying to connect to database at {}", path) }
        }
        let connection = match Connection::open(filename) {
            Ok(connection) => {
                info!("connected to database");
                connection
            }
            Err(err) => {
                error!("connection error: {}", err.to_string());
                return Err(ConnectionError(err));
            }
        };
        let migration = include_str!("migration.init.sql");
        info!("migration loaded \n{migration}", migration = migration);
        match connection.execute_batch(migration) {
            Ok(_) => {
                info!("migration is done")
            }
            Err(err) => {
                error!("connection error: {}", err.to_string());
                return Err(MigrationError(err));
            }
        }
        return Ok(Box::new(Repository { connection }));
    }
}

#[cfg(test)]
#[allow(unused_imports)]
#[allow(dead_code)]
mod tests {
    use std::path::Path;
    use rusqlite::Error;
    use crate::repository::Repository;
    use log::{info, log};

    #[test]
    fn new_repository() {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        match Repository::new(Path::new("main.db")) {
            Ok(_) => { info!("Repository::new") }
            Err(err) => { panic!("{}", err.to_string()) }
        };
    }
}

