use std::sync::Arc;
use neo4rs::Graph;
use tokio::sync::OnceCell;

pub(crate) static GLOBAL_GRAPH: OnceCell<Arc<Graph>> = OnceCell::const_new(); //这里用的是tokio的OnceCell，如果您使用 tokio 提供的 OnceCell，可以直接传递异步函数，无需手动固定 Future，这是因为 tokio::sync::OnceCell 的 get_or_init 方法本身支持异步初始化

pub(crate) async fn init_graph() -> Arc<Graph> {
    let graph = Graph::new("127.0.0.1:7687", "neo4j", "Agilean@123").await.unwrap();
    Arc::new(graph)
}

pub(crate) async fn get_graph() -> Arc<Graph> {
    GLOBAL_GRAPH.get_or_init(init_graph).await.clone() //是否需要克隆
}
