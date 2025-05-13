use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Status{
    pub status: String,
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Diary{
    pub id : Option<i64>,
    pub name : String,
    pub address : String
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Entry{
    pub id : Option<i64>,
    pub diary_id : i64,
    pub entry : String,
    pub time : String,
}