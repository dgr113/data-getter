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
    let root_node = InterfaceLayer::extract_root_node(tree, access_key);
    match root_node.get( &getter_config.file_path_field_name) {
        Some( _ ) => InterfaceLayer::process_one(root_node, &getter_config),
        None => InterfaceLayer::process_many(root_node, &getter_config)
    }
}



/** Run module as CLI (TESTING ONLY) */
pub fn run_cli() {
    println!( "RUN GETTER AS CLI..." );
}
