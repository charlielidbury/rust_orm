mod orm;
use orm::*;
use rusqlite::{params, Connection, Result};

#[derive(Storable, Debug)]
struct User {
    id: u32,
    name: String,
}

#[derive(Database)]
struct Db {
    users: Table<User>,
}

fn main() -> Result<()> {
    let db = Db::connect();

    let me = User {
        id: 0,
        name: "Steven".to_string(),
    };

    db.users
        .conn
        .execute("INSERT INTO users (name) VALUES (?1)", (&me.name,))?;

    let mut stmt = db.users.conn.prepare("SELECT id, name FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for user in user_iter {
        println!("Found user {:?}", user.unwrap());
    }

    Ok(())
}
