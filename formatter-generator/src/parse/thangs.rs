use std::time::Duration;

use soup::prelude::*;
use ureq::{Agent, AgentBuilder};

use crate::model_schema::Model;

pub fn from_url(url: String) -> Option<Model> {
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let body: String = agent.get(url.as_str())
        .call().unwrap()
        .into_string().unwrap();

    //println!("{}", body);

    let soup = Soup::new(body.as_str());

    for (_, script) in soup.tag("script").find_all().enumerate() {
        match script.get("id") {
            Some(s) => {
                if s.contains("__NEXT_DATA__") {
                    let model_data = &serde_json::from_str::<serde_json::Value>(script.text().as_str()).unwrap()["props"]["pageProps"]["initialState"]["model"]["data"];
                    //println!("{:?}", model_data["parts"][0]["thumbnailUrl"]);
                    return Model::new(
                        model_data["name"].to_string().replace("\\", "").replace("\"", "").trim().into(),
                        model_data["owner"]["username"].to_string().replace("\\", "").replace("\"", ""),
                        "unpopulated".into(),
                        url.replace("\\", "").replace("\"", ""),
                        model_data["parts"][0]["thumbnailUrl"].to_string().replace("\\", "").replace("\"", ""),
                        model_data["description"].to_string().replace("\\n", "\n").replace("\\", "").replace("\"", "").trim().into()
                    );
                }
            },
            None => ()
        }
    }

    eprintln!("failed to load thangs page");
    return None;
}
