use axum::{
    http::{
        StatusCode,
        HeaderMap,
    },
    Json,
    body::Body,
    response::{
        Response,
        IntoResponse,
    }
};
use serde::{Deserialize, Serialize};
use super::paginable::Paginable;
use serde_json::Value;

use crate::constants::DEFAULT_LIMIT;
use crate::constants::DEFAULT_PAGE;

#[derive(Debug, Clone)]
pub enum CustomResponse {
    Api(ApiResponse),
    Empty(EmptyResponse),
    Paged(PagedResponse),
}

impl CustomResponse {
    pub fn api(status: StatusCode, message: &str, data: Option<Value>) -> Self {
        CustomResponse::Api(ApiResponse::new(status, message, data))
    }
    pub fn paged(status: StatusCode, message: &str, data: Option<Value>, pagination: Pagination) -> Self {
        CustomResponse::Paged(PagedResponse::new(status, message, data, pagination))
    }
    pub fn empty(status: StatusCode, message: &str) -> Self {
        CustomResponse::Empty(EmptyResponse {
            status,
            message: message.to_string(),
        })
    }
}


pub type PdfResponse = (HeaderMap, Vec<u8>);


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<Value>,
}

impl ApiResponse {
    pub fn new(status: StatusCode, message: &str, data: Option<Value>) -> Self {
        Self {
            status: status.as_u16(),
            message: message.to_string(),
            data,
        }
    }
}

impl From<ApiResponse> for CustomResponse {
    fn from(api_response: ApiResponse) -> Self {
        CustomResponse::Api(api_response)
    }
}

impl From<EmptyResponse> for CustomResponse {
    fn from(empty_response: EmptyResponse) -> Self {
        CustomResponse::Empty(empty_response)
    }
}

impl From<PagedResponse> for CustomResponse {
    fn from(paged_response: PagedResponse) -> Self {
        CustomResponse::Paged(paged_response)
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

impl IntoResponse for CustomResponse {
    fn into_response(self) -> Response {
        match self {
            CustomResponse::Api(api_response) => api_response.into_response(),
            CustomResponse::Empty(empty_response) => empty_response.into_response(),
            CustomResponse::Paged(page_response) => page_response.into_response(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub pages: u32,
    pub records: i64,
    pub prev: Option<String>, // previous page
    pub next: Option<String>, // next page
}

impl Pagination {
    pub fn new(params: &impl Paginable, count: i64, base_path: &str) -> Self {
        let limit = params.limit().unwrap_or(DEFAULT_LIMIT);
        let page = params.page().unwrap_or(DEFAULT_PAGE);
        let total_pages = (count as f32 / limit as f32).ceil() as u32;

        Self {
            page,
            limit,
            pages: total_pages,
            records: count,
            prev: if page > 1 {
                Some(format!("{}?page={}&limit={}", base_path, page - 1, limit))
            } else {
                None
            },
            next: if page < total_pages {
                Some(format!("{}?page={}&limit={}", base_path, page + 1, limit))
            } else {
                None
            },
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct PagedResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<Value>,
    pub pagination: Pagination,
}

impl PagedResponse {
    pub fn new(status: StatusCode, message: &str, data: Option<Value>, pagination: Pagination) -> Self {
        Self {
            status: status.as_u16(),
            message: message.to_string(),
            data,
            pagination,
        }
    }
}

impl IntoResponse for PagedResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Clone)]
pub struct EmptyResponse {
    pub status: StatusCode,
    pub message: String,
}
impl EmptyResponse {
    pub fn create(status: StatusCode, message: &str) -> Response<Body> {
        Response::builder()
            .status(status)
            .body(Body::from(message.to_string())) // Cuerpo de la respuesta
            .unwrap()
    }
}

impl IntoResponse for EmptyResponse {
    fn into_response(self) -> Response {
        EmptyResponse::create(self.status, self.message.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestParams {
        page: Option<u32>,
        limit: Option<u32>,
    }

    impl Paginable for TestParams {
        fn page(&self) -> Option<u32> {
            self.page
        }

        fn limit(&self) -> Option<u32> {
            self.limit
        }
    }

    #[test]
    fn test_pagination_new() {
        let params = TestParams { page: Some(2), limit: Some(10) };
        let pagination = Pagination::new(&params, 100, "/test");
        assert_eq!(pagination.page, 2);
        assert_eq!(pagination.limit, 10);
        assert_eq!(pagination.pages, 10);
        assert_eq!(pagination.records, 100);
        assert_eq!(pagination.prev, Some("/test?page=1&limit=10".to_string()));
        assert_eq!(pagination.next, Some("/test?page=3&limit=10".to_string()));
    }

    #[test]
    fn test_pagination_first_page() {
        let params = TestParams { page: Some(1), limit: Some(10) };
        let pagination = Pagination::new(&params, 100, "/test");
        assert_eq!(pagination.prev, None);
        assert_eq!(pagination.next, Some("/test?page=2&limit=10".to_string()));
    }

    #[test]
    fn test_pagination_last_page() {
        let params = TestParams { page: Some(10), limit: Some(10) };
        let pagination = Pagination::new(&params, 100, "/test");
        assert_eq!(pagination.prev, Some("/test?page=9&limit=10".to_string()));
        assert_eq!(pagination.next, None);
    }
}


