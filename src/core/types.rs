use std::collections::HashMap;

use serde_json;
use serde_yaml;
use crate::errors::ApiError;




pub type UnitTree = HashMap<String, serde_yaml::Value>;


pub type ResultParse<T> = Result<T, ApiError>;
pub type UnitContent = HashMap<String, serde_json::Value>;
pub type UnitContentPack = Vec<UnitContent>;
