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

#[derive(Debug)]
pub struct Date {
    timestamp: i64,
}

impl Date {
    pub fn now() -> Self {
        //SystemTime 提供的时间是基于 UTC 的绝对时间，不依赖于任何时区
        //duration_since(UNIX_EPOCH).expect("Time went backwards");：计算从 Unix 纪元（1970-01-01 00:00:00 UTC）到现在的持续时间。如果系统时间在 Unix 纪元之前，这将返回一个错误
        Self { timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64 }
    }

    pub fn to_local_date(&self) -> NaiveDate {
        // 将时间戳转换为 DateTime<Utc>
        let datetime_utc = chrono::DateTime::<Utc>::from_timestamp_millis(self.timestamp).unwrap();

        // 将 DateTime<Utc> 转换为 DateTime<Local>
        let datetime_local = datetime_utc.with_timezone(&Local);

        // 返回 NaiveDateTime
        datetime_local.date_naive()
    }

    // pub fn to_utc_datetime(&self) -> NaiveDateTime {
    //     chrono::DateTime::from_timestamp_millis(self.timestamp).unwrap().naive_utc()
    // }
}

/// 时间戳
/// 更好的实践：数据库中转换成UtcDateTime或者UtcDate存储, 应用中使用LocalDateTime、LocalDate
/// 但本产品并不打算支持其他时区，所以都使用本地时区
#[derive(Getter, Debug, Serialize, Deserialize, Copy, Clone)]
pub struct DateTime {
    timestamp: i64,
}

impl DateTime {
    pub fn now() -> Self {
        //SystemTime 提供的时间是基于 UTC 的绝对时间，不依赖于任何时区
        //duration_since(UNIX_EPOCH).expect("Time went backwards");：计算从 Unix 纪元（1970-01-01 00:00:00 UTC）到现在的持续时间。如果系统时间在 Unix 纪元之前，这将返回一个错误
        DateTime { timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64 }
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
    use crate::newtypes::DateTime;

    #[test]
    pub fn test_timestamp() {
        let timestamp = DateTime::now();
        println!("timestamp={:?}", timestamp);

        // let utc = timestamp.to_utc_datetime();
        // println!("utc={:?}", utc);

        let local = timestamp.to_local_datetime();
        println!("local={:?}", local);
    }
}