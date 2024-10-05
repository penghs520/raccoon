use serde::{Deserialize, Serialize};
use crate::schema::Schema;

#[derive(Debug, Serialize, Deserialize)]
pub enum CardType {
    MemberType(MemberType),
    TeamType(TeamType),
    WorkItemType(WorkItemType),
    CommonTraitType(CommonTraitType),
}

///公共特性类型，被其他卡片类所继承，达到拥有公共属性或关联的目的
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonTraitType {
    id: String,
    name: String,
    org_id: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberType {
    id: String,
    name: String,
    org_id: String,
    description: Option<String>,
    trait_ids: Option<Vec<String>>,
    permission: Option<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamType {
    id: String,
    name: String,
    org_id: String,
    description: Option<String>,
    trait_ids: Option<Vec<String>>,
    permission: Option<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkItemType {
    id: String,
    name: String,
    org_id: String,
    description: Option<String>,
    trait_ids: Option<Vec<String>>,
    permission: Option<Permission>,
    card_faces: Vec<CardFace>, //普通版只能定义一个卡片，并且卡面的自定义度也有限
}


impl Schema for CardType {
    fn id(&self) -> &str {
        match self {
            CardType::MemberType(it) => {
                &it.id
            }
            CardType::TeamType(it) => {
                &it.id
            }
            CardType::WorkItemType(it) => {
                &it.id
            }
            CardType::CommonTraitType(it) => {
                &it.id
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            CardType::MemberType(it) => {
                &it.name
            }
            CardType::TeamType(it) => {
                &it.name
            }
            CardType::WorkItemType(it) => {
                &it.name
            }
            CardType::CommonTraitType(it) => {
                &it.name
            }
        }
    }

    fn org_id(&self) -> &str {
        match self {
            CardType::MemberType(it) => {
                &it.org_id
            }
            CardType::TeamType(it) => {
                &it.org_id
            }
            CardType::WorkItemType(it) => {
                &it.org_id
            }
            CardType::CommonTraitType(it) => {
                &it.org_id
            }
        }
    }

    fn secondary_indexes(&self) -> Option<Vec<String>> {
        match self {
            CardType::MemberType(it) => {
                it.trait_ids.clone()
            }
            CardType::TeamType(it) => {
                it.trait_ids.clone()
            }
            CardType::WorkItemType(it) => {
                it.trait_ids.clone()
            }
            CardType::CommonTraitType(it) => {
                None
            }
        }
    }

    fn description(&self) -> &Option<String> {
        match self {
            CardType::MemberType(it) => {
                &it.description
            }
            CardType::TeamType(it) => {
                &it.description
            }
            CardType::WorkItemType(it) => {
                &it.description
            }
            CardType::CommonTraitType(it) => {
                &it.description
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardFace {}
