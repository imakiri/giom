use crate::core;
use rusqlite::Connection;
use crate::core::CategoryLink;

type Category = core::Category<Vec<u8>>;

type CategoryLinks = core::CategoryLink<Vec<u8>, String>;

pub fn list_all_categories(connection: Connection) -> Vec<Category> {



}