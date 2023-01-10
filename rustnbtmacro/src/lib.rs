#![allow(unused)]

extern crate proc_macro;
use std::{ops::ControlFlow, collections::HashSet};

use proc_macro::{TokenStream};
use quote::{
    quote,
    quote_spanned, ToTokens,
};

use syn::{
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    token::{
        self,
    },
    punctuated::Punctuated,
    spanned::Spanned,
    parse_macro_input,
    Expr,
    Ident,
    Type,
    Visibility,
    Block,
    Token,
    parenthesized,
};

/*
struct Whatever {
	name: String,
	x: i32,
	z: i32,
	#[in(nbt_to_something), out(nbt_from_something)]
	other: Something,
}
*/

struct Xyz {
	x: i32,
	y: i32,
	z: i32,
}

struct TestStruct {
	name: String,
	x: i32,
	z: i32,
	maybe: Option<String>,
	test: (i32, i32, i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

