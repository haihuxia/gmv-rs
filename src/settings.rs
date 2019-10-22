#[derive(Debug, Deserialize)]
pub struct Settings {
    pub from: From,
    pub to: To,
}

#[derive(Debug, Deserialize)]
pub struct From {
    pub git: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct To {
    pub url: String,
    pub personal_token: String,
    pub group: String,
}