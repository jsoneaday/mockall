// use crate::{auto::Repository, models::{EntityId, NewProfile, Profile}};
// use sqlx::{Error, PgPool};
// use async_trait::async_trait;
// use mockall::mock;



// #[cfg(test)]
// mod tests {
//     use crate::auto::get_conn_pool;
//     use crate::auto::DbService;
//     use super::*;
//     use fake::faker::lorem::en::Sentence;
//     use fake::faker::internet::en::Username;
//     use fake::faker::name::en::{FirstName, LastName};
//     use fake::faker::address::en::CountryName;    
//     use fake::Fake;

//     #[tokio::test]
//     async fn test_create_portfolio_creates_new_portfolio() {
//         let user_name: String = Username().fake();
//         let new_user_name = user_name.clone();
//         let full_name: String = format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>());
//         let new_full_name = full_name.clone();
//         let description: String = Sentence(1..2).fake();
//         let new_description = description.clone();
//         let region: String = CountryName().fake();
//         let new_region = region.clone();
//         let main_url = "http://test.com".to_string();
//         let new_main_url = main_url.clone();
//         let avatar: Vec<u8> = vec![];
//         let new_avatar = avatar.clone();

//         let mut repo_mock = MockMyRepo::new();
//         let pool = get_conn_pool().await;
//         repo_mock.expect_get_profile()
//             .withf(|_pool: &PgPool, id: &i64| *id == 1)
//             .times(1)
//             .returning(move |_pool, id| Ok(Some(Profile {
//             id,
//             user_name: user_name.clone(),
//             full_name: full_name.clone(),
//             description: description.clone(),
//             region: region.clone(),
//             main_url: main_url.clone(),
//             avatar: avatar.clone()
//         })));

//         let service = DbService::new(repo_mock);
//         let profile = service.db.get_profile(&pool, 1).await;
//         assert_eq!(profile.unwrap().unwrap().user_name, new_user_name);
//     }
// }