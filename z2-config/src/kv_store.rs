use rmp_serde::decode::Error as MsgPackDecodeError;
use rmp_serde::encode::Error as MsgPackEncodeError;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KvError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("MsgPackEncode error: {0}")]
    MsgPackEncode(#[from] MsgPackEncodeError),

    #[error("MsgPackEncode error: {0}")]
    MsgPackDecode(#[from] MsgPackDecodeError),
}

pub type KvResult<T> = Result<T, KvError>;

#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, Vec<u8>>,
    db_file_path: PathBuf,
}

impl KvStore {
    pub fn new<P: AsRef<Path>>(db_path: P) -> KvStore {
        let map = HashMap::new();
        let mut db_file_path = PathBuf::new();
        db_file_path.push(db_path);

        KvStore { map, db_file_path }
    }

    pub fn load<P: AsRef<Path>>(db_path: P) -> KvResult<KvStore> {
        let content = std::fs::read(db_path.as_ref()).map_err(KvError::Io)?;
        let map = rmp_serde::from_read_ref(&content).map_err(KvError::MsgPackDecode)?;

        let mut db_file_path = PathBuf::new();
        db_file_path.push(db_path);

        Ok(KvStore { map, db_file_path })
    }

    pub fn get<V>(&self, key: &str) -> Option<V>
    where
        V: DeserializeOwned + Debug,
    {
        self.map
            .get(key)
            .map(|value| rmp_serde::from_slice(&value).ok())
            .flatten()
    }

    pub fn set<V>(&mut self, key: &str, value: V) -> Result<(), KvError>
    where
        V: Serialize + Debug,
    {
        let data = rmp_serde::to_vec(&value)?;
        self.map.insert(key.to_string(), data);

        self.dump()?;

        Ok(())
    }

    pub fn dump(&self) -> KvResult<()> {
        let ser_db = rmp_serde::to_vec(&self.map)?;
        let _ = std::fs::write(&self.db_file_path, ser_db)?;

        Ok(())
    }
}
