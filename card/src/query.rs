use crate::card::Card;
use crate::graph::get_graph;
use std::collections::HashMap;
use std::{error, fmt};
use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;
use neo4rs::{query, Node};

pub struct Condition {
    items: Vec<ConditionItem>, //And
    logic_conditions: Vec<LogicCondition>, //And
}

pub enum ConditionItem {}

pub struct LogicCondition {
    logic_parts: Vec<LogicConditionPart>, // And
}

pub struct LogicConditionPart {
    items: Vec<ConditionItem>, //Or
}

pub enum LogicConditionItem {}

pub struct QueryResult<'a> {
    cards: Vec<Card<'a>>,
    total: u32,
}


pub enum Page {
    Limit(u32/*num*/, u8/*size*/),
    LimitAfterSort(Sort, u32, u8),
    None,
}

pub struct Sort {}

pub struct Yields {}

pub struct QueryContext {
    tenant_id: String,
    member_id: String,
    params: HashMap<String, String>,
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub async fn query_card<'a>(condition: Condition, query_context: QueryContext, yields: Yields, page: Page) -> Result<QueryResult<'a>> {
    let mut graph = get_graph(); //可以直接传递异步函数，无需手动固定 Future，这是因为 tokio::sync::OnceCell 的 get_or_init 方法本身支持异步初始化
    let mut result = graph.await.execute(
        query("MATCH (p:Person {name: $name}) RETURN p").param("name", "Tom")
    ).await.unwrap();
    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("p").unwrap();
        let name: String = node.get("name").unwrap();
        println!("{}", name);
    }
    //Err(Box::new(QueryError::new("查询出错了")))
    Ok(QueryResult {
        cards: vec![],
        total: 0,
    })
}


#[derive(Debug)]
struct QueryError {
    message: String,
}

impl QueryError {
    fn new(message: &str) -> Self {
        Self { message: message.to_string() }
    }
}

impl Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for QueryError {}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    // 一个模拟的异步方法
    async fn async_add(a: i32, b: i32) -> i32 {
        tokio::time::sleep(Duration::from_millis(50)).await;
        a + b
    }
    #[test]
    async fn test_async_add() {
        let result = async_add(2, 3).await;
        assert_eq!(result, 5);
    }

    #[test]
    async fn test_query_card() {
        query_card(
            Condition {
                items: vec![],
                logic_conditions: vec![],
            },
            QueryContext {
                tenant_id: String::from("1"),
                member_id: String::from("!"),
                params: HashMap::new(),
            },
            Yields {},
            Page::None,
        ).await.unwrap();
    }
}