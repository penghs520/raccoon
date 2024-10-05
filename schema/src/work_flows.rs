//工作流
use serde::{Deserialize, Serialize};
use crate::schema::Schema;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkFlow {
    id: String,
    name: String,
    card_type_id: String, //工作项卡片类型id
    org_id: String,
    description: Option<String>,
}

impl Schema for WorkFlow {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn org_id(&self) -> &str {
        &self.org_id
    }

    fn secondary_indexes(&self) -> Option<Vec<String>> {
        Some(vec![self.card_type_id.to_string()])
    }

    fn description(&self) -> &Option<String> {
        &self.description
    }
}