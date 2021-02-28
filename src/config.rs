use elasticlunr::Language;
use serde::{Deserialize, Serialize};
use toml::value::Table;


/// Configuration of the search functionality of the HTML renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Search {
    pub command: String,
    /// Enable the search feature. Default: `true`.
    pub enable: bool,
    /// Maximum number of visible results. Default: `30`.
    pub limit_results: u32,
    /// The number of words used for a search result teaser. Default: `30`.
    pub teaser_word_count: u32,
    /// Define the logical link between multiple search words.
    /// If true, all search words must appear in each result. Default: `false`.
    pub use_boolean_and: bool,
    /// Boost factor for the search result score if a search word appears in the header.
    /// Default: `2`.
    pub boost_title: u8,
    /// Boost factor for the search result score if a search word appears in the hierarchy.
    /// The hierarchy contains all titles of the parent documents and all parent headings.
    /// Default: `1`.
    pub boost_hierarchy: u8,
    /// Boost factor for the search result score if a search word appears in the text.
    /// Default: `1`.
    pub boost_paragraph: u8,
    /// True if the searchword `micro` should match `microwave`. Default: `true`.
    pub expand: bool,
    /// Documents are split into smaller parts, seperated by headings. This defines, until which
    /// level of heading documents should be split. Default: `3`. (`### This is a level 3 heading`)
    pub heading_split_level: u8,
    /// Copy JavaScript files for the search functionality to the output directory?
    /// Default: `true`.
    pub copy_js: bool,
    /// language
    /// elasticlunr_rs::lang::Language
    pub language: String, 
}


impl Default for Search {
    fn default() -> Search {
        // Please update the documentation of `Search` when changing values!
        Search {
            command: "search".to_string(),
            enable: true,
            limit_results: 30,
            teaser_word_count: 30,
            use_boolean_and: false,
            boost_title: 2,
            boost_hierarchy: 1,
            boost_paragraph: 1,
            expand: true,
            heading_split_level: 3,
            copy_js: true,
            language: "en".to_string(),
        }
    }
}

fn get_u32(t: &Table, key: &str, default: u32) -> u32{
    t.get(key).map(|v|{
        match v {
            toml::Value::Integer(i) => (*i) as u32,
            _ => default
        }
    }).unwrap_or(default)
}

fn get_u8(t: &Table, key: &str, default: u8) -> u8{
    t.get(key).map(|v|{
        match v {
            toml::Value::Integer(i) => (*i) as u8,
            _ => default
        }
    }).unwrap_or(default)
}

fn get_bool(t: &Table, key: &str, default: bool) -> bool{
    t.get(key).map(|v|{
        match v{
            toml::Value::String(vv) => {
                if "true" == vv {
                    true
                } else {
                    false
                }
            }
            toml::Value::Boolean(vv) => *vv,
            _ => default
        }
    }).unwrap_or(default)
}

impl From<&Table> for Search{
    fn from(t: &Table) -> Self {

        let language = t.get("language").map(|e|{
            match e {
                toml::Value::String(s) => s.to_string(),
                _ => Language::English.to_code().to_string()
            }
        }).unwrap_or(Language::English.to_code().to_string());
        
        Search {
            command: "search".to_string(),
            enable: get_bool(t, "enable", true),
            limit_results: get_u32(t, "limit_results", 30),
            teaser_word_count: get_u32(t, "teaser_word_count", 30),
            use_boolean_and: get_bool(t, "use_boolean_and", true),
            boost_title: get_u8(t, "boost_title", 2),
            boost_hierarchy: get_u8(t, "boost_hierarchy", 1),
            boost_paragraph: get_u8(t, "boost_paragraph", 1),
            expand: get_bool(t, "expand", true),
            heading_split_level: get_u8(t, "heading_split_level", 3),
            copy_js: get_bool(t, "copy_js", true),
            language: language,
        }
    }
}