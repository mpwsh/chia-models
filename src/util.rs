use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::de::Deserialize;
use serde::de::Deserializer;

pub(crate) fn deserialize_optional_timestamp<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<DateTime<Utc>>, D::Error> {
    match Option::<i64>::deserialize(d)? {
        Some(ts) => Ok(Some(DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(ts, 0).unwrap(),
            Utc,
        ))),
        None => Ok(None),
    }
}
