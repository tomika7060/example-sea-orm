use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub text: String,
}
