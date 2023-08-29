/// 用于替换 sqlx::query!()
///
/// 无需数据库编译期验证
/// ## 参数
/// - t
/// - sql
/// - params*
#[macro_export]
macro_rules! query {
    ( $sql: expr, $($p: expr),* ) => {
        sqlx::query($sql)
            $(.bind($p))*
    };
}

/// 用于替换 sqlx::query_as!()
///
/// 无需数据库编译期验证
/// ## 参数
/// - t
/// - sql
/// - params*
#[macro_export]
macro_rules! query_as {
    ( $t: ty, $sql: expr, $($p: expr),* ) => {
        sqlx::query_as::<_, $t>($sql)
            $(.bind($p))*
    };
}

/// 将所有数据查询出来，以 `Result<Vec<xxxRow>>` 的形式返回
///
/// ## 参数
/// - db
/// - sql
/// - params*
#[macro_export]
macro_rules! query_all {
    ( $db: expr, $sql: expr, $($p: expr),* $(,)? ) => {
        $crate::query!($sql, $($p),*)
            .fetch_all($db)
            .await
    };
}

/// 将所有数据查询出来，以 `Result<Vec<T>>` 的形式返回
///
/// `T` 要求实现 `FromRow`
/// ## 参数
/// - t
/// - db
/// - sql
/// - params*
#[macro_export]
macro_rules! query_as_all {
    ( $t: ty, $db: expr, $sql: expr, $($p: expr),* ) => {
        $crate::query_as!($t, $sql, $($p),*)
            .fetch_all($db)
            .await
    };
}

/// 执行不返回的语句，返回 `Result<xxQueryResult, Error>`
///
/// /// 参数
/// - db
/// - sql
/// - params*
#[macro_export]
macro_rules! execute {
    ( $db: expr, $sql: expr, $($p: expr),* ) => {
        $crate::query!($sql, $($p),*)
            .execute($db)
            .await
    };
}
