use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum AppError {}
