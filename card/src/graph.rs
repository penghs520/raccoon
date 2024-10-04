use neo4rs::{ConfigBuilder, Database, Graph};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

//Graph对象本身能够在多线程中clone
pub(crate) static GLOBAL_GRAPH: OnceCell<Graph> = OnceCell::const_new();


const GRAPH_TYPE: GraphType = GraphType::Memgraph;

#[derive(Eq, PartialEq)]
pub enum GraphType {
    Neo4j,
    Memgraph,
}

pub(crate) async fn init_graph() -> Graph {
    //Graph::new("127.0.0.1:7687", "", "").await.unwrap();
    if GraphType::Neo4j == GRAPH_TYPE{
        Graph::new("127.0.0.1:7687", "", "").await.unwrap()
    } else {
        let config = ConfigBuilder::default()
            .uri("127.0.0.1:7687")
            .user("")
            .password("")
            .db(Database::from("memgraph"))
            .build();
        if let Ok(config) = config {
            Graph::connect(config).await.unwrap()
        } else {
            panic!("failed to initialize graph")
        }
    }
}

pub(crate) async fn get_graph() -> Graph {
    //可以直接传递异步函数，无需手动固定 Future，这是因为 tokio::sync::OnceCell 的 get_or_init 方法本身支持异步初始化
    GLOBAL_GRAPH.get_or_init(init_graph).await.clone() //是否需要克隆
}
