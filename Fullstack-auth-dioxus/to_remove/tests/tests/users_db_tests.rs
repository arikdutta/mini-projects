mod helper;

use std::sync::LazyLock;

use app::domain::auth::_users::data::users_db::UsersDb;
use app::domain::auth::_users::pk::UserPk;
use app::utils::types::unid::Unid;
use helper::tracing::TRACING;
use sqlx::PgPool;

/* ========================================================== */
/*                     ✨ GET ALL ✨                          */
/* ========================================================== */

#[sqlx::test(migrations = "../migrations")]
async fn test_get_all_users_with_seed_data(pool: PgPool) -> sqlx::Result<()> {
    LazyLock::force(&TRACING);

    let users = UsersDb::get_all(&pool).await?;

    // Migration seeds 4 users: Root, Admin, Alice, Bob
    assert_eq!(users.len(), 4);
    assert!(users.iter().any(|u| u.email == "root@example.com"));
    assert!(users.iter().any(|u| u.email == "admin@example.com"));
    assert!(users.iter().any(|u| u.email == "alice@example.com"));
    assert!(users.iter().any(|u| u.email == "bob@example.com"));

    Ok(())
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_all_users_have_names(pool: PgPool) -> sqlx::Result<()> {
    LazyLock::force(&TRACING);

    let users = UsersDb::get_all(&pool).await?;

    // Verify user names
    assert!(users.iter().any(|u| u.first_name == "Root"));
    assert!(users.iter().any(|u| u.first_name == "Admin"));
    assert!(users.iter().any(|u| u.first_name == "Alice" && u.last_name == "Aubert"));
    assert!(users.iter().any(|u| u.first_name == "Bob" && u.last_name == "Baker"));

    Ok(())
}

/* ========================================================== */
/*                   ✨ GET BY UNID ✨                        */
/* ========================================================== */

#[sqlx::test(migrations = "../migrations")]
async fn test_get_by_unid_existing_user(pool: PgPool) -> sqlx::Result<()> {
    LazyLock::force(&TRACING);

    // Root user unid from migration
    let root_unid: Unid<UserPk> =
        Unid::from_string("485d1ad0-792c-436b-a790-17c106135c67").unwrap();

    let user = UsersDb::get_by_unid(&pool, root_unid).await?;

    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.email, "root@example.com");
    assert_eq!(user.first_name, "Root");

    Ok(())
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_by_unid_non_existing_user(pool: PgPool) -> sqlx::Result<()> {
    LazyLock::force(&TRACING);

    let fake_unid: Unid<UserPk> = Unid::new();
    let user = UsersDb::get_by_unid(&pool, fake_unid).await?;

    assert!(user.is_none());

    Ok(())
}

/* ========================================================== */
/*                 ✨ GET FROM EMAIL ✨                       */
/* ========================================================== */

#[sqlx::test(migrations = "../migrations")]
async fn test_get_from_email_existing(pool: PgPool) -> sqlx::Result<()> {
    LazyLock::force(&TRACING);

    let user = UsersDb::get_from_email("alice@example.com".to_string(), &pool).await;

    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.email, "alice@example.com");

    Ok(())
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_from_email_non_existing(pool: PgPool) -> sqlx::Result<()> {
    LazyLock::force(&TRACING);

    let user = UsersDb::get_from_email("nonexistent@example.com".to_string(), &pool).await;

    assert!(user.is_none());

    Ok(())
}

/* ========================================================== */
/*              ✨ INTEGRATION FLOW TESTS ✨                  */
/* ========================================================== */

#[sqlx::test(migrations = "../migrations")]
async fn test_get_all_then_get_by_unid(pool: PgPool) -> sqlx::Result<()> {
    LazyLock::force(&TRACING);

    // Get all users
    let users = UsersDb::get_all(&pool).await?;
    assert!(!users.is_empty());

    // Get the first user by unid
    let first_user = &users[0];
    let unid: Unid<UserPk> = Unid::from_string(&first_user.unid.to_string()).unwrap();

    let fetched_user = UsersDb::get_by_unid(&pool, unid).await?;
    assert!(fetched_user.is_some());
    let fetched_user = fetched_user.unwrap();
    assert_eq!(fetched_user.email, first_user.email);

    Ok(())
}
