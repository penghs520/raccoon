use common::id_generator;
use my_proc_macros::Getter;

#[derive(Debug, Getter)]
pub struct Card<'a> {
    pub id: String,
    code: String,
    name: String,
    state: CardState,
    type_id: &'a str,
    tenant_id: &'a str,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    id: u32,
    value: FieldValue,
}

#[derive(Debug)]
pub enum FieldValue {
    Int(i32),
    Float(f32),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum CardState {
    Active = 1,
    Archived = 2,
    Abandoned = 3,
}

impl<'a> Card<'a> {
    pub fn new(code: String, name: String, type_id: &'a str, tenant_id: &'a str) -> Card<'a> {
        Card {
            id: id_generator::generate_id(),
            code,
            name,
            state: CardState::Active,
            type_id,
            tenant_id,
            fields: Vec::new(),
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
        let mut card = Card::new("10001".to_string(), "卡片01".to_string(), "1", "1");
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
        let mut card = Card::new("10001".to_string(), "卡片01".to_string(), "1", "1");
        card.rename("第一张卡片");
        assert_eq!(card.name, "第一张卡片");
    }
}