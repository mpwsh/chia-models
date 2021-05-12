use std::path::Path;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use reqwest::Identity;
use serde::de::Deserializer;
use serde::de::Deserialize;
use tokio::fs::read;

use crate::Error;

pub(crate) async fn load_pem_pair(key: impl AsRef<Path>, cert: impl AsRef<Path>) -> Result<Identity, Error> {
	let mut buf = read(key.as_ref()).await?;
	buf.append(&mut read(cert.as_ref()).await?);
	Ok(Identity::from_pem(&buf)?)
}

pub(crate) fn deserialize_optional_timestamp<'de, D: Deserializer<'de>>(d: D) -> Result<Option<DateTime<Utc>>, D::Error> {
	match Option::<i64>::deserialize(d)? {
		Some(ts) => Ok(Some(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts, 0), Utc))),
		None => Ok(None)
	}
}

