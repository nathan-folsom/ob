pub trait Convert {
    fn to_binary(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}
