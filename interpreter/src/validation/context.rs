use crate::{modules, types};

pub struct Context {
    types: Vec<types::function::Function>,
    functions: Vec<types::function::Function>,
    tables: Vec<types::table::Table>,
    memories: Vec<types::memory::Memory>,
    globals: Vec<types::global::Global>,
    element_segments: Vec<types::reference::Reference>,
    data_segments: Vec<()>,
    locals: Vec<types::value::Value>,
    labels: Vec<types::result::Result>,
    ret: Option<types::result::Result>,
    references: Vec<modules::function_index::FunctionIndex>,
}
