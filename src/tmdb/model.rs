use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::build_image_path;

fn des_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Option::<Vec<T>>::deserialize(deserializer).map(|v| v.unwrap_or_else(Vec::new))
}

fn des_image_path<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)
        .map(|v| 
            v.map(|path| build_image_path(&path))
        )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagedResponse<T>
where
        T: DeserializeOwned + Serialize
{
    pub page: Option<u32>,
    pub total_results: Option<u32>,
    pub total_pages: Option<u32>,
    #[serde(deserialize_with = "des_empty_vec")]
    pub results: Vec<T>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub overview: String,
    #[serde(deserialize_with = "des_image_path")]
    pub poster_path: Option<String>,
}
