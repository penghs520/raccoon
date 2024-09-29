use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewDefinition {
    id: u32,
    name: String,
    description: String,
    tenant_id: u32,
    view_type: ViewType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ViewType {
    ListView(ListViewDefinition),
    BoardView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ListViewDefinition {
    columns: Vec<Column>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Column {}


mod tests {
    use super::*;

    #[test]
    fn list_view_definition() {
        let list_view_def = ViewDefinition {
            id: 1,
            name: String::from("列表视图"),
            description: String::from("这是一个列表视图"),
            tenant_id: 1,
            view_type: ViewType::ListView(
                ListViewDefinition {
                    columns: vec![]
                }
            ),
        };
        let string = serde_json::to_string(&list_view_def).unwrap();
        println!("string: {}", string);
        let list_view_def = serde_json::from_str::<ViewDefinition>(&string).unwrap();
        assert_eq!(list_view_def.id, 1);
        assert_eq!(list_view_def.view_type, ViewType::ListView(
            ListViewDefinition {
                columns: vec![]
            }
        ));
        assert_eq!(list_view_def.name, "列表视图");
    }
}