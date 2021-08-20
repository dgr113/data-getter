use std::fs;
use std::collections::HashMap;

use serde_yaml;
use serde_json;
use rayon::prelude::*;

use crate::core::env_utils::err_to_string;
use crate::core::data_utils::DataGroupLayer;
use crate::core::types::{ ResultParse, UnitContent, UnitContentPack, UnitTree };




pub(crate) struct InterfaceLayer;

impl InterfaceLayer {
    fn get_many(d: UnitTree, path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<serde_json::Value> {
        serde_json::to_value(d.par_iter()
            .map( |(node_name, node)| (node_name, DataGroupLayer::tree_walk_recursive( node )) )
            .map( |(node_name, walk_results)| (node_name, ProcessingLayer::get_contents(walk_results, path_field, extract_fields, fields_key_sep)) )
            .collect::<HashMap<_, _>>()
        ).map_err( err_to_string )
    }

    /** Extract root node from Tree */
    pub(crate) fn extract_root_node(tree: &serde_yaml::Value, access_key: &[&str]) -> serde_yaml::Value {
        let mut root_node = DataGroupLayer::get_nested_mapping(tree, access_key).clone();
        if root_node.is_sequence() {
            root_node = root_node.as_sequence().unwrap().first().unwrap().to_owned()
        }
        root_node
    }

    pub(crate) fn process_many(node: serde_yaml::Value, path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<serde_json::Value> {
        let many_getter = move |d| Self::get_many(d, path_field, extract_fields, fields_key_sep);
        serde_yaml::from_value::<HashMap<String, serde_yaml::Value>>( node )
            .map_err( err_to_string )
            .and_then( many_getter )
    }

    pub(crate) fn process_one(node: serde_yaml::Value, path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<serde_json::Value> {
        ProcessingLayer::get_unit_content(&node, path_field, extract_fields, fields_key_sep)
            .and_then( |result| serde_json::to_value( result ).map_err( err_to_string ) )
            .map_err( err_to_string )
    }
}



struct ProcessingLayer;

impl ProcessingLayer {
    /// Get file short description
    ///
    /// # Parameters:
    /// `content`: string form of data content
    /// `keys`: ID keys
    /// `key_sep`: Key separator
    ///
    fn extract_file_content(content: String, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<UnitContent> {
        serde_json::from_str( &content ).and_then( |node| {
            match extract_fields {
                Some( t ) => Ok(
                    t.iter()
                        .map( |key| (key.to_string(), DataGroupLayer::get_nested(&node, key, fields_key_sep)) )
                        .collect::<HashMap<String, serde_json::Value>>()
                ),
                None => serde_json::from_value::<HashMap<String, serde_json::Value>>( node )
            }
        }).map_err( err_to_string )
    }

    fn get_unit_content(obj: &serde_yaml::Value, path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<UnitContent> {
        obj[path_field].as_str().ok_or( String::from("Error contert path field into string") )
            .and_then( |content_path| fs::read_to_string( content_path ).map_err( err_to_string ) )
            .map_err( err_to_string )
            .and_then( |content| Self::extract_file_content(content, extract_fields, fields_key_sep) )
    }

    pub fn build_results_sequence(node: &serde_yaml::Value, path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<UnitContentPack> {
        node.as_sequence().ok_or( String::from( "Error getting contents" ) )
            .and_then( |nodes| {
                Ok(
                    // Fetches all data that has been successfully parsed. Errors are discarded
                    nodes.iter()
                        .map( |node| ProcessingLayer::get_unit_content(node, path_field, extract_fields, fields_key_sep) )
                        .filter( |x| x.is_ok() )
                        .map( |result| result.unwrap() )
                        .collect()
                )
            })
    }

    pub fn get_contents(nodes: Vec<&serde_yaml::Value>, path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> UnitContentPack {
        let seq_processing_func = move |node| Self::build_results_sequence(node, path_field, extract_fields, fields_key_sep);
        nodes.iter().flat_map( |node| seq_processing_func( node ) )
            .flatten()
            .collect()
    }
}
