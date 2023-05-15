pub mod model;

use moka::sync::Cache;
use reqwest::{Client, RequestBuilder, Method};
use anyhow::Result;

use self::model::{PagedResponse, Movie};

pub struct TmDb {
    client: Client,
    api_key: String,
    search_cache: Cache<String, PagedResponse<Movie>>
}

const BASE_URL: &str = "https://api.themoviedb.org/3";
const BASE_IMAGE_URL: &str = "https://image.tmdb.org/t/p/w500";

pub fn build_path(path: &str) -> String {
    if path.starts_with('/') {
        return format!("{}{}", BASE_URL, path);
    } else {
        return format!("{}/{}", BASE_URL, path);
    }
}

pub fn build_image_path(path: &str) -> String {
    if path.starts_with('/') {
        return format!("{}{}", BASE_IMAGE_URL, path);
    } else {
        return format!("{}/{}", BASE_IMAGE_URL, path);
    }
}

impl TmDb {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();

        Self {
            client,
            api_key,
            search_cache: Cache::new(100)
        }
    }

    fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = build_path(path);

        self.client.request(method, url)
            .bearer_auth(&self.api_key)
    }

    pub async fn find_movies(&self, query: &str) -> Result<PagedResponse<Movie>> {
        match self.search_cache.get(query) {
            Some(r) => Ok(r),
            None => {
                let req = self.request(Method::GET, "/search/movie")
                    .query(&[
                        ("query", query),
                        ("include_adult", "true")
                    ]);

                let res = req.send()
                    .await?
                    .json::<PagedResponse<Movie>>()
                    .await?;

                self.search_cache.insert(query.to_string(), res.clone());
                Ok(res)
            }
        }
    }
}
