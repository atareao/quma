use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

// Importamos los módulos donde pondremos la lógica
mod crud;
mod pagination;

// --- Macro de Atributo: #[axum_crud] ---
#[proc_macro_attribute]
pub fn axum_crud(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    // Pasamos 'attr' para poder leer la ruta si la pones: #[axum_crud("/units")]
    crud::expand_axum_crud(attr, input).into()
}

// --- Macro de Derive: #[derive(Paginable)] ---
#[proc_macro_derive(Paginable)]
pub fn paginable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Pasamos la lógica a pagination_logic
    pagination::expand_paginable(input).into()
}

