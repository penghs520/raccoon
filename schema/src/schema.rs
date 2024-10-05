pub trait Schema {
    fn id(&self) -> &str; //保持和self相同的生命周期
    fn name(&self) -> &str;
    fn org_id(&self) -> &str;
    ///Schema的二级索引
    fn secondary_indexes(&self) -> Option<Vec<String>>;
    fn description(&self) -> &Option<String>;
}