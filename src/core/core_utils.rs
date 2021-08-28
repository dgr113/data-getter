use std::fs;
use std::collections::HashMap;
use std::hash::Hash;
use serde::Deserialize;
use std::fmt::Display;

use serde_yaml;
use serde_json;
use rayon::prelude::*;

use crate::errors::ApiError;
use crate::config::GetterConfig;
use crate::core::data_utils::DataGroupLayer;
use crate::core::types::{ ResultParse, UnitContent, UnitContentPack, UnitTree };




pub( crate ) struct InterfaceLayer;

impl InterfaceLayer {
    fn get_many(d: UnitTree, getter_config: &GetterConfig) -> ResultParse<serde_json::Value> {
        let res = serde_json::to_value(d.par_iter()
            .map( |(node_name, node)| (node_name, DataGroupLayer::tree_walk_recursive( node )) )
            .map( |(node_name, walk_results)| (node_name, ProcessingLayer::get_contents(walk_results, getter_config)) )
            .collect::<HashMap<_, _>>()
        ) ?;
        Ok( res )
    }

    /** Extract root node from Tree */
    pub( crate ) fn extract_root_node<S: Into<String> + serde_yaml::Index>(tree: &serde_yaml::Value, access_key: &[S]) -> serde_yaml::Value {
        let mut root_node = DataGroupLayer::get_nested_mapping(tree, access_key).clone();
        if root_node.is_sequence() {
            root_node = root_node.as_sequence().unwrap().first().unwrap().to_owned()
        }
        root_node
    }

    pub( crate ) fn process_many(node: serde_yaml::Value, getter_config: &GetterConfig) -> ResultParse<serde_json::Value> {
        let many_getter = move |d| Self::get_many(d, getter_config);
        let res = serde_yaml::from_value::<HashMap<String, serde_yaml::Value>>( node ) ?;
        many_getter( res )
        // serde_yaml::from_value::<HashMap<String, serde_yaml::Value>>( node )
        //     // .map_err( err_to_string )
        //     .and_then( many_getter )
    }

    pub( crate ) fn process_one(node: serde_yaml::Value, getter_config: &GetterConfig) -> ResultParse<serde_json::Value> {
        let a = ProcessingLayer::get_unit_content::<String>(&node, getter_config) ?;
        let res = serde_json::to_value( a ) ?;
        Ok( res )
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
    fn extract_file_content<'a, S>(content: String, extract_fields: Option<&Vec<S>>, fields_key_sep: Option<String>)
        -> ResultParse<UnitContent>
            where S: Into<String> + Display + ToString + Clone + Eq + Hash + Deserialize<'a>
    {
        let res = serde_json::from_str( &content ).and_then( |node| {
            match extract_fields {
                Some( t ) => Ok(
                    t.iter()
                        .map( |key| {
                            // let b = fields_key_sep.clone();
                            let res = (key.to_string(), DataGroupLayer::get_nested(&node, &key.to_string(), fields_key_sep.as_ref()));
                            res
                        })
                        .collect::<HashMap<String, serde_json::Value>>()
                ),
                None => serde_json::from_value::<HashMap<String, serde_json::Value>>( node )
            }
        }) ?;

        Ok( res )
            // .map_err( |err| err_to_string( err ) )
            // .map(|res| res)
    }

    // fn get_unit_content(obj: &serde_yaml::Value, path_field: &str, extract_fields: Option<&Vec<&str>>, fields_key_sep: Option<&str>) -> ResultParse<UnitContent> {
    fn get_unit_content<S: Into<String>>(obj: &serde_yaml::Value, getter_config: &GetterConfig) -> ResultParse<UnitContent> {
        let content_path = obj[&getter_config.file_path_field_name].as_str().ok_or( ApiError::SerdeError( "Error content path field into string".to_string() ) ) ?;
        let content = fs::read_to_string( content_path ) ?;
        Self::extract_file_content(content, getter_config.extract_fields.as_ref(), getter_config.fields_key_sep.clone())
    }

    pub fn build_results_sequence(node: &serde_yaml::Value, getter_config: &GetterConfig) -> ResultParse<UnitContentPack> {
        node.as_sequence().ok_or( ApiError::SimpleMsgError( "Error getting contents".to_string() ) )
            .map( |nodes| {
                    // Fetches all data that has been successfully parsed. Errors are discarded
                    nodes.iter()
                        .map( |node| ProcessingLayer::get_unit_content::<String>(node, getter_config) )
                        .filter( |x| x.is_ok() )
                        .map( |result| result.unwrap() )
                        .collect()
            })
    }

    pub fn get_contents(nodes: Vec<&serde_yaml::Value>, getter_config: &GetterConfig) -> UnitContentPack {
        let seq_processing_func = move |node| {
            Self::build_results_sequence(node, getter_config)
        };
        nodes.iter().flat_map( |node| seq_processing_func( node ) )
            .flatten()
            .collect()
    }
}
