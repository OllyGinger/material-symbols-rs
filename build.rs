use std::{env, fs};

use heck::ToPascalCase;
use quote::{format_ident, quote};

fn main() {
    let codepoints = include_str!("data/codepoints.txt");

    let mut idents = Vec::new();
    let mut match_arms = Vec::new();
    let mut codepoint_match_arms = Vec::new();
    for line in codepoints.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let space = line.find(' ').expect("malformed codepoints.txt");
        let symbol = &line[..space];
        let codepoint = u32::from_str_radix(&line[space + 1..], 16)
            .expect(&format!("malformed codepoints.txt (invalid codepoint in: \"{}\")", line));
        let mut symbol_pascal_case = symbol.to_pascal_case();

        if !symbol_pascal_case
            .chars()
            .next()
            .unwrap()
            .is_ascii_alphabetic()
        {
            symbol_pascal_case = format!("_{symbol_pascal_case}");
        }
        let ident = format_ident!("{symbol_pascal_case}");
        let codepoint_char = std::char::from_u32(codepoint);
        idents.push(ident.clone());
        match_arms.push(quote! {
            Symbol::#ident => #symbol
        });

        codepoint_match_arms.push(quote! {
            Symbol::#ident => {#codepoint_char}
        });
    }

    let result = quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[doc = "A Material Symbol. See [this page](https://fonts.google.com/icons) for a visual list."]
        pub enum Symbol {
            #(#idents,)*
        }

        impl Symbol {
            #[doc = "Returns the snake_case name of this symbol, which corresponds to the ligature used to render it."]
            pub fn name(self) -> &'static str {
                match self {
                    #(#match_arms,)*
                }
            }

            #[doc = "Returns the snake_case name of this symbol, which corresponds to the ligature used to render it."]
            pub fn codepoint(self) -> char {
                match self {
                    #(#codepoint_match_arms,)*
                }
            }
        }
    };
    let result = result.to_string();
    fs::write(
        format!("{}/generated.rs", env::var("OUT_DIR").unwrap()),
        result.as_bytes(),
    )
    .expect("failed to write generated code");
}
