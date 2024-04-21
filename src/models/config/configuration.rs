use serde::{Serialize, Deserialize};
use tokio::fs::read_to_string;
use std::{process, fmt::{self, Display}};

use super::{
    Podcast,
    super::archive::IAClient};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    log_level: String,
    public: String,
    style_css: String,
    timezone: String,
    podcast: Podcast,
    iaclient: IAClient,
}

impl Display for Configuration{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "log_level: {}\npublic: {}",
            self.get_log_level(),
            self.get_public(),
        )
    }
}

impl Configuration {
    pub fn get_podcast(&self) -> &Podcast{
        &self.podcast
    }

    pub fn get_iaclient(&self) -> &IAClient{
        &self.iaclient
    }

    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }

    pub fn get_public(&self) -> &str{
        &self.public
    }

    pub fn get_style_css(&self) -> &str{
        &self.style_css
    }

    pub async fn read_configuration() -> Configuration{
        let content = match read_to_string("config.yml")
            .await {
                Ok(value) => value,
                Err(e) => {
                    println!("Error with config file `config.yml`: {e}");
                    process::exit(0);
                }
            };
        match serde_yaml::from_str(&content){
            Ok(configuration) => configuration,
            Err(e) => {
                println!("Error with config file `config.yml`: {e}");
                process::exit(0);
            }
        }
    }
}
