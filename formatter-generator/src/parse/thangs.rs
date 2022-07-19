use std::time::Duration;

use rustyline::Editor;
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
                    let mut rl = Editor::<()>::new().expect("failed to hook stdin.");
                    let model_data = &serde_json::from_str::<serde_json::Value>(script.text().as_str()).unwrap()["props"]["pageProps"]["initialState"]["model"]["data"];
                    //println!("{:?}", model_data["parts"][0]["thumbnailUrl"]);
                    let model_name: String = model_data["name"].to_string().replace("\\", "").replace("\"", "").trim().into();
                    let model_creator: String =  model_data["owner"]["username"].to_string().replace("\\", "").replace("\"", "");
                    let mut model_category: String = "none".into();
                    let mut model_grid_size: (usize, usize) = (1,1);
                    println!("Please provide a category for:\n\t{}\n\t{}\n\t{}", model_name, model_creator, url);
                    let readline = rl.readline("\tCategory: [none] > ");
                    match readline {
                        Ok(o) => model_category = o.trim().to_lowercase(),
                        Err(_) => ()
                    }
                    println!("Please provide a grid size in 'XxY' format (1x1, 2x1, 3x5, etc.):");
                    let readline = rl.readline("\tGrid Size: [1x1] > ");
                    match readline {
                        Ok(o) => {
                            let line_formatted = o.trim().to_lowercase();
                            let temp_size: Vec<&str> = line_formatted.split('x').collect();
                            model_grid_size = (temp_size[0].parse().expect("failed to parse grid size"), temp_size[1].parse().expect("failed to parse grid size"));
                        }
                        Err(_) => ()
                    }
                    return Model::new(
                        model_name,
                        model_creator,
                        "unimplemented".into(),
                        url.replace("\\", "").replace("\"", ""),
                        model_data["parts"][0]["thumbnailUrl"].to_string().replace("\\", "").replace("\"", ""),
                        model_data["description"].to_string().replace("\\n", "\n").replace("\\", "").replace("\"", "").trim().into(),
                        model_category,
                        model_grid_size.0,
                        model_grid_size.1
                    );
                }
            },
            None => ()
        }
    }

    eprintln!("failed to load thangs page");
    return None;
}
