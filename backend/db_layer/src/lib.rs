#[macro_use]
extern crate diesel;
pub mod schema;

pub mod todo {
    use crate::schema::*;
    use diesel::{Insertable, Queryable};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Queryable)]
    pub struct Todo {
        pub id: i32,
        pub descr: String,
        pub date_created: String,
    }

    #[derive(Debug, Insertable, Serialize)]
    #[table_name = "todos"]
    pub struct TodoNew {
        pub descr: String,
        pub date_created: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TodoForm {
        pub descr: String,
    }
}

pub mod db {
    pub use diesel::prelude;
    pub use diesel::RunQueryDsl;
    pub use diesel::{
        delete, insert_into,
        r2d2::{ConnectionManager, Pool},
        SqliteConnection,
    };
    #[tracing::instrument]
    pub fn get_pool() -> Pool<ConnectionManager<SqliteConnection>> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("db not found");
        Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap()
    }
}
