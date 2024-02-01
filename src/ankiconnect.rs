use std::time::Duration;

use ureq::Agent;

use crate::DeckList;

pub struct AnkiConnect(String, ureq::Agent);

impl AnkiConnect {
    pub fn new(hostname: &str, port: u32) -> Self {
        let agent: Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        Self(format!("http://{hostname}:{port}"), agent)
    }

    pub fn list_decks(&self) -> Vec<DeckList> {
        let mut decklist = Vec::with_capacity(250);
        let response = self.1.post(&self.0).send_json(ureq::json!({
        "action": "deckNamesAndIds",
        "version": 6
            }));
        let r = response.unwrap().into_json::<serde_json::Value>().unwrap();
        let r = r.get("result").unwrap();
        r.as_object().unwrap().iter().for_each(|(a, b)| {
            decklist.push(DeckList {
                id: b.as_i64().unwrap(),
                name: a.to_string(),
                card_count: -1,
                timestamp: None,
            })
        });
        decklist
    }

    pub fn list_models(&self) -> Vec<String> {
        let mut model_list = Vec::with_capacity(250);
        let response = self.1.post(&self.0).send_json(ureq::json!({
        "action": "modelNames",
        "version": 6
            }));
        let r = response.unwrap().into_json::<serde_json::Value>().unwrap();
        let r = r.get("result").unwrap();
        r.as_array().unwrap().iter().for_each(|a| {
            model_list.push(a.as_str().unwrap().to_owned());
        });
        model_list
    }
}
