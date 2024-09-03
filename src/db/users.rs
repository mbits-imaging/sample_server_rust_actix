use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
struct NewUser<'a> {
    name: &'a str,
}

pub fn insert_new_user(conn: &mut SqliteConnection, user_name: &str) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    // Create insertion model
    let new_user = NewUser { name: user_name };

    // normal diesel operations
    let user = diesel::insert_into(users)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn)?;

    Ok(user)
}

pub fn get_users(conn: &mut SqliteConnection) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;

    // normal diesel operations
    let result = users.load(conn)?;

    Ok(result)
}

pub fn get_user_by_id(conn: &mut SqliteConnection, user_id: i32) -> QueryResult<Option<User>> {
    use crate::schema::users::dsl::*;

    // normal diesel operations
    let user = users.find(user_id).first(conn).optional()?;

    Ok(user)
}

pub fn update_user_by_id(
    conn: &mut SqliteConnection,
    user_id: i32,
    user_name: &str,
) -> QueryResult<Option<User>> {
    use crate::schema::users::dsl::*;

    // normal diesel operations
    let user = diesel::update(users.find(user_id))
        .set(name.eq(user_name))
        .returning(User::as_returning())
        .get_result(conn)
        .optional()?;

    Ok(user)
}

pub fn delete_user_by_id(conn: &mut SqliteConnection, user_id: i32) -> QueryResult<Option<User>> {
    use crate::schema::users::dsl::*;

    // normal diesel operations
    let user = diesel::delete(users.find(user_id))
        .returning(User::as_returning())
        .get_result(conn)
        .optional()?;

    Ok(user)
}
