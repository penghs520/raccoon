pub mod card;
pub mod store;
pub mod query;
mod graph;
mod mock_data;
mod newtypes;
mod events;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::card::{Card, CardState};

    #[test]
    #[test]
    fn test_new_card() {
        let card = Card::new("10001".to_string(), "卡片01".to_string(), "1", "1", None, vec![], HashMap::new());
        println!("{:?}", card);
        assert_eq!(card.code(), "10001");
        assert_eq!(card.type_id(), "1");
        assert_eq!(card.tenant_id(), "1");
        assert_eq!(card.fields().len(), 0);
        assert_eq!(card.name(), "卡片01");
        assert_eq!(card.state(), &CardState::Active);
    }
}
