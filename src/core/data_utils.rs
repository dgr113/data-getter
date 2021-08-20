use serde_yaml;




pub(crate) struct DataGroupLayer;

impl DataGroupLayer {
    /** Nested getter for serde-json Value object
    *
    * # Parameters:
    * `obj`: Recursive-like nested object
    * `keys`: Keys path for access to nested object level
    *
    * # Returns: Nested serde-json Value
    */
    fn nested_getter(obj: &serde_json::Value, keys: Vec<&str>) -> serde_json::Value {
        keys.iter().fold(obj.clone(), |result, k| result[k].clone())
    }

    /** Nested getter wrapper function
    *
    * # Parameters:
    * `obj`: Recursive-like nested object
    * `key`: Key path for access to nested object level
    * `key_sep`: Key separator
    *
    * # Returns: Nested serde-json Value
    */
    pub(crate) fn get_nested(obj: &serde_json::Value, key: &str, key_sep: Option<&str>) -> serde_json::Value {
        let access_key = match key_sep {
            Some(t) => key.split(t).collect::<Vec<&str>>(),
            None => vec![key, ]
        };
        Self::nested_getter(obj, access_key)
    }

    /** Get nested element from Tree */
    pub(crate) fn get_nested_mapping<'a>(tree: &'a serde_yaml::Value, keys: &[&str]) -> &'a serde_yaml::Value {
        let mut curr_link = tree;
        for k in keys {
            curr_link = &curr_link[k];
        }
        curr_link
    }

    /** Further walk on the Tree with all data collect (useful for non-completely keys) */
    pub(crate) fn tree_walk_recursive(node: &serde_yaml::Value) -> Vec<&serde_yaml::Value> {
        let mut results = Vec::new();
        match node.as_mapping() {
            Some(t) => {
                for (_, v) in t.iter() {
                    match &v.is_mapping() {
                        true => results.extend(Self::tree_walk_recursive(v)),
                        false => results.push(v )
                    };
                }
            },
            None => {
                results.push(&node)
            }
        }
        results
    }
}
