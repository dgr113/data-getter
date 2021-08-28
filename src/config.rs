use serde::{ Serialize, Deserialize };




#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetterConfig {
    pub access_key: Vec<String>,
    pub path_field: String,
    pub extract_fields: Option<Vec<String>>,
    pub fields_key_sep: Option<String>
}
