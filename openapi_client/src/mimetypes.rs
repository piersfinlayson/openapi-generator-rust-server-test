/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for AGet
    lazy_static! {
        pub static ref A_GET_OK: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for AGet
    lazy_static! {
        pub static ref A_GET_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for APost
    lazy_static! {
        pub static ref A_POST_OK: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for APost
    lazy_static! {
        pub static ref A_POST_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

}
