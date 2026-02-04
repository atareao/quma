use crate::constants::DEFAULT_LIMIT;
use crate::constants::DEFAULT_PAGE;

pub trait Paginable {
    fn page(&self) -> Option<u32>;
    fn limit(&self) -> Option<u32>;

    // Nuevos métodos que devuelven valores concretos
    fn page_or_default(&self) -> i64 {
        self.page().unwrap_or(DEFAULT_PAGE).into()
    }

    fn limit_or_default(&self) -> i64 {
        self.limit().unwrap_or(DEFAULT_LIMIT).into()
    }

    fn offset(&self) -> i64 {
        (self.page_or_default() - 1) * self.limit_or_default()
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
    fn test_paginable_defaults() {
        let params = TestParams { page: None, limit: None };
        assert_eq!(params.page_or_default(), DEFAULT_PAGE as i64);
        assert_eq!(params.limit_or_default(), DEFAULT_LIMIT as i64);
        assert_eq!(params.offset(), 0);
    }

    #[test]
    fn test_paginable_with_values() {
        let params = TestParams { page: Some(2), limit: Some(10) };
        assert_eq!(params.page_or_default(), 2);
        assert_eq!(params.limit_or_default(), 10);
        assert_eq!(params.offset(), 10);
    }
}

