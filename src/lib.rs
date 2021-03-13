extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn expr(_: TokenStream) -> TokenStream {
    "1 + 1".parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
