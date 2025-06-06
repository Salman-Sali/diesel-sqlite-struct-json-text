### Your cargo.toml
```toml
[dependencies]
diesel = { version="*", features= ["sqlite"]}
serde = {version = "*", features = ["derive"]}
serde_json = "*"
```


### The target struct
```rust 
#[diesel_struct_json_text]
pub struct MyJson {
    pub my_item: String,
    pub my_item_2: i32
}
```
### The database entity
```rust 
#[diesel(table_name = crate::schema::my_entity)]
#[derive(Debug, Queryable, Selectable, Identifiable, Associations, PartialEq, Insertable)]
pub struct MyEntity {
    pub id: i32,
    pub my_json: MyJson
}
```
### The schema
```rust 
diesel::table! {
    my_entity (id) {
        id -> Integer,
        my_json -> Text
    }
}
```