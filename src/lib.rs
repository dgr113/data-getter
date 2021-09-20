mod core;
pub mod config;
pub mod errors;

use crate::core::core_utils::InterfaceLayer;
pub use crate::core::types::ResultParse;
pub use crate::config::GetterConfig;




/** Run module wrapper */
pub fn run<S>(tree: &serde_yaml::Value, getter_config: GetterConfig, access_key: &[S])
    -> ResultParse<serde_json::Value>
        where S: Into<String> + serde_yaml::Index
{
    println!( "Data GETTER STARTING ..." );

    let root_node = InterfaceLayer::extract_root_node(tree, access_key);
    match root_node.get( &getter_config.file_path_field_name) {
        Some( _ ) => InterfaceLayer::process_one(root_node, &getter_config),
        None => InterfaceLayer::process_many(root_node, &getter_config)
    }
}



/** Run module as CLI (TESTING ONLY) */
pub fn run_cli() {
    println!( "RUN GETTER AS CLI..." );

    let tree_str = r#"
        "":
              7-7-Movie_10:
                - MESSAGE: /opt/data/assets/7-7-Movie_10/7-7-Movie_10.json
                  ASSET_ID: 7-7-Movie_10
                  BASE_PATH: /opt/data/assets/7-7-Movie_10/
                  ASSET_LANG: ""
              7-7-Movie_122:
                - ASSET_LANG: ""
                  MESSAGE: /opt/data/assets/7-7-Movie_122/7-7-Movie_122.json
                  ASSET_ID: 7-7-Movie_122
                  BASE_PATH: /opt/data/assets/7-7-Movie_122/
    "#;
}
