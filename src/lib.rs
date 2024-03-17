//! This crate provides a simple generated type, `Symbol`, to represent
//! any one of the many [Material Symbols](https://fonts.google.com/icons).

#![doc(html_root_url = "https://docs.rs/material-symbols/0.1.0")]

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(test)]
mod tests {
    use super::Symbol;

    #[test]
    fn symbol_works() {
        assert_eq!(Symbol::Menu.name(), "menu");
    }


    #[test]
    fn codepoint_works() {
        assert_eq!(Symbol::Menu.codepoint(), '\u{e5d2}');
    }
}
