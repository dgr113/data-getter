#[macro_use] extern crate clap;
extern crate serde_yaml;
extern crate serde_json;

use std::fs;
use clap::App;

mod core;
use crate::core::core_utils::InterfaceLayer;
pub use crate::core::types::ResultParse;




/** Run module wrapper */
pub fn run<'a>(tree: &'a serde_yaml::Value, access_key: &[&str], path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<serde_json::Value> {
    let processing_one = move |node| InterfaceLayer::process_one(node, path_field, extract_fields, fields_key_sep);
    let processing_many = move |node| InterfaceLayer::process_many(node, path_field, extract_fields, fields_key_sep);

    let root_node = InterfaceLayer::extract_root_node(tree, access_key);

    match root_node.get( path_field ) {
        Some( _ ) => processing_one( root_node ),
        None => processing_many( root_node )
    }
}



/** Run module as CLI */
pub fn run_cli() {
    let cli_config = load_yaml!( "../cli.yml" );
    let parser = App::from_yaml( cli_config ).get_matches();

    // User defined variables
    let tree_path = parser.args["config"].vals[0].to_str().unwrap();
    let fields = vec!["title", "body.directors"];
    let access_key = vec!["en-US", "0-0-Movie_1"];

    // Read sources from files
    let tree_str = fs::read_to_string( tree_path ).expect( "Error load json file!" );
    let tree = serde_yaml::from_str( &tree_str ).expect( "Error parse tree file" );

    let results = run(&tree, &access_key, "MESSAGE", Some( &fields ), Some( "." ))
        .unwrap();

    fs::write("results.json", serde_json::to_string( &results ).unwrap())
        .expect("Error writing results file");
}
