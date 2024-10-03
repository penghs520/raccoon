use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::time::SystemTime;
use chrono::{TimeZone};
use serde::{Deserialize, Serialize};
use common::id_generator;

pub mod card_id {
    use super::*;

    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Serialize,
        Deserialize,
    )] //为了newtype与底层类型相似，需要派生这些属性，然后还要实现Display
    pub struct CardId(String); //不要这样使用pub，pub struct CardId(pub String)，因为这样做外部可以任意构造CardId而不经过校验

    impl CardId {

        pub fn from(id: String) -> Self {
            CardId(id)
        }

        pub fn from_str(id: &str) -> Self {
            CardId(String::from(id))
        }

        pub fn new() -> Self {
            let id = id_generator::generate_id();
            Self::from(id)
        }
    }

    // 实现 Deref trait
    impl Deref for CardId {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Display for CardId {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

pub mod field_id {
    use super::*;

    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Serialize,
        Deserialize,
    )] //为了newtype与底层类型相似，需要派生这些属性，然后还要实现Display
    pub struct FieldId(String); //不要这样使用pub，pub struct CardId(pub String)，因为这样做外部可以任意构造CardId而不经过校验

    impl FieldId {
        pub fn from(id: String) -> Self {
            FieldId(id)
        }

        pub fn from_str(id: &str) -> Self {
            FieldId(String::from(id))
        }

        pub fn new() -> Self {
            let id = id_generator::generate_id();
            Self::from(id)
        }
    }

    // 实现 Deref trait
    impl Deref for FieldId {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Display for FieldId {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

pub mod card_type_id {
    use super::*;

    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Serialize,
        Deserialize,
    )] //为了newtype与底层类型相似，需要派生这些属性，然后还要实现Display
    pub struct CardTypeId(String); //不要这样使用pub，pub struct CardId(pub String)，因为这样做外部可以任意构造CardId而不经过校验

    impl CardTypeId {
        pub fn from(id: String) -> Self {
            CardTypeId(id)
        }

        pub fn from_str(id: &str) -> Self {
            CardTypeId(String::from(id))
        }

        pub fn new() -> Self {
            let id = id_generator::generate_id();
            Self::from(id)
        }
    }

    // 实现 Deref trait
    impl Deref for CardTypeId {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Display for CardTypeId {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

pub mod timestamp {
    use super::*;
    /// 时间戳
    /// 最佳实践：数据库和应用层都使用时间戳打交道，输出给前端的也是时间戳，由前端来复杂本地化显示
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Serialize,
        Deserialize
    )] //为了newtype与底层类型相似，需要派生这些属性，然后还要实现Display
    pub struct Timestamp(i64);


    impl Deref for Timestamp {
        type Target = i64;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Display for Timestamp {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Timestamp {
        pub fn now() -> Self {
            //SystemTime 提供的时间是基于 UTC 的绝对时间，不依赖于任何时区
            //duration_since(UNIX_EPOCH).expect("Time went backwards");：计算从 Unix 纪元（1970-01-01 00:00:00 UTC）到现在的持续时间。如果系统时间在 Unix 纪元之前，这将返回一个错误
            Timestamp(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64)
        }

        // pub fn to_local_datetime(&self) -> NaiveDateTime {
        //     // 将时间戳转换为 DateTime<Utc>
        //     let datetime_utc = chrono::DateTime::<Utc>::from_timestamp_millis(self.0).unwrap();
        //
        //     // 将 DateTime<Utc> 转换为 DateTime<Local>
        //     let datetime_local = datetime_utc.with_timezone(&Local);
        //
        //     // 返回 NaiveDateTime
        //     datetime_local.naive_local()
        // }

        // pub fn to_utc_datetime(&self) -> NaiveDateTime {
        //     chrono::DateTime::from_timestamp_millis(self.0).unwrap().naive_utc()
        // }
    }
}

#[cfg(test)]
mod test {
    use crate::newtypes::card_id::CardId;
    use crate::newtypes::timestamp::Timestamp;
    use super::*;
    #[test]
    pub fn test_card_id() {
        let card_id = CardId::from_str("123");
        println!("{:?}", card_id);
        println!("{}", card_id);

        assert_eq!(*card_id, String::from("123"));

        let card_id_str: &str = &card_id; //等价于 card_id.as_str()
        assert_eq!(card_id_str, "123");

        assert_eq!(CardId::from_str("123"), CardId::from_str("123"));

        let json = serde_json::to_string(&card_id).unwrap();
        println!("{}", json);
    }

    pub fn test_timestamp() {
        let timestamp = Timestamp::now();

        println!("{:?}", timestamp);
    }
}