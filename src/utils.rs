use std::{str::FromStr, fmt::Display};
use anyhow::{Error, anyhow};

use serde::{Serialize, Deserialize};
use serenity::{model::prelude::{Message}, utils::Colour};

pub const STAR: &str = "⭐";

pub fn find_args(message: &Message) -> Vec<String> {
    let raw_message = message.content.clone();

    return raw_message
        .split_whitespace()
        .map(|s| s.to_string())
        // this will prevent the command name from being included in the args
        .skip(1) 
        .collect::<Vec<String>>();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating(u8);

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / 10", self.0)
    }
}

impl FromStr for Rating {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rating = s.parse::<u8>()
            .map_err(|_| anyhow!("El rating solo puede ser un numero."))?;

        let rating = Self::new(rating)?;

        Ok(rating)
    }
}

impl Rating {
    pub fn new(rating: u8) -> Result<Self, Error> {
        if rating > 10 {
            return Err(anyhow!("El rating debe ser entre 0 y 10"));
        }

        Ok(Self(rating))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn to_stars(&self) -> String {
        (0..self.value())
            .map(|_| STAR)
            .collect::<Vec<&str>>()
            .join("")
    } 

}

pub fn random_color() -> Colour {
    Colour::new(rand::random::<u32>() & 0xFFFFFF)
}
