use std::sync::atomic::AtomicI32;
use std::sync::LazyLock;

pub(crate) static CODE_COUNTER: LazyLock<AtomicI32> = LazyLock::new(|| AtomicI32::new(0));


#[cfg(test)]
mod tests {
    use crate::graph::get_graph;
    use crate::mock_neo4j_data::CODE_COUNTER;
    use crate::newtypes::timestamp::Timestamp;
    use chrono::NaiveDateTime;
    use neo4rs::{BoltInteger, BoltString, BoltType, Graph, Node, Query, Txn};
    use rand::Rng;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Semaphore;
    use tokio::task;
    /*
                 * 数据规模（百万级）
                 * 成员:系统功能 = 1:100
                 * 需求:系统功能 = 1:20
                 * 版本:需求 = 1:10
                 * 小队:成员 = 1:10
                 * 部落:小队 = 1:10
                 * 假设系统功能卡的规模为1_000_000
                 * 则需求卡 = 1000_000 / 20 = 50_000
                 * 版本卡 = 50_000 / 10 = 5_000
                 * 成员卡 = 1_000_000 / 100 = 10_000
                 * 小队卡 = 10_000 / 10 = 1_000
                 * 部落卡 = 1_000 / 10 = 100
                 * 总计 = 1_000_000 + 50_000 + 5_000 + 10_000 + 1_000 + 100 = 1_066_100
                 */

    const SYS_TASK_SCALE: u32 = 1_000_000; //系统功能卡的规模
    const DEMAND_SCALE: u32 = SYS_TASK_SCALE / 20; //需求卡的规模
    const VERSION_SCALE: u32 = DEMAND_SCALE / 10; //版本卡的规模
    const MEMBER_SCALE: u32 = SYS_TASK_SCALE / 100; //成员卡的规模
    const TEAM_SCALE: u32 = MEMBER_SCALE / 10; //小队卡的规模
    const TRIBE_SCALE: u32 = TEAM_SCALE / 10; //部落卡的规模
    const ORG_ID: &str = "测试组织";

    #[tokio::test]
    pub async fn test_mock_data() {
        let graph = get_graph().await;
        let mut txn = graph.start_txn().await.unwrap();
        create_constraint(&mut txn).await;
        create_index(&mut txn).await;
        txn.commit().await.unwrap();

        //因为一次事务里不能创建太多数据，这会导致内存超出，所以创建卡片和关联使用graph的api
        create_cards(graph.clone()).await;
        create_relationships();
    }

    async fn create_cards(graph: Graph) {
        //创建系统任务卡
        do_create_cards(graph.clone(), "系统任务".to_string(), SYS_TASK_SCALE).await;
        //创建需求卡
        do_create_cards(graph.clone(), "业务需求".to_string(), DEMAND_SCALE).await;
        //创建版本卡
        do_create_cards(graph.clone(), "版本".to_string(), VERSION_SCALE).await;
        //创建成员卡
        do_create_cards(graph.clone(), "成员".to_string(), MEMBER_SCALE).await;
        //创建小队卡
        do_create_cards(graph.clone(), "小队".to_string(), TEAM_SCALE).await;
        //创建部落卡
        do_create_cards(graph.clone(), "部落".to_string(), TRIBE_SCALE).await;
    }

    /*
        通过UNWIND批量建卡，语法如下：
        UNWIND $maps AS map
        MERGE (n:Card)
        SET n += map
     */
    async fn do_create_cards(graph: Graph, card_type: String, size: u32) {
        let mut maps: Vec<HashMap<String, BoltType>> = vec![];
        let now = *Timestamp::now();
        for i in 0..size {
            let code = CODE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            let mut map: HashMap<String, BoltType> = HashMap::new();
            let state = if i % 3 == 0 {
                "Active"
            } else if i % 3 == 1 {
                "Archived"
            } else {
                "Abandoned"
            };
            let status = if i % 3 == 0 {
                "未开始"
            } else if i % 3 == 1 {
                "进行中"
            } else {
                "已完成"
            };
            map.insert("id".to_string(), BoltType::String(BoltString::new(&format!("{}-{}", &card_type, i))));
            map.insert("code".to_string(), BoltType::String(BoltString::new(code.to_string().as_str())));
            map.insert("name".to_string(), BoltType::String(BoltString::new(&format!("{}-{}", &card_type, i))));
            map.insert("org_id".to_string(), BoltType::String(BoltString::new(ORG_ID)));
            map.insert("card_type_id".to_string(), BoltType::String(BoltString::new(&card_type)));
            map.insert("state".to_string(), BoltType::String(BoltString::new(&state)));
            map.insert("flow_id".to_string(), BoltType::String(BoltString::new(&format!("{}价值流", &card_type))));
            map.insert("flow_status_id".to_string(), BoltType::String(BoltString::new(&status)));
            map.insert("create_time".to_string(), BoltType::Integer(BoltInteger::new(now)));
            map.insert("update_time".to_string(), BoltType::Integer(BoltInteger::new(now)));
            map.insert("计划开始时间".to_string(), BoltType::Integer(BoltInteger::new(generate_random_timestamp())));
            map.insert("计划完成时间".to_string(), BoltType::Integer(BoltInteger::new(generate_random_timestamp())));
            maps.push(map);
            //*CODE_COUNTER += 1;
            if i % 1000 == 0 {
                //用merge的话注意要提前给唯一键，不能在 +=中给，否则会多次创建
                let query = neo4rs::query("UNWIND $maps AS map CREATE (n:Card {id: map.id}) SET n += map")
                    .param("maps", maps);
                graph.run(query).await.expect("创建系统任务卡失败");
                maps = Vec::new();
            }
        }
        if !maps.is_empty() {
            let query = neo4rs::query("UNWIND $maps AS map CREATE (n:Card {id: map.id}) SET n += map")
                .param("maps", maps);
            graph.run(query).await.expect("创建系统任务卡失败");
        }
    }

    fn create_relationships() {}

    async fn create_index(txn: &mut Txn) {
        let mut queries = vec![];
        //card_id是唯一的，会默认创建一个RANGE INDEX，所以不用重复创建

        //我们为card_type_id、org_id、state创建RANGE INDEX
        queries.push("CREATE INDEX card_type_id_range_node_index IF NOT EXISTS FOR (n:Card) ON (n.card_type_id)");
        queries.push("CREATE INDEX org_id_range_node_index IF NOT EXISTS FOR (n:Card) ON (n.org_id)");
        queries.push("CREATE INDEX state_range_node_index IF NOT EXISTS FOR (n:Card) ON (n.state)");

        //为flow_id 何 flow_status_id创建联合索引
        queries.push("CREATE INDEX flow_composite_range_node_index IF NOT EXISTS FOR (n:Card) ON (n.flow_id, n.flow_status_id)");

        //为code、name创建TEXT INDEX，因为TEXT INDEX对CONTAINS、ENDS WITH操作做过优化
        queries.push("CREATE TEXT INDEX code_text_node_index IF NOT EXISTS FOR (n:Card) ON (n.code)");
        queries.push("CREATE TEXT INDEX name_text_node_index IF NOT EXISTS FOR (n:Card) ON (n.name)");

        let queries: Vec<Query> = queries.iter().map(|&q| { neo4rs::query(q) }).collect();
        txn.run_queries(queries).await.expect("创建索引失败");
    }

    /*
        创建语法：
        CREATE CONSTRAINT [constraint_name] [IF NOT EXISTS]
        FOR (n:LabelName)
        REQUIRE n.propertyName IS [NODE] UNIQUE
        [OPTIONS "{" option: value[, ...] "}"]

        删除语法：DROP CONSTRAINT constraint_name [IF EXISTS]
     */
    async fn create_constraint(txn: &mut Txn) {
        let queries = vec![
            "CREATE CONSTRAINT card_id_is_unique IF NOT EXISTS FOR (c:Card) REQUIRE c.id IS UNIQUE;",
            //属性非空约束要求企业版
            //"CREATE CONSTRAINT card_org_id_is_not_null IF NOT EXISTS FOR (c:Card) REQUIRE c.org_id IS NOT NULL;",
            //"CREATE CONSTRAINT card_state_is_not_null IF NOT EXISTS FOR (c:Card) REQUIRE c.state IS NOT NULL;",
            //"CREATE CONSTRAINT card_type_id_is_not_null IF NOT EXISTS FOR (c:Card) REQUIRE c.card_type_id IS NOT NULL;",
        ];
        let queries: Vec<Query> = queries.iter().map(|&q| { neo4rs::query(q) }).collect();
        txn.run_queries(queries).await.expect("创建约束失败");
    }

    fn generate_random_timestamp() -> i64 {
        // 定义 2020-01-01 00:00:00 和 2024-12-31 23:59:59 的时间戳
        let start_timestamp = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc().timestamp();
        let end_timestamp = NaiveDateTime::parse_from_str("2024-12-31 23:59:59", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc().timestamp();
        // 生成范围内的随机时间戳
        rand::thread_rng().gen_range(start_timestamp..=end_timestamp)
    }

    //模拟十个用户的并发查询
    #[tokio::test]
    async fn mock_concurrent_query() {
        // 定义并发数量
        let concurrency = 20;

        // 创建一个 vector 来保存任务句柄
        let mut handles = vec![];

        let mut graph = get_graph().await;

        for i in 0..20 {
            let graph = graph.clone();
            // 启动任务并将句柄存储在向量中
            let handle = task::spawn(async move {
                // 执行查询
                //每个线程查100次
                for j in 0..(i + 1) * 10 {
                    let mut result = graph.execute(
                        neo4rs::query("MATCH (n:Card) where n.code=$code RETURN n").param("code", j.to_string())
                    ).await.unwrap();
                    while let Ok(Some(row)) = result.next().await {
                        let node: Node = row.get("n").unwrap();
                        let name: String = node.get("name").unwrap();
                        println!("{}", name);
                    }
                }
            });

            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            let _ = handle.await;
        }
        println!("All queries completed.");
    }
}