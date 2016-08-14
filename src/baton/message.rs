#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    #[serde(default,rename="type")]
    pub msg_type: String,
    pub sha: Option<String>,
    pub branch: Option<String>,
    pub url: Option<String>,
    pub checksum_url: Option<String>,
}
