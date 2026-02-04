use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Fields, ItemStruct};

pub fn expand_axum_crud(attr: TokenStream1, input: ItemStruct) -> TokenStream {
    // 1. Identidad del struct principal (ej: Unit, User, Task)
    let name = &input.ident;

    // 2. Validación: Solo structs con campos
    if let Fields::Unit = input.fields {
        return Error::new_spanned(
            &input,
            "#[axum_crud] solo puede usarse en structs con campos (named fields).",
        )
        .to_compile_error();
    }

    // 3. Parseo de atributos configurables
    // Espera: #[axum_crud(path = "/units", new = "NewItem", params = "Params")]
    let attr_str = attr.to_string();

    let route_path = extract_attr(&attr_str, "path").unwrap_or_else(|| "/".into());
    let new_type_name = extract_attr(&attr_str, "new").unwrap_or_else(|| "NewItem".into());
    let params_type_name = extract_attr(&attr_str, "params").unwrap_or_else(|| "Params".into());

    // Convertimos strings en identificadores reales de Rust
    let new_item_ident = format_ident!("{}", new_type_name);
    let params_ident = format_ident!("{}", params_type_name);

    // 4. Generación de código
    quote! {
        #input

        // Verificación de tipos en tiempo de compilación para evitar errores crípticos
        const _: () = {
            type _ValidateNew = #new_item_ident;
            type _ValidateParams = #params_ident;
            type _ValidateState = crate::models::AppState;
        };

        // El router se asocia al struct para mantener el orden
        impl #name {
            pub fn router() -> axum::Router<std::sync::Arc<crate::models::AppState>> {
                axum::Router::new()
                    .route("/", axum::routing::post(create))
                    .route("/", axum::routing::patch(update))
                    .route("/", axum::routing::get(read))
                    .route("/", axum::routing::delete(delete))
            }
        }

        // --- HANDLERS GENERADOS ---

        pub async fn create(
            axum::extract::State(app_state): axum::extract::State<std::sync::Arc<crate::models::AppState>>,
            axum::Json(payload): axum::Json<#new_item_ident>,
        ) -> impl axum::response::IntoResponse {
            tracing::debug!("Creando {}: {:?}", stringify!(#name), payload);
            match #name::create(&app_state.pool, payload).await {
                Ok(item) => crate::models::ApiResponse::new(
                    axum::http::StatusCode::CREATED,
                    &format!("{} creado con éxito", stringify!(#name)),
                    crate::models::Data::Some(serde_json::to_value(item).unwrap()),
                ),
                Err(e) => {
                    tracing::error!("Error en create {}: {:?}", stringify!(#name), e);
                    crate::models::ApiResponse::new(axum::http::StatusCode::BAD_REQUEST, &e.to_string(), crate::models::Data::None)
                }
            }
        }

        pub async fn update(
            axum::extract::State(app_state): axum::extract::State<std::sync::Arc<crate::models::AppState>>,
            axum::Json(payload): axum::Json<#name>,
        ) -> impl axum::response::IntoResponse {
            match #name::update(&app_state.pool, payload).await {
                Ok(updated) => crate::models::ApiResponse::new(
                    axum::http::StatusCode::OK,
                    &format!("{} actualizado", stringify!(#name)),
                    crate::models::Data::Some(serde_json::to_value(updated).unwrap()),
                ),
                Err(e) => crate::models::ApiResponse::new(axum::http::StatusCode::BAD_REQUEST, &e.to_string(), crate::models::Data::None),
            }
        }

        pub async fn read(
            axum::extract::State(app_state): axum::extract::State<std::sync::Arc<crate::models::AppState>>,
            axum::extract::Query(params): axum::extract::Query<#params_ident>,
        ) -> impl axum::response::IntoResponse {
            // 1. Búsqueda por ID
            if let Some(id) = params.id {
                return match #name::read_by_id(&app_state.pool, id).await {
                    Ok(Some(item)) => crate::models::CustomResponse::api(axum::http::StatusCode::OK, "Encontrado", crate::models::Data::Some(serde_json::to_value(item).unwrap())),
                    Ok(None) => crate::models::CustomResponse::api(axum::http::StatusCode::NOT_FOUND, "No encontrado", crate::models::Data::None),
                    Err(e) => crate::models::CustomResponse::api(axum::http::StatusCode::BAD_REQUEST, &e.to_string(), crate::models::Data::None),
                };
            }

            // 2. Intento de lectura paginada
            if params.page.is_some() {
            let records_res = #name::read_paged(&app_state.pool, &params).await;
            let count_res = #name::count_paged(&app_state.pool, &params).await;

            if let (Ok(records), Ok(count)) = (records_res, count_res) {
                let pagination = crate::models::Pagination::new(&params, count, #route_path);
                return crate::models::CustomResponse::paged(
                    axum::http::StatusCode::OK,
                    "Resultados paginados",
                    crate::models::Data::Some(serde_json::to_value(records).unwrap()),
                    pagination
                );
            }
            }

            // 3. Fallback: Todos
            match #name::read_all(&app_state.pool).await {
                Ok(items) => crate::models::CustomResponse::api(axum::http::StatusCode::OK, "Lista completa", crate::models::Data::Some(serde_json::to_value(items).unwrap())),
                Err(e) => crate::models::CustomResponse::api(axum::http::StatusCode::BAD_REQUEST, &e.to_string(), crate::models::Data::None),
            }
        }

        pub async fn delete(
            axum::extract::State(app_state): axum::extract::State<std::sync::Arc<crate::models::AppState>>,
            axum::extract::Query(params): axum::extract::Query<#params_ident>,
        ) -> impl axum::response::IntoResponse {
            let Some(id) = params.id else {
                return crate::models::ApiResponse::new(axum::http::StatusCode::BAD_REQUEST, "ID requerido", crate::models::Data::None);
            };

            match #name::delete(&app_state.pool, id).await {
                Ok(item) => crate::models::ApiResponse::new(
                    axum::http::StatusCode::OK,
                    "Eliminado",
                    crate::models::Data::Some(serde_json::to_value(item).unwrap())
                ),
                Err(e) => crate::models::ApiResponse::new(axum::http::StatusCode::BAD_REQUEST, &e.to_string(), crate::models::Data::None),
            }
        }
    }
}

/// Extrae valores de atributos tipo llave="valor"
fn extract_attr(attr: &str, key: &str) -> Option<String> {
    attr.split(',')
        .find(|s| s.contains(key))
        .and_then(|s| s.split('=').next_back()) // <--- Cambiado .last() por .next_back()
        .map(|s| {
            s.trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == '(' || c == ')')
                .to_string()
        })
}

