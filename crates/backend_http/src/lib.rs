pub mod http_server;
pub mod middleware;
pub mod product_presentation;
pub mod user_http;

pub mod features {
    pub mod user {
        pub mod application {
            pub use backend_application::features::user::application::*;
        }

        pub mod ports {
            pub use backend_application::features::user::ports::*;
        }

        pub mod presentation {
            pub mod http {
                pub use crate::user_http::*;
            }
        }
    }
}

pub mod shared {
    pub mod bootstrap {
        pub mod features {
            pub use crate::http_server::Services;
            pub use crate::http_server::Services as ApplicationServices;
        }

        pub mod middleware {
            pub use crate::middleware::*;
        }
    }

    pub mod common {
        pub mod error {
            pub use backend_common::common::error::*;
        }
    }
}
