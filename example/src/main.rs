mod models;
mod schema;

use diesel::prelude::*;
use diesel_migrations::*; 
use models::*;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
fn main() {
    let mut connection = SqliteConnection::establish("./Database.db").unwrap();
    let test = connection.run_pending_migrations(MIGRATIONS);
    println!("{:?}", test);

    let result = diesel::insert_into(schema::posts::dsl::posts)
        .values(NewPost::default())
        .returning(Post::as_returning())
        .get_result::<Post>(&mut connection);

    println!("{:?}", result);
}