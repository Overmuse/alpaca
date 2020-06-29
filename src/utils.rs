use std::fmt::Display;
use std::str::FromStr;
//use log::warn;
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;
//use serde_json::Value;

pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
                
}

pub fn to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where T: Display,
          S: Serializer
{
    serializer.collect_str(value)
}

//pub fn from_str_optional<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
//    where T: FromStr,
//                    T::Err: Display,
//                              D: serde::Deserializer<'de>
//{
//        let deser_res: Result<Value, _> = serde::Deserialize::deserialize(deserializer);
//        match deser_res {
//                    Ok(Value::String(s)) => T::from_str(&s).map_err(serde::de::Error::custom).map(Option::from),
//                    Ok(v) => {
//                                    warn!("string expected but found something else: {}", v);
//                                                Ok(None)
//                                                            
//                    },
//                            Err(_) => Ok(None)
//                                    
//        }
//        
//}
