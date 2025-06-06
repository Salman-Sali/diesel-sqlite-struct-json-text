use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_attribute]
pub fn diesel_sqlite_struct_json_text(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    return quote!{
        #[derive(Debug, PartialEq, diesel::FromSqlRow, diesel::AsExpression, Eq, serde::Serialize, serde::Deserialize)]
        #[diesel(sql_type = diesel::sql_types::Text)]
        #input

        impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for #name {
            fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
                let json = serde_json::to_string(self).unwrap();
                out.set_value(json);
                Ok(diesel::serialize::IsNull::No)                
            }
        }

        impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for #name {
            fn from_sql(bytes: diesel::sqlite::SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
                let value = <String as diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
                Ok(serde_json::from_str(&value).unwrap())
            }
        }
    }.into();
}