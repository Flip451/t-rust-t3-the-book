extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // syn を用いて、TokenStream からの input をデータ構造に変換して、解釈・操作可能にする
    // 出力は DeriveInput 構造体（パースされた Rust コードを表す）
    let ast = syn::parse(input).unwrap();

    // トレイトを実装する処理
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // quote! マクロ内で #hoge と書くと、その部分は hoge という変数の値と置き換えられる
    // stringify! マクロは。1 + 2 などのような Rust の式を取り、コンパイル時に "1 + 2" のような文字列リテラルにその式を変換する
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()

}
