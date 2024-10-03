use crate::card::Card;
use crate::graph::get_graph;
use neo4rs::{Node};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::time::{Duration};
use std::{error, fmt};
use serde::{Deserialize, Serialize};
use crate::types::{LinkDescriptor, Path};

//查询条件
#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    items: Vec<ConditionItem>, //且条件
    logic_condition_bulks: Vec<LogicConditionBulk>, //多个或条件集，集之间是And关系
}

//单个条件项
#[derive(Debug, Serialize, Deserialize)]
pub enum ConditionItem {
    CardType(CardTypeOperator), //卡片类型条件项
    State, //卡片活跃状态条件项
    Status, //卡片价值流状态条件项
    Code(String),
    Title(String),
    Text(TextOperator), //文本属性条件项
    Number(NumberOperator), //数字属性条件项
    Enum(EnumOperator), //枚举属性条件项
    Date(DateOperator), //日期属性条件项
    Link(LinkOperator), //关联属性条件项
}

//卡片类型条件项的操作符，仅支持AnyIn
#[derive(Debug, Serialize, Deserialize)]
pub enum CardTypeOperator {
    AnyIn(Vec<String>)
}

//文本属性条件项的操作符
#[derive(Debug, Serialize, Deserialize)]
pub enum TextOperator {
    StartsWith(String),
    Contains(String),
    NotContains(String),
    Equals(PropertyValue<String>),
    NotEquals(PropertyValue<String>),
    IsNull(bool),
}

//普通属性类型条件项的值，可能是一个引用值，或者是一个直接的静态值
#[derive(Debug, Serialize, Deserialize)]
pub enum PropertyValue<T> {
    ReferValue(ReferPoint, Path, String), //引用值
    StaticValue(T), //某个具体的值
}

//引用参考点
#[derive(Debug, Serialize, Deserialize)]
pub enum ReferPoint {
    CurrentMember, //引用自当前成员
    CurrentCard, //引用自当前卡
    Parameter, //引用自一个参数卡
}


//数字属性条件项的操作符
#[derive(Debug, Serialize, Deserialize)]
pub enum NumberOperator {
    LessThan(PropertyValue<i64>),
    GreaterThan(PropertyValue<i64>),
    LessThanOrEqualTo(PropertyValue<i64>),
    GreaterThanOrEqualTo(PropertyValue<i64>),
    Between(PropertyValue<i64>, PropertyValue<i64>),
    NotBetween(PropertyValue<i64>, PropertyValue<i64>),
    Equals(PropertyValue<i64>),
    NotEquals(PropertyValue<i64>),
    IsNull(bool),
}

//枚举属性条件项的操作符
#[derive(Debug, Serialize, Deserialize)]
pub enum EnumOperator {
    AnyIn(PropertyValue<Vec<String>>),
    AllIn(PropertyValue<Vec<String>>),
    AnyNotIn(PropertyValue<Vec<String>>),
    AllNotIn(PropertyValue<Vec<String>>),
    IsNull(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DateOperator {
    //日期支持精度，精度由日期属性定义决定
    After(PropertyValue<u64>),
    Before(PropertyValue<u64>),
    Equals(PropertyValue<u64>),
    NotEquals(PropertyValue<u64>),
    Between(PropertyValue<u64>, PropertyValue<u64>),
    NotBetween(PropertyValue<u64>, PropertyValue<u64>),
    IsNull(bool),
}


//关联属性条件项的操作符
#[derive(Debug, Serialize, Deserialize)]
pub enum LinkOperator {
    AnyIn(LinkValue),
    AllIn(LinkValue),
    AnyNotIn(LinkValue),
    AllNotIn(LinkValue),
    IsNull(bool),
}

//关联属性条件项的值
#[derive(Debug, Serialize, Deserialize)]
pub enum LinkValue {
    ReferValue(ReferPoint, Vec<LinkDescriptor>),
    StaticValue(Vec<String>),
}

//或条件集，由多个或条件组构成，组之间是And的关系
#[derive(Debug, Serialize, Deserialize)]
pub struct LogicConditionBulk {
    groups: Vec<LogicConditionGroup>, // And
}

//或条件组，有多个之间为Or关系的条件项组成
#[derive(Debug, Serialize, Deserialize)]
pub struct LogicConditionGroup {
    items: Vec<ConditionItem>, //Or
}


//查询结果
#[derive(Debug)]
pub struct QueryResult<'a> {
    cards: Vec<Card<'a>>,
    total: u32,
}


//查询时指定的分页参数
#[derive(Debug, Serialize, Deserialize)]
pub enum Page {
    Limit(u32/*num*/, u8/*size*/),
    LimitAfterSort(Sort, u32, u8),
    None,
}

//分页查询时是否开启排序
#[derive(Debug, Serialize, Deserialize)]
pub enum Sort {}

//查询时希望返回卡片上的哪些属性
#[derive(Debug)]
pub struct Yields {

}

//查询发生时的上下文
#[derive(Debug)]
pub struct QueryContext {
    tenant_id: String,
    member_id: String,
    parameters: HashMap<String, String>,
}

impl Condition {
    pub fn new(items: Vec<ConditionItem>, logic_condition_bulks: Vec<LogicConditionBulk>) -> Self {
        Self { items, logic_condition_bulks }
    }

    pub fn and(&mut self, item: ConditionItem) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn and_logic(&mut self, logic_condition_bulk: LogicConditionBulk) -> &mut Self {
        self.logic_condition_bulks.push(logic_condition_bulk);
        self
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self { items: Vec::new(), logic_condition_bulks: Vec::new() }
    }
}

impl LogicConditionBulk {
    pub fn new(groups: Vec<LogicConditionGroup>) -> Self {
        Self { groups }
    }
}

impl Default for LogicConditionBulk {
    fn default() -> Self {
        Self { groups: Vec::new() }
    }
}

impl LogicConditionGroup {
    pub fn new(items: Vec<ConditionItem>) -> Self {
        Self { items }
    }

    pub fn or(&mut self, item: ConditionItem) -> &mut Self {
        self.items.push(item);
        self
    }
}

impl Default for LogicConditionGroup {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}


type Result<T> = std::result::Result<T, Box<dyn error::Error>>; //因为Error是一个动态类型，大小无法在编译期确定，所以需要用Box分配到堆上
pub async fn query<'a>(condition: Condition, query_context: QueryContext, yields: Yields, page: Page) -> Result<QueryResult<'a>> {
    let mut graph = get_graph(); //可以直接传递异步函数，无需手动固定 Future，这是因为 tokio::sync::OnceCell 的 get_or_init 方法本身支持异步初始化
    let mut result = graph.await.execute(
        neo4rs::query("MATCH (p:Person {name: $name}) RETURN p").param("name", "Tom")
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
    async fn test_query() {
        query(
            Condition {
                items: vec![],
                logic_condition_bulks: vec![],
            },
            QueryContext {
                tenant_id: String::from("1"),
                member_id: String::from("!"),
                parameters: HashMap::new(),
            },
            Yields {},
            Page::None,
        ).await.unwrap();
    }

    #[test]
    async fn test_condition_serde() {
        {
            let mut condition = Condition::default();
            condition.and(ConditionItem::CardType(CardTypeOperator::AnyIn(vec!["123".to_string()])))
                .and(ConditionItem::Text(TextOperator::StartsWith("hello".to_string())))
                .and_logic(LogicConditionBulk::new(
                    vec![
                        LogicConditionGroup::new(
                            vec![
                                ConditionItem::Number(NumberOperator::GreaterThan(PropertyValue::StaticValue(12)))
                            ]
                        )
                    ]
                ));
            println!("{:?}", condition);
            let serialized = serde_json::to_string(&condition).unwrap();
            println!("{}", serialized);
            let condition: Condition = serde_json::from_str(&serialized).unwrap();
            println!("通过反序列化：{:#?}", condition);
        }
    }
}