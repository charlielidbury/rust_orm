// Can be stored as a row in a table
pub trait Storable<'a>: Clone + Query<Self> {
    fn create_statement(name: &str) -> String;

    // SQL which gets a row, and puts it in memory
    fn row_to_mem() -> String;
}

// Represents a table of rows of type T
#[derive(Clone)]
pub struct Table<T: Storable + Clone> {
    pub conn: Rc<Connection>,
    pub name: String,
    _fuck_you_rustc: PhantomData<T>,
}

impl<T: Storable + Clone> Table<T> {
    pub fn create(conn: Rc<Connection>, name: &str) -> Table<T> {
        // Creates table if not exists
        conn.execute(
            &T::create_statement(name),
            (), // empty list of parameters.
        )
        .unwrap();

        Table {
            conn,
            name: name.to_owned(),
            _fuck_you_rustc: PhantomData,
        }
    }
}

impl<T: Storable> Query<Vec<T>> for Table<T> {
    type FromData<'a> = ();
    type ToData = ();

    fn query(&self) -> Vec<T> {
        let mut stmt = self.conn.prepare(&self.to_sql(())).unwrap();

        let T_iter = stmt
            .query_map([], |row| Ok(T::from_sql((&row, 0))))
            .unwrap();

        T_iter.map(Result::unwrap).collect()
    }

    fn connection(&self) -> Rc<Connection> {
        self.conn.clone()
    }

    fn from_sql<'a>((): Self::FromData<'a>) -> Self {
        unimplemented!()
    }

    fn to_sql(&self, (): Self::ToData) -> String {
        format!("SELECT {} FROM {}", T::row_to_mem(), self.name)
    }
}

// This struct can be interpretted as a database
pub trait Database {
    fn connect() -> Self;
}
