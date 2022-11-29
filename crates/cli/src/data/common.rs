#[derive(Debug, Deserialize, Clone)]
pub struct Socials {
    pub facebook: Option<String>,
    pub instragram: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub youtube: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Contacts {
    pub email: Option<String>,
    #[serde(rename = "websiteUrl")]
    pub website_url: Option<String>,
    pub phone: Option<String>,
}
