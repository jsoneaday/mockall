use crate::models::{EntityId, NewProfile, Profile};
use async_trait::async_trait;
use dotenv::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, Error, PgPool};
use mockall::automock;

#[async_trait]
pub trait Repository {
    async fn get_profile(&self, id: i64) -> Result<Option<Profile>, Error>;
    async fn create_profile(&self, new_profile: NewProfile) -> Result<EntityId, Error>;
}

struct DbRepo {
    pool: PgPool
}

#[async_trait]
impl Repository for DbRepo {
    async fn get_profile(&self, id: i64) -> Result<Option<Profile>, Error> {
        sqlx::query_as::<_, Profile>(r"
            select *
            from profile
            where id = $1            
        ")
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }
    async fn create_profile(&self, new_profile: NewProfile) -> Result<EntityId, Error> {
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
        .fetch_one(&self.pool)
        .await
    }
}

struct DbService<T: Repository> {
    db: T
}

impl<T: Repository> DbService<T> {
    fn new(db: T) -> Self {
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
        let pool = get_conn_pool().await;
        let service = DbService::new(DbRepo { pool });

        let profile_id = service.db.create_profile(NewProfile {
            user_name: user_name.clone(),
            full_name: format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>()),
            description: Sentence(1..2).fake(),
            region: CountryName().fake(),
            main_url: "https://test.com".to_string(),
            avatar: vec![]
        }).await.unwrap();

        let profile = service.db.get_profile(profile_id.id).await.unwrap();
        assert_eq!(profile.unwrap().user_name, user_name);
    }
}