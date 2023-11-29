use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    /// The number of items to skip before starting to collect the result set
    pub offset: Option<usize>,

    /// The numbers of items to return
    pub limit: Option<usize>,
}

const DEFAULT_PAGE_SIZE: usize = 25;

impl Default for PageRequest {
    fn default() -> Self {
        PageRequest {
            offset: Some(0),
            limit: Some(DEFAULT_PAGE_SIZE),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod page_requests {
        use super::*;

        #[test]
        fn it_should_implement_default_for_page_requests() {
            let page_request = PageRequest::default();
            assert_eq!(Some(0), page_request.offset);
            assert_eq!(Some(25), page_request.limit);
        }
    }
}
