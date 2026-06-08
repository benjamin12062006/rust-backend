pub mod features {
    pub mod product {
        pub mod application {
            #[allow(unused_imports)]
            pub use backend_application::features::product::application::*;
        }

        pub mod domain {
            #[allow(unused_imports)]
            pub use backend_domain::features::product::domain::*;
        }

        pub mod infrastructure {
            #[allow(unused_imports)]
            pub use backend_infrastructure::features::product::infrastructure::*;
        }

        pub mod ports {
            #[allow(unused_imports)]
            pub use backend_application::features::product::ports::*;
        }

        pub mod presentation {
            #[allow(unused_imports)]
            pub use backend_http::product_presentation::*;
        }
    }

    pub mod user {
        pub mod application {
            pub use backend_application::features::user::application::*;
        }

        pub mod domain {
            pub use backend_domain::features::user::domain::*;
        }

        pub mod infrastructure {
            pub use backend_infrastructure::features::user::infrastructure::*;
        }

        pub mod ports {
            pub use backend_application::features::user::ports::*;
        }

        pub mod presentation {
            pub mod http {
                pub use backend_http::features::user::presentation::http::*;
            }
        }
    }
}

pub mod shared {
    pub mod bootstrap {
        pub use backend_bootstrap::bootstrap::*;

        pub mod http_server {
            pub use backend_http::http_server::*;
        }

        pub mod middleware {
            pub use backend_http::middleware::*;
        }
    }

    pub mod common {
        pub mod error {
            pub use backend_common::common::error::*;
        }
    }

    pub mod configuration {
        pub use backend_config::configuration::*;
    }

    pub mod infrastructure {
        pub use backend_infrastructure::shared_infrastructure::*;
    }
}
