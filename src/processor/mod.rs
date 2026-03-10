use crate::{
    Count,
    processor::{
        css::CssProcessor, html::HtmlProcessor, java::JavaProcessor, js::JsProcessor,
        kotlin::KotlinProcessor, rust::RustProcessor, yml::YamlProcessor,
    },
};

mod css;
mod html;
mod java;
mod js;
mod kotlin;
mod rust;
mod yml;

pub trait LangProcessor {
    fn count(&mut self, text: &str) -> Count;
}

struct GenericProcessor {}

impl GenericProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

impl LangProcessor for GenericProcessor {
    fn count(&mut self, text: &str) -> Count {
        let mut c = Count::default();

        for line in text.lines() {
            c.lines += 1;
            if line.trim().is_empty() {
                c.empty += 1;
            }
        }
        c
    }
}

pub fn get_processor(ext: &str) -> Box<dyn LangProcessor + Send> {
    match ext {
        "rs" => Box::new(RustProcessor::new()),
        "js" | "jsx" | "ts" | "tsx" => Box::new(JsProcessor::new()),
        "html" | "htm" | "xml" => Box::new(HtmlProcessor::new()),
        "css" | "scss" | "less" => Box::new(CssProcessor::new()),
        "java" => Box::new(JavaProcessor::new()),
        "yml" | "yaml" => Box::new(YamlProcessor::new()),
        "kt" => Box::new(KotlinProcessor::new()),
        _ => Box::new(GenericProcessor::new()),
    }
}
