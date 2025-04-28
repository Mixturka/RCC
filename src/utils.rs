use std::fmt;

const INDENT: &str = "  ";

pub(crate) fn write_indent(indent: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for _ in 0..indent {
        write!(f, "{}", INDENT)?;
    }
    Ok(())
}
