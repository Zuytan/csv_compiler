pub mod raw_formatter;

pub trait Formatter {
    fn compile(&self);
}
