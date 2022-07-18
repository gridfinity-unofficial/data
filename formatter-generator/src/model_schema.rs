use rand::prelude::*;
use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    id: u64,
    name: String,
    creator: String,
    license: String,
    #[serde(with = "url_serde")]
    url: Url,
    #[serde(with = "url_serde")]
    image_url: Url,
    description: String
}

impl Model {
    pub fn from_json(json_string: String) -> Model {
        let model: Model = serde_json::from_str(json_string.as_str()).unwrap();
        return model;
    }
    pub fn to_json(self) -> String {
        return serde_json::to_string_pretty(&self).unwrap();
    }
    pub fn new(name: String, creator: String, license: String, url: String, image_url: String, description: String) -> Option<Model> {
        match Url::parse(url.as_str()) {
            Ok(url) => {
                match Url::parse(image_url.as_str()) {

                    Ok(image_url) => {
                        let mut rng = rand::thread_rng();
                        return Some(Model {
                            id: rng.gen(),
                            name: name,
                            creator: creator,
                            license: license,
                            url: url,
                            image_url: image_url,
                            description: description
                        });
                    },
                    Err(_) => {
                        eprintln!("failed to parse image url: {}\n\tfor: {}", image_url, url);
                        return None;
                    }
                }
            },
            Err(_) => {
                eprintln!("failed to parse url: {}", url);
                return None;
            }
        }
    }
    pub fn id(&self) -> u64 {
        return self.id.clone();
    }
    pub fn replace_id(mut self) -> Model {
        let mut rng = rand::thread_rng();
        self.id = rng.gen();
        return self;
    }
    pub fn check_url(&self, test: &String) -> bool {
        return self.url.to_string().contains(test.as_str());
    }
}
