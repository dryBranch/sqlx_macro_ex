# Sql Macro Ex
Just for no compling time check.

## Example
```rust

#[derive(Debug, FromRow)]
struct User {
    uid: i32,
    name: String,
}

async fn insert_user(db: &DB, user: &User) -> sqlx::Result<()> {
    sqlx_macro_ex::query!(
        r#"
        insert into user(uid, name)
        values(?, ?)
        "#,
        &user.uid,
        &user.name,
    ).execute(mdb).await?;
    Ok(())
}

async fn get_user_with_uid(db: &DB, uid: i32) -> sqlx::Result<User> {
    sqlx_macro_ex::query_as!(
        User,
        r#"
        select * from user
        where uid = ?
        "#,
        uid
    ).fetch_one(db).await
}

async fn get_user_names__in_range(db: &DB, uid: i32) -> sqlx::Result<Vec<String>> {
    sqlx_macro_ex::query_scalar!(
        String,
        r#"
        select name from user
        where uid < ?
        "#,
        uid
    ).fetch_all(db).await
}
```