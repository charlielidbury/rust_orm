use std::{marker::PhantomData, rc::Rc};

pub use orm_derive::*;
use rusqlite::Connection;

// Can be stored as a row in a table
pub trait Storable {
    fn create_statement(name: &str) -> String;
}

// Can be stored as a cell in a table
pub trait CellStorable {}

// Can be queried to form a T
pub trait Query<T> {
    fn query(&self) -> T;
}

// This struct can be interpretted as a database
pub trait Database {
    fn connect() -> Self;
}

// Represents a table of rows of type T
pub struct Table<T: Storable> {
    pub conn: Rc<Connection>,
    _fuck_you_rustc: PhantomData<T>,
}

impl<T: Storable> Table<T> {
    pub fn create(conn: Rc<Connection>, name: &str) -> Table<T> {
        conn.execute(
            &T::create_statement(name),
            (), // empty list of parameters.
        )
        .unwrap();

        Table {
            conn,
            _fuck_you_rustc: PhantomData,
        }
    }
}

// impl<T: Storable> Query<Vec<T>> for Table<T> {
//     fn query(&self) -> Vec<T> {
//         let mut stmt = self.conn.prepare("SELECT id, name FROM users").unwrap();

//         let user_iter = stmt.query_map([], |row| Ok(T::from_sql_row(row)));

//         user_iter.collect()
//     }
// }
