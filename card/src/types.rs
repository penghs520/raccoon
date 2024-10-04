use serde::{Deserialize, Serialize};

//关联描述符，由关联关系类型和方向构成
#[derive(Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize)]
pub enum LinkDescriptor {
    Src(String),
    Dest(String),
}

//关联关系路径
#[derive(Debug, Serialize, Deserialize)]
pub enum Path {
    Segment(LinkDescriptor, Box<Path>),
    Nil,
}

// pub type CardId = String;  type只是别名，本质上和原类型无区别
// pub type CardTypeId = String;
// pub type FieldId = String;
// pub type Timestamp = i64;


pub enum InternalField {
    CreateTime,
    UpdateTime,
    Creator
}