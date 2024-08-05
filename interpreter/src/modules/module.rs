use crate::{modules, types};

pub struct Module {
    types: Vec<types::function::Function>,
    funcs: Vec<modules::function::Function>,
    tables: Vec<modules::table::Table>,
    mems: Vec<modules::memory::Memory>,
    globals: Vec<modules::global::Global>,
    elems: Vec<modules::element_segments::ElementSegment>,
    datas: Vec<modules::data_segments::DataSegments>,
    start: Option<modules::function_index::FunctionIndex>,
    imports: Vec<modules::import::Import>,
    exports: Vec<modules::export::Export>,
}
