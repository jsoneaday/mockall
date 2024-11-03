use crate::models::{EntityId, NewProfile, Profile};
use async_trait::async_trait;
use dotenv::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, Error, PgPool};
use mockall::automock;

#[automock]
#[async_trait]
pub trait Repository {
    async fn get_profile(&self, pool: &PgPool, id: i64) -> Result<Option<Profile>, Error>;
    async fn create_profile(&self, pool: &PgPool, new_profile: NewProfile) -> Result<EntityId, Error>;
}

struct DbRepo;

#[async_trait]
impl Repository for DbRepo {
    async fn get_profile(&self, pool: &PgPool, id: i64) -> Result<Option<Profile>, Error> {
        sqlx::query_as::<_, Profile>(r"
            select *
            from profile
            where id = $1            
        ")
        .bind(id)
        .fetch_optional(pool)
        .await
    }
    async fn create_profile(&self, pool: &PgPool, new_profile: NewProfile) -> Result<EntityId, Error> {
        sqlx::query_as::<_, EntityId>(r"
            insert into profile
            (user_name, full_name, description, region, main_url, avatar)
            values
            ($1, $2, $3, $4, $5, $6)
            returning id
        ")
        .bind(new_profile.user_name)
        .bind(new_profile.full_name)
        .bind(new_profile.description)
        .bind(new_profile.region)
        .bind(new_profile.main_url)
        .bind(new_profile.avatar)
        .fetch_one(pool)
        .await
    }
}

pub struct DbService<T: Repository> {
    pub db: T
}

impl<T: Repository> DbService<T> {
    pub fn new(db: T) -> Self {
        Self {
            db
        }
    }
}

pub async fn get_conn_pool() -> PgPool {
    dotenv().ok();

    let host = env::var("POSTGRES_HOST").unwrap();
    let port = env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap();
    let password = env::var("POSTGRES_PASSWORD").unwrap();
    let user = env::var("POSTGRES_USER").unwrap();
    let db = env::var("POSTGRES_DB").unwrap();
    let pg_url = format!("postgres://{user}:{password}@{host}:{port}/{db}");
    println!("url {pg_url}");
    
    PgPoolOptions::new().max_connections(4).connect(&pg_url).await.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::lorem::en::Sentence;
    use fake::faker::internet::en::Username;
    use fake::faker::name::en::{FirstName, LastName};
    use fake::faker::address::en::CountryName;    
    use fake::Fake;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_repo_calls() {
        let user_name = Username().fake::<String>();
        let full_name = format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>());
        let description = Sentence(1..2).fake::<String>();
        let region = CountryName().fake::<String>();
        let main_url = "https://test.com".to_string();
        let avatar: Vec<u8> = vec![];
        let pool = get_conn_pool().await;

        let mut mock = MockRepository::new();

        mock.expect_get_profile()
            .withf(|_pool: &PgPool, id: &i64| *id == 1)      
            .times(1)
            .returning(move |_pool, _id| Ok(Some(Profile { 
                id: 1, 
                user_name: user_name.clone(), 
                full_name: full_name.clone(), 
                description: description.clone(), 
                region: region.clone(), 
                main_url: main_url.clone(), 
                avatar: avatar.clone() 
            })));

        let service = DbService::new(mock);

        let profile = service.db.get_profile(&pool, 1).await.unwrap();
        assert_eq!(profile.unwrap().id, 1);
        _ = service.db.get_profile(&pool, 1).await.unwrap();
    }
}