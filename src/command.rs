use std::collections::HashMap;
pub struct Command<'a> {
    pub keyword: &'a str,
    pub help: &'a str,
    pub func: fn(Vec<&str>,&HashMap<&str,String>,&HashMap<&str,Command<'a>>)-> String,
}
