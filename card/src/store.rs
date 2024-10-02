use crate::card::Card;
use std::future::Future;

pub mod neo4j_store {
    use neo4rs::{Query, RowStream, Txn};
    use crate::card::{Card, FieldValue};
    use crate::graph::get_graph;

    pub struct Neo4jStore;

    impl Neo4jStore {
        pub async fn create<'a>(card: &'a Card<'a>, member_id: &'a str) -> bool {
            let mut graph = get_graph().await;
            let mut txn = graph.start_txn().await.unwrap();
            let create_card_query = Self::build_create_query(card);
            let create_rs_with_member_query = Self::build_create_rs_with_member_query(card.id(), member_id);
            let result = txn.execute(create_card_query).await;
            if result.is_ok() {
                //卡片和卡片创建人的关联
                let mut result = txn.execute(create_rs_with_member_query).await.unwrap();
                if !create_rs_with_member_success(&mut result, &mut txn).await {
                    eprintln!("create card failed, because create relationship with member failed!");
                    return false;
                } else {
                    //todo 创建和其他卡片的关联，不用校验是否关联成功，因为关联的卡片可能不存在，只要语法不报错就可以
                    txn.commit().await.unwrap();
                    //todo 发送事件，异步记录操作历史
                    return true;
                }
            }
            eprintln!("create card failed: {:?}", result.err().unwrap());
            false
        }

        fn build_create_query(card: &Card) -> Query {
            let flow_status_str;
            if let Some(_) = card.flow_status() {
                flow_status_str = ",flow_id:$flow_id,flow_status_id:$flow_status_id"
            } else {
                flow_status_str = ""
            }
            let mut props_str = String::new();
            for field in card.fields() {
                props_str.push_str(&format!(",`{}`:$`{}`", field.id(), field.id()))
            }
            let query = format!("CREATE (n:Card {{ id:$id, state:$state, type_id:$type_id, tenant_id:$tenant_id, create_time:$create_time, update_time:$update_time {flow_status_str} {props_str}}})");
            let mut create_query = neo4rs::query(&query)
                .param("id", String::from(card.id()))
                .param("create_time", card.create_time().to_local_datetime())
                .param("update_time", card.update_time().to_local_datetime())
                .param("type_id", card.type_id())
                .param("tenant_id", card.tenant_id())
                .param("state", card.state().to_string());
            if let Some(flow_status) = card.flow_status() {
                create_query = create_query.param("flow_id", String::from(flow_status.flow_id()))
                    .param("flow_status_id", String::from(flow_status.flow_status_id()));
            }
            for field in card.fields() {
                let key = field.id().as_str();
                create_query = match field.value() {
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
                        create_query.param(key, v.to_local_date())
                    }
                    FieldValue::DateTime(v) => {
                        create_query.param(key, v.to_local_datetime())
                    }
                }
            }
            create_query
        }


        fn build_create_rs_with_member_query(card_id: &str, member_id: &str) -> Query {
            //如果创建成功则会返回1
            neo4rs::query("MATCH (n:Card {id:$card_id}) MATCH (m:Card {id:$member_id}) \
                              CREATE (n)-[:creator]->(m) RETURN 1
                            ")
                .param("card_id", String::from(card_id))
                .param("member_id", String::from(member_id))
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
    use crate::newtypes::{Date, DateTime};
    use super::*;

    #[tokio::test]
    async fn test_create_card() {
        let fields = vec![
            Field::new("text-field", FieldValue::Text(String::from("hello world"))),
            Field::new("int-field", FieldValue::Int(111)),
            Field::new("float-field", FieldValue::Float(111.0)),
            Field::new("enum-field", FieldValue::Enum(vec!["1".to_string(), "2".to_string()])),
            Field::new("date-field", FieldValue::Date(Date::now())),
            Field::new("datetime-field", FieldValue::DateTime(DateTime::now()))
        ];
        let links = HashMap::new();
        let card: Card = Card::new("101".to_string(), "卡片101".to_string(), "t101", "o101", Some(FlowStatus::new("flow-1", "status-1")), fields, links);
        assert!(neo4j_store::Neo4jStore::create(&card, "m101").await);
    }
}


