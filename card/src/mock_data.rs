#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::graph::get_graph;

    #[tokio::test]
    pub async fn test_mock_data() {
        let graph = get_graph().await;

        //创建故事卡、任务卡、小队卡，以及它们之间的关联，用于测试

        let mut queries = vec![];
        for i in 0..1000 {
            let query = "CREATE (n:`user_story` { id:$id, name:$name, `start_date`:$start_date})";
            let query = neo4rs::query(query)
                .param("id", i.to_string())
                .param("name", format!("故事-{}", i))
                .param("start_date", NaiveDate::from_ymd_opt(2016, 7, 8).unwrap());
            queries.push(query);
        }
        let mut txn = graph.start_txn().await.unwrap();
        txn.run_queries(queries).await.unwrap();
        txn.commit().await.unwrap();
    }
}