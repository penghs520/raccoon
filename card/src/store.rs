use crate::card::Card;
use std::future::Future;

pub mod neo4j_store {
    use neo4rs::{Query, RowStream, Txn};
    use crate::card::{Card, FieldValue};
    use crate::graph::get_graph;
    use crate::newtypes::card_id::CardId;

    pub struct Neo4jStore;

    impl Neo4jStore {
        pub async fn create<'a>(card: &'a Card<'a>, member_id: &'a CardId) -> bool {
            let mut graph = get_graph().await;
            let mut txn = graph.start_txn().await.unwrap();
            let create_card_query = Self::build_create_query(card);
            let create_rs_with_member_query = Self::build_create_rs_with_member_query(&card.id, member_id);
            let result = txn.run(create_card_query).await; //在memgraph上不能用execute，因为返回了不正确的结果
            if result.is_ok() {
                //卡片和卡片创建人的关联 todo 因为run方法不返回结果，所以不知道是否成功关联
                let mut result = txn.run(create_rs_with_member_query).await;
                if result.is_ok() {
                    return match txn.commit().await {
                        Ok(_) => true,
                        Err(err) => {
                            eprintln!("failed to commit transaction: {:?}", err);
                            false
                        }
                    };
                }
            }
            false
        }

        fn build_create_query(card: &Card) -> Query {
            let flow_status_str;
            if let Some(_) = card.flow_status {
                flow_status_str = ",flow_id:$flow_id,flow_status_id:$flow_status_id"
            } else {
                flow_status_str = ""
            }
            let mut props_str = String::new();
            for field in &card.fields {
                props_str.push_str(&format!(",`{}`:$`{}`", field.id, field.id))
            }
            let query = format!("CREATE (n:Card {{ id:$id, code:$code, name:$name, state:$state, card_type_id:$card_type_id, org_id:$org_id, create_time:$create_time, update_time:$update_time {flow_status_str} {props_str}}})");
            let mut create_query = neo4rs::query(&query)
                //.param("id", *card.id) 不能移动，因为String没有实现Copy
                .param("id", card.id.as_str())
                .param("code", card.code.as_str())
                .param("name", card.name.as_str())
                .param("create_time", *card.create_time)
                .param("update_time", *card.update_time)
                .param("card_type_id", card.card_type_id)
                .param("org_id", card.org_id)
                .param("state", card.state.to_string());
            if let Some(flow_status) = &card.flow_status {
                create_query = create_query.param("flow_id", flow_status.flow_id.as_str())
                    .param("flow_status_id", flow_status.flow_status_id.as_str());
            }
            for field in &card.fields {
                let key = field.id.as_str();
                create_query = match &field.value {
                    FieldValue::Int(v) => {
                        create_query.param(key, *v)
                    }
                    FieldValue::Float(v) => {
                        create_query.param(key, *v)
                    }
                    FieldValue::Text(v) => {
                        create_query.param(key, String::from(v))
                    }
                    FieldValue::Enum(v) => {
                        create_query.param(key, v.clone())
                    }
                    FieldValue::Date(v) => {
                        create_query.param(key, v.to_string())
                    }
                    FieldValue::DateTime(v) => {
                        create_query.param(key, v.to_string())
                    }
                }
            }
            create_query
        }

        fn build_create_rs_with_member_query(card_id: &CardId, member_id: &CardId) -> Query {
            //如果创建成功则会返回1
            neo4rs::query("MATCH (n:Card {id:$card_id}) MATCH (m:Card {id:$member_id}) CREATE (n)-[:creator]->(m)")
                .param("card_id", card_id.as_str())
                .param("member_id", member_id.as_str())
        }

        async fn archive<'a>(card_id: &'a str, member_id: &'a str) {}

        async fn abandon<'a>(card_id: &'a str, reason: &'a str, member_id: &'a str) {}

        async fn restore() {}
    }

    async fn create_rs_with_member_success(row_stream: &mut RowStream, txn: &mut Txn) -> bool {
        if let Ok(opt) = row_stream.next(txn.handle()).await {
            if let Some(row) = opt {
                if let Ok(value) = row.to::<i64>() {
                    //创建卡片和创建人的关系成功后会返回固定值1
                    return value == 1;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::card::{Field, FieldValue, FlowStatus};
    use crate::newtypes::card_id::CardId;
    use crate::newtypes::card_type_id::CardTypeId;
    use crate::newtypes::field_id::FieldId;
    use crate::newtypes::timestamp::Timestamp;
    use super::*;

    #[tokio::test]
    async fn test_create_card() {
        let fields = vec![
            Field::new(FieldId::from_str("text-field"), FieldValue::Text(String::from("hello world"))),
            Field::new(FieldId::from_str("int-field"), FieldValue::Int(111)),
            Field::new(FieldId::from_str("float-field"), FieldValue::Float(111.0)),
            Field::new(FieldId::from_str("enum-field"), FieldValue::Enum(vec!["1".to_string(), "2".to_string()])),
            Field::new(FieldId::from_str("date-field"), FieldValue::Date(Timestamp::now())),
            Field::new(FieldId::from_str("datetime-field"), FieldValue::DateTime(Timestamp::now()))
        ];
        let links = HashMap::new();
        let card_type_id = CardTypeId::from_str("t101");
        let card: Card = Card::new("c106".to_string(), "卡片101".to_string(), &card_type_id, "o101", Some(FlowStatus::new("flow-1", "status-1")), fields, links);
        assert!(neo4j_store::Neo4jStore::create(&card, &CardId::from_str("m103")).await);
    }
}


