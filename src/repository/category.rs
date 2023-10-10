use crate::core;
use rusqlite::{Connection, Error};
use uuid::Bytes;

type Category = core::Category<Bytes, Vec<u8>>;

type CategoryLinks = core::CategoryLink<Bytes, Vec<u8>, String>;

pub fn list_all_categories(connection: Connection) -> Result<Vec<Category>, Error> {
    let mut stmt = connection.prepare("SELECT id, name, description, icon FROM categories")?;
    let rows = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            icon: row.get(3)?,
        })
    })?;

    let mut categories = Vec::new();
    for category in rows {
        categories.push(category?);
    }

    return Ok(categories);
}