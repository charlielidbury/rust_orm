use std::{marker::PhantomData, rc::Rc};

pub use orm_derive::*;
use rusqlite::{Connection, Row, Rows};

// Can be queried to form a T
pub trait Query<T>: Clone {
    fn query(&self) -> Vec<T>;

    fn from_data<'a>(data: Rows) -> Self;

    fn sql(&self) -> String;
}

pub trait RowQuery<T>: Clone {
    fn query(&self) -> T;

    fn from_data(data: &Row, i: usize) -> Self;

    fn sql(&self) -> String;
}

// Can be stored as a row in a table
pub trait Storable: Clone + RowQuery<Self> {
    fn sql_create_table(name: &str) -> String;

    // SQL which gets a row, and puts it in memory
    fn row_to_mem() -> String;
}

pub trait CellStorable: Clone {}

impl CellStorable for i32 {}
impl CellStorable for String {}

// Represents a table of rows of type T
#[derive(Clone)]
pub struct Table<T: Storable + Clone> {
    pub con: Rc<Connection>,
    pub name: String,
    _fuck_you_rustc: PhantomData<T>,
}

impl<T: Storable + Clone> Table<T> {
    pub fn create(conn: Rc<Connection>, name: &str) -> Table<T> {
        // Creates table if not exists
        conn.execute(
            &T::sql_create_table(name),
            (), // empty list of parameters.
        )
        .unwrap();

        Table {
            con: conn,
            name: name.to_owned(),
            _fuck_you_rustc: PhantomData,
        }
    }
}

impl<T: Storable> Query<T> for Table<T> {
    fn query(&self) -> Vec<T> {
        let mut stmt = self.con.prepare(&self.sql()).unwrap();

        let T_iter = stmt.query_map([], |row| Ok(T::from_data(row, 0))).unwrap();

        T_iter.map(Result::unwrap).collect()
    }

    fn from_data<'a>(data: Rows) -> Self {
        unimplemented!()
    }

    fn sql(&self) -> String {
        format!("SELECT {} FROM {}", T::row_to_mem(), self.name)
    }
}

// This struct can be interpretted as a database
pub trait Database {
    fn connect() -> Self;
}
