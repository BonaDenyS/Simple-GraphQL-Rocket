use crate::db::Database;
use crate::entities::prelude::User as UserEntity;
use async_graphql::{Context, Object, SimpleObject};
use sea_orm::{EntityTrait, QueryOrder};

/// GraphQL-safe type
#[derive(SimpleObject)]
pub struct UserObject {
    pub id: i32,
    pub name: String,
    pub email: String,
}

/// Convert SeaORM Model → GraphQL Object
impl From<crate::entities::user::Model> for UserObject {
    fn from(m: crate::entities::user::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            email: m.email,
        }
    }
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<UserObject>> {
        let db = ctx.data::<Database>()?;
        let conn = db.conn.as_ref();

        let users = UserEntity::find()
            .order_by_asc(crate::entities::user::Column::Id)
            .all(conn)
            .await?;

        Ok(users.into_iter().map(UserObject::from).collect())
    }

    pub async fn user(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Vec<UserObject>> {
        let db = ctx.data::<Database>()?;
        let conn = db.conn.as_ref();

        let users = UserEntity::find_by_id(id)
            .order_by_asc(crate::entities::user::Column::Id)
            .all(conn)
            .await?;

        Ok(users.into_iter().map(UserObject::from).collect())
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        name: String,
        email: String,
    ) -> async_graphql::Result<UserObject> {
        let db = ctx.data::<Database>()?;
        let conn = db.conn.as_ref();

        let active = crate::entities::user::ActiveModel {
            name: sea_orm::ActiveValue::set(name),
            email: sea_orm::ActiveValue::set(email),
            ..Default::default()
        };

        let res = UserEntity::insert(active).exec(conn).await?;

        let model = UserEntity::find_by_id(res.last_insert_id)
            .one(conn)
            .await?
            .expect("Inserted row missing");

        Ok(UserObject::from(model))
    }
}
