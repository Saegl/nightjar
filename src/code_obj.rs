use crate::value::Value;

#[derive(Debug)]
pub struct CodeObject {
    pub code: Vec<u8>,
    pub consts: Vec<Value>,
}
