use serenity::prelude::TypeMapKey;

use crate::tmdb::TmDb;

pub struct MovieDb(TmDb);

impl TypeMapKey for MovieDb {
    type Value = MovieDb;
}

impl MovieDb {

    pub fn load(api_key: String) -> Self {
        Self(TmDb::new(api_key))
    }

}

impl AsRef<TmDb> for MovieDb {
    fn as_ref(&self) -> &TmDb {
        &self.0
    }
}
