use std::time::SystemTime;
use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use my_proc_macros::Getter;

//关联描述符，由关联关系类型和方向构成
#[derive(Debug, Serialize, Deserialize)]
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

/// 时间戳
/// 最佳实践：数据库和应用层都使用时间戳打交道，输出给前端的也是时间戳，由前端来复杂本地化显示
#[derive(Getter, Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Timestamp {
    timestamp: i64,
}

impl Timestamp {
    pub fn now() -> Self {
        //SystemTime 提供的时间是基于 UTC 的绝对时间，不依赖于任何时区
        //duration_since(UNIX_EPOCH).expect("Time went backwards");：计算从 Unix 纪元（1970-01-01 00:00:00 UTC）到现在的持续时间。如果系统时间在 Unix 纪元之前，这将返回一个错误
        Timestamp { timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64 }
    }

    pub fn to_local_datetime(&self) -> NaiveDateTime {
        // 将时间戳转换为 DateTime<Utc>
        let datetime_utc = chrono::DateTime::<Utc>::from_timestamp_millis(self.timestamp).unwrap();

        // 将 DateTime<Utc> 转换为 DateTime<Local>
        let datetime_local = datetime_utc.with_timezone(&Local);

        // 返回 NaiveDateTime
        datetime_local.naive_local()
    }

    // pub fn to_utc_datetime(&self) -> NaiveDateTime {
    //     chrono::DateTime::from_timestamp_millis(self.timestamp).unwrap().naive_utc()
    // }
}

#[cfg(test)]
mod test {
    use crate::newtypes::Timestamp;

    #[test]
    pub fn test_timestamp() {
        let timestamp = Timestamp::now();
        println!("timestamp={:?}", timestamp);

        // let utc = timestamp.to_utc_datetime();
        // println!("utc={:?}", utc);

        let local = timestamp.to_local_datetime();
        println!("local={:?}", local);
    }
}