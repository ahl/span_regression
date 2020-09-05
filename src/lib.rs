#![allow(dead_code)]

#[proc_macro_attribute]
pub fn identity(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_attribute]
pub fn nested(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut ret = proc_macro::TokenStream::new();
    let pre = vec![
        proc_macro::TokenTree::Ident(proc_macro::Ident::new("fn", proc_macro::Span::call_site())),
        proc_macro::TokenTree::Ident(proc_macro::Ident::new("foo", proc_macro::Span::call_site())),
        proc_macro::TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Parenthesis,
            proc_macro::TokenStream::new(),
        )),
        proc_macro::TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Brace, item)),
    ];
    ret.extend(pre);
    ret
}
