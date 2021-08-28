use serde::{ Serialize, Deserialize };




/** 'file_path_field_name' ('MESSAGE' by default) field is field that contains full path to asset json data into assets Tree:

    0-3-Mus_11:
    - MESSAGE: .../assets/0-3-Mus_11/0-3-Mus_11.json
      BASE_PATH: .../assets/0-3-Mus_11/
      ASSET_ID: 0-3-Mus_11
      ASSET_LANG: "RU"
*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetterConfig {
    pub file_path_field_name: String,  // Name of field that containing contents of the file ( Ex `path_field` )
    pub extract_fields: Option<Vec<String>>,
    pub fields_key_sep: Option<String>
}
