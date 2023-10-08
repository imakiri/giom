use std::path::Path;
use sqlite3::Connection;

struct Repository<'connection> {
    db: &'connection Connection,
}

enum Error {
    NotFound,
    Error(),
}

impl Repository {
    fn new(filename: &Path) -> Result<Box<Repository>, sqlite3::Error> {
        let connection = sqlite3::open(filename)?;

        connection.execute("CREATE ")?;







        let repository = Box::new(Repository { db: connection.borrow() });
        return Ok(repository);
    }
}