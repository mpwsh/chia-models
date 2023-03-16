use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RoutesResponse {
    pub routes: Option<Vec<String>>,
    pub success: String,
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HealthzResponse {
    pub success: String,
    pub error: Option<String>,
}
