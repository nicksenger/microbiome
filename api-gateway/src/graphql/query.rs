use juniper::FieldResult;

use super::schema::Context;
use crate::entities::{User};

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn user_by_id(ctx: &Context, id: i32) -> FieldResult<User> {
        Ok(ctx.user_data.user_by_id(id).await)
    }
}