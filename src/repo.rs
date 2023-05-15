use mongodb::{options::ClientOptions, Client, Collection, bson::{doc, oid::ObjectId, DateTime}};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use serenity::{futures::TryStreamExt, prelude::TypeMapKey};

use crate::{utils::Rating, tmdb::model::Movie};


pub struct MovieRepo(Collection<RatedMovie>);

impl TypeMapKey for MovieRepo {
    type Value = MovieRepo;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatedMovie {
    pub rating: Rating,
    pub movie: Movie,
    pub date: DateTime
}

impl MovieRepo {

    pub async fn load(host: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(host).await?;
        let client = Client::with_options(client_options)?;

        let db = client.default_database().ok_or_else(|| anyhow!("No default database"))?;

        let collection = db.collection::<RatedMovie>("movies");

        Ok(Self(collection))
    } 

    pub async fn create_movie(&self, movie: RatedMovie) -> Result<()> {
        self.0.insert_one(movie, None).await?;
        Ok(())
    }

    pub async fn load_movie(&self, id: &ObjectId) -> Result<Option<RatedMovie>> {
        let movie = self.0.find_one(doc! { "_id": id }, None).await?;
        Ok(movie)
    }

    pub async fn load_all_movies(&self) -> Result<Vec<RatedMovie>> {
        let movies = self.0
            .find(None, None)
            .await?
            .try_collect()
            .await?;

        Ok(movies)
    }

}
