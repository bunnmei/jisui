use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub folder_name: String,
    pub uuid: String,
    pub list: Vec<String>,
}
