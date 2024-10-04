pub mod card;
pub mod store;
pub mod query;
mod graph;
mod mock_neo4j_data;
pub mod newtypes;
mod events;
mod relationship;
mod types;
mod mock_memgraph_data;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::card::{Card, CardState};

    #[test]
    #[test]
    fn test_new_card() {
        let card = Card::new("10001".to_string(), "卡片01".to_string(), "1", "1", None, vec![], HashMap::new());
        println!("{:?}", card);
        println!("{}", card.id);
        assert_eq!(card.code, "10001");
        assert_eq!(card.card_type_id, "1");
        assert_eq!(card.org_id, "1");
        assert_eq!(card.fields.len(), 0);
        assert_eq!(card.name, "卡片01");
        assert_eq!(card.state, CardState::Active);
    }
}
