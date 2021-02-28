#[macro_use]
extern crate lazy_static;
use std::io::Write;
use crate::processor::SearchChinese;
use clap::{App, Arg, ArgMatches, SubCommand};
use env_logger::Builder;
use chrono::Local;
use log::{LevelFilter, info};
use mdbook::{preprocess::Preprocessor, renderer::RenderContext};
use toml_edit::{Array, Document, Item, Table, Value};
use std::{collections::HashSet, path::Path, env, process, str::FromStr};
use std::io;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor};
use crate::config::Search;
mod  config;
mod search;
mod theme;
mod processor;

const ZH_JS: &[u8] = include_bytes!("theme/searcher/zh.js");

fn main() {
    init_logger();

    let matches = make_app().get_matches();

    let preprocessor = SearchChinese{};

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Some(_) = matches.subcommand_matches("install") {
        handle_install("./");
    } else if let Some(_) = matches.subcommand_matches("preprocessor") {
        if let Err(e) = handle_preprocessing(&preprocessor) {
            eprintln!("{}", e);
            process::exit(1);
        }
    } else {
        let mut stdin = io::stdin();
        let ctx = RenderContext::from_json(&mut stdin).unwrap();
        
    }
}


#[test]
pub fn test(){
    handle_install("/mnt/d/dev/project/philo-rust/common/mydoc");
}

// install search into your book.toml
fn handle_install(path: impl AsRef<Path>) -> () {
    let bt = path.as_ref().join("book.toml");
    let book_toml = std::fs::read_to_string(bt.clone()).expect(format!("nof found book.toml in path {}", "./book.toml").as_str());


    let zh = "zh.js".to_string();

    let mut doc = Document::from_str(book_toml.as_str()).unwrap();

    let root = doc.root.as_table_mut().unwrap();

    if let Some(o) =  root.entry("output").as_table_mut(){
        if let Some(oo) = o.entry("html").as_table_mut(){
            if let Some(a) =  oo.entry("additional-js").as_array_mut(){
                let ass = a.iter().fold(HashSet::new(), |mut s, v| {
                    s.insert(v.as_str().unwrap().to_string());
                    s
                });
                println!("{:?}", ass);
                if !ass.contains(&zh) {
                    a.push(zh).unwrap();
                }
            }

            if let Some(pld_s) = oo.entry("search").as_table_mut(){
                pld_s.remove("enable");
                let d = Value::from(false);
                pld_s.entry("enable").or_insert(Item::Value(d));
            } else {
                let mut t =Table::new();
                let d = Value::from(false);
                t.entry("enable").or_insert(Item::Value(d));
                oo.entry("search").or_insert(Item::Table(t));
            }
        } else {
            let mut tt = Table::new();

            {
                let mut a = Array::default();
                a.push(zh).unwrap();
                tt.entry("additional-js").or_insert(Item::Value(Value::Array(a)));
            }
            {
                let mut t =Table::new();
                let d = Value::from(false);
                t.entry("enable").or_insert(Item::Value(d));
                tt.entry("search").or_insert(Item::Table(t));
            }

            o.entry("html").or_insert(Item::Table(tt));

        }    
    } else {
        let mut t =Table::new();
        let  mut tt = t.clone();
        let mut a = Array::default();
        a.push(zh).unwrap();
        tt.entry("additional-js").or_insert(Item::Value(Value::Array(a)));
        t.entry("html").or_insert(Item::Table(tt));
        root.entry("output").or_insert(Item::Table(t));
    }

    if let Some(o) =  root.entry("preprocessor").as_table_mut(){
        let s = Search::default();
        let t = toml::to_string_pretty(&s).unwrap();
        let c = Document::from_str(t.as_str()).unwrap();
        o.entry("search").or_insert(c.root);
    }


    std::fs::write(bt, doc.to_string().as_bytes()).unwrap();

    let zh = path.as_ref().join("zh.js");

    if !zh.exists(){
        std::fs::write(zh, ZH_JS).unwrap();
    }

}

pub fn make_app() -> App<'static, 'static> {
    App::new("nop-preprocessor")
        .about("A mdbook preprocessor which search with chinese")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            SubCommand::with_name("install")
            .about("install search into your book.toml")
        )
        .subcommand(
            SubCommand::with_name("preprocessor")
            .about("preprocessor")
        )
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    info!("search start");
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}




fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
        // Filter extraneous html5ever not-implemented messages
        builder.filter(Some("html5ever"), LevelFilter::Error);
    }

    builder.init();
}
