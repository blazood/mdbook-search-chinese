use std::{borrow::Borrow, path::PathBuf};
use log::info;
use mdbook::preprocess::Preprocessor;
use crate::{config::Search, search};


pub struct SearchChinese{}

impl Preprocessor for SearchChinese{

    fn name(&self) -> &str {
        return "search";
    }

    fn run(&self, ctx: &mdbook::preprocess::PreprocessorContext, book: mdbook::book::Book) -> mdbook::errors::Result<mdbook::book::Book> {

        // 添加资源
        info!("添加资源");
        if let Some(t) = ctx.config.get_preprocessor(self.name()){
             let search_config: Search = t.into();
             let root: &PathBuf = ctx.root.borrow();
             let path = root.as_path();
             info!("添加文件:{:?}", path);
             search::create_files(&search_config, path, &book)?;
        }
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }

}