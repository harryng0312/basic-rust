pub mod vec_rw_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::sync::{Arc, RwLock};

    // pub fn serialize<S, T>(val: &Arc<RwLock<Vec<T>>>, serializer: S) -> Result<S::Ok, S::Error>
    pub fn serialize<S, T>(val: &Arc<RwLock<Vec<T>>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        let read = val.read().map_err(serde::ser::Error::custom)?;
        read.serialize(serializer)
    }

    // pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Arc<RwLock<Vec<T>>>, D::Error>
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Arc<RwLock<Vec<T>>>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let vec = Vec::<T>::deserialize(deserializer)?;
        Ok(Arc::new(RwLock::new(vec)))
    }
}
