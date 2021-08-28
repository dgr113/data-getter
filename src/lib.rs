mod core;
pub mod config;
pub mod errors;

use crate::core::core_utils::InterfaceLayer;
pub use crate::core::types::ResultParse;
pub use crate::config::GetterConfig;




// /** Run module wrapper */
// pub fn run<'a>(tree: &'a serde_yaml::Value, access_key: &[&str], path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<serde_json::Value> {
//     let processing_one = move |node| InterfaceLayer::process_one(node, path_field, extract_fields, fields_key_sep);
//     let processing_many = move |node| InterfaceLayer::process_many(node, path_field, extract_fields, fields_key_sep);
//
//     let root_node = InterfaceLayer::extract_root_node(tree, access_key);
//
//     match root_node.get( path_field ) {
//         Some( _ ) => processing_one( root_node ),
//         None => processing_many( root_node )
//     }
// }

/** Run module wrapper */
pub fn run<'a>(tree: &'a serde_yaml::Value, getter_config: GetterConfig) -> ResultParse<serde_json::Value> {
    let root_node = InterfaceLayer::extract_root_node(tree, getter_config.access_key.as_slice());
    match root_node.get( &getter_config.path_field ) {
        Some( _ ) => InterfaceLayer::process_one(root_node, &getter_config),
        None => InterfaceLayer::process_many(root_node, &getter_config)
    }
}



/** Run module as CLI (TESTING ONLY) */
pub fn run_cli() {
    println!( "RUN GETTER AS CLI..." );
}
