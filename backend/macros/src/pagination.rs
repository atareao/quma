use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand_paginable(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    quote! {
        impl #name {
            pub fn limit_sql(&self, default: u32) -> i64 {
                self.limit.unwrap_or(default) as i64
            }

            pub fn offset_sql(&self, default_page: u32, default_limit: u32) -> i64 {
                let p = self.page.unwrap_or(default_page);
                let l = self.limit.unwrap_or(default_limit);
                if p > 0 { ((p - 1) * l) as i64 } else { 0 }
            }

            pub fn is_paged(&self) -> bool {
                self.page.is_some() || self.limit.is_some()
            }
        }

        impl Paginable for #name {
            fn page(&self) -> Option<u32> { self.page }
            fn limit(&self) -> Option<u32> { self.limit }
        }
    }
}

