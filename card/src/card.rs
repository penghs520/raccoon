use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use crate::newtypes::card_id::CardId;
use crate::newtypes::field_id::FieldId;
use crate::newtypes::card_type_id::CardTypeId;
use crate::newtypes::timestamp::Timestamp;
use crate::types::LinkDescriptor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card<'a> {
    pub id: CardId,
    pub code: String,
    pub name: String,
    pub state: CardState,
    pub flow_status: Option<FlowStatus>, //不是所有类型的卡都有流动状态，仅工作项类型卡具有
    pub type_id: &'a str,
    pub tenant_id: &'a str,
    pub create_time: Timestamp,
    pub update_time: Timestamp,
    pub fields: Vec<Field>,
    pub links: HashMap<LinkDescriptor, HashSet<Card<'a>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: FieldId,
    pub value: FieldValue,
}

impl Field {
    pub fn new(id: FieldId, value: FieldValue) -> Self {
        Self { id, value }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldValue {
    Int(i32),
    Float(f32),
    Text(String),
    Enum(Vec<String>),
    Date(Timestamp),
    DateTime(Timestamp),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowStatus {
    pub flow_id: String,
    pub flow_status_id: String,
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
            id: CardId::new(),
            code,
            name,
            state: CardState::Active,
            flow_status,
            type_id,
            tenant_id,
            create_time: now.clone(),
            update_time: now.clone(),
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

// 手动实现 PartialEq 和 Eq
impl<'a> PartialEq for Card<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

//Eq 并不需要任何附加的方法实现，而是仅仅表示 PartialEq 的实现满足等价关系的所有属性。实现它只是为了表示您的类型符合更强的等同性约束。
impl<'a> Eq for Card<'a> {}

// 手动实现 Hash
impl<'a> Hash for Card<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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
    fn test_card_serde() {
        let card_type_id = CardTypeId::from_str("1");
        let card = Card::new("10001".to_string(), "卡片01".to_string(), &card_type_id, "1", None, vec![], HashMap::new());
        let json = serde_json::to_string(&card).unwrap();
        println!("serialize = {}", json);
        let card = serde_json::from_str::<Card>(&json).unwrap();
        println!("deserialize = {:?}", card);
    }

    #[test]
    fn test_rename() {
        let card_type_id = CardTypeId::from_str("1");
        let mut card = Card::new("10001".to_string(), "卡片01".to_string(), &card_type_id, "1", None, vec![], HashMap::new());
        card.rename("第一张卡片");
        assert_eq!(card.name, "第一张卡片");
        println!("{:?}", card.state);
    }
}