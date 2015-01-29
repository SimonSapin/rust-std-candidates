#![feature(plugin_registrar, quote, rustc_private)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{TokenTree, Ident};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacItems, IdentTT, get_single_str_from_tts};
use rustc::plugin::Registry;

fn expand_mod_path<'a>(cx: &'a mut ExtCtxt, sp: Span, ident: Ident, tts: Vec<TokenTree>)
            -> Box<MacResult + 'a> {
    let path = match get_single_str_from_tts(cx, sp, &*tts, "mod_path!") {
        Some(string) => string,
        None => return DummyResult::expr(sp),
    };
    let path = &*path;

    MacItems::new(vec![quote_item!(cx,

        #[path = $path]
        pub mod $ident;

    ).unwrap()].into_iter())
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("mod_path"), IdentTT(Box::new(expand_mod_path), None));
}
