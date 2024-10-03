use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Pointer};
use common::id_generator;
use my_proc_macros::Getter;
use crate::newtypes::{Timestamp, LinkDescriptor};

#[derive(Debug, Getter)]
pub struct Card<'a> {
    id: String,
    code: String,
    name: String,
    state: CardState,
    flow_status: Option<FlowStatus>, //不是所有类型的卡都有流动状态，仅工作项类型卡具有
    type_id: &'a str,
    tenant_id: &'a str,
    create_time: Timestamp,
    update_time: Timestamp,
    fields: Vec<Field>,
    links: HashMap<LinkDescriptor, HashSet<Card<'a>>>,
}

#[derive(Debug, Getter)]
pub struct Field {
    id: String,
    value: FieldValue,
}

impl Field {
    pub fn new(id: &str, value: FieldValue) -> Self {
        Self { id: String::from(id), value }
    }
}

#[derive(Debug)]
pub enum FieldValue {
    Int(i32),
    Float(f32),
    Text(String),
    Enum(Vec<String>),
    Date(Timestamp),
    DateTime(Timestamp),
}

#[derive(Debug, PartialEq)]
pub enum CardState {
    Active = 1,
    Archived = 2,
    Abandoned = 3,
}

impl Display for CardState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CardState::Active => { write!(f, "Active") }
            CardState::Archived => { write!(f, "Archived") }
            CardState::Abandoned => { write!(f, "Abandoned") }
        }
    }
}

#[derive(Debug, Getter)]
pub struct FlowStatus {
    flow_id: String,
    flow_status_id: String,
}

impl FlowStatus {
    pub fn new(flow_id: &str, status_id: &str) -> Self {
        Self {
            flow_id: String::from(flow_id),
            flow_status_id: String::from(status_id),
        }
    }
}

impl<'a> Card<'a> {
    pub fn new(code: String, name: String, type_id: &'a str, tenant_id: &'a str, flow_status: Option<FlowStatus>, fields: Vec<Field>, links: HashMap<LinkDescriptor, HashSet<Card<'a>>>) -> Card<'a> {
        let now = Timestamp::now();
        Card {
            id: id_generator::generate_id(),
            code,
            name,
            state: CardState::Active,
            flow_status,
            type_id,
            tenant_id,
            create_time: now,
            update_time: now,
            fields,
            links,
        }
    }

    pub fn rename(&mut self, new_name: &'a str) {
        assert!(!new_name.trim().is_empty(), "card's name is empty");
        self.name = String::from(new_name);
    }

    pub fn active(&mut self) {
        self.state = CardState::Active;
    }

    pub fn abandoned(&mut self) {
        self.state = CardState::Abandoned;
    }

    pub fn archived(&mut self) {
        self.state = CardState::Archived;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_card() {
        let mut card = Card::new("10001".to_string(), "卡片01".to_string(), "1", "1", None, vec![], HashMap::new());
        card.code = String::from("10001");
        println!("{:?}", card);
        assert_eq!(card.code, "10001");
        assert_eq!(card.type_id, "1");
        assert_eq!(card.tenant_id, "1");
        assert_eq!(card.fields.len(), 0);
        assert_eq!(card.name, "卡片01");
        assert_eq!(card.state, CardState::Active);
    }

    #[test]
    fn test_rename() {
        let mut card = Card::new("10001".to_string(), "卡片01".to_string(), "1", "1", None, vec![], HashMap::new());
        card.rename("第一张卡片");
        assert_eq!(card.name, "第一张卡片");
        println!("{:?}", card.state);
    }
}