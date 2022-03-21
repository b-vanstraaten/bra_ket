use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    println!("_item: {}", _item.to_string());
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
