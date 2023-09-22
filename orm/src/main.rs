mod orm;
use std::{fmt::format, rc::Rc};

use orm::*;
use rusqlite::{params, Connection, Result, Row};

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

impl RowQuery<User> for User {
    fn query(&self) -> User {
        self.clone()
    }

    fn from_data(row: &Row, i: usize) -> Self {
        User {
            id: row.get(i + 0).unwrap(),
            name: row.get(i + 1).unwrap(),
        }
    }

    fn sql(&self) -> String {
        let i = 0;
        format!("{} AS a{}, {} AS a{}", self.id, i, self.name, i + 1)
    }
}

impl Storable for User {
    fn sql_create_table(name: &str) -> String {
        format!(
            "
                CREATE TABLE IF NOT EXISTS {} (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL
                )
            ",
            name
        )
    }

    fn row_to_mem() -> String {
        format!("id AS a0, name AS a1")
    }
}

struct Db {
    users: Table<User>,
}

impl Database for Db {
    fn connect() -> Self {
        let con = Rc::new(Connection::open_in_memory().unwrap());

        Self {
            users: Table::create(con.clone(), "users"),
        }
    }
}

fn main() -> Result<()> {
    let db = Db::connect();

    let me = User {
        id: 0,
        name: "Steven".to_string(),
    };

    db.users
        .con
        .execute("INSERT INTO users (name) VALUES (?1)", (&me.name,))?;

    // Get the name of all dada
    let users = db.users
        .map(|u| u.name)
        .query();

    dbg!(users);

    Ok(())
}
