/*
The code in this library is heavily inspired (copied) from thiserror
*/

#![allow(unused)]

// mod expand;

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
	DeriveInput,
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

// #[proc_macro_derive(Nbt, attributes(
// 	encoder,
// 	decoder,
// 	preprocess,
// 	postprocess,
// 	remainder,
// 	default,
// 	name,
// ))]
// pub fn derive_nbt(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     expand::nbt_derive(&input)
//         .unwrap_or_else(|err| err.to_compile_error())
//         .into()
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

