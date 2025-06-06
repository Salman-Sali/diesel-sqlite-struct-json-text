use diesel::prelude::*;
use diesel_struct_json_text::diesel_struct_json_text;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub my_json_struct: MyJsonStruct
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
    pub my_json_struct: MyJsonStruct
}

impl Default for NewPost {
    fn default() -> Self {
        Self {
            title: "this is my title".into(),
            body: "this is my body".into(),
            published: true,
            my_json_struct: MyJsonStruct::default()
        }
    }
}

#[diesel_struct_json_text]
#[derive(Clone)]
pub struct MyJsonStruct {
    pub text: String,
    pub number: i32,
}

impl Default for MyJsonStruct {
    fn default() -> Self {
        Self {
            text: "hello world".into(),
            number: 2394,
        }
    }
}
