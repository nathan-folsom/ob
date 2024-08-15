use crate::{modules, types};

/// Represents a wasm module
/// https://webassembly.github.io/spec/core/syntax/modules.html
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

impl Module {
    pub fn instantiate() -> Self {
        todo!()
    }
}
