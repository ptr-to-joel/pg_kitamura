#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) value: String,
    pub(super) position: u32,
}
