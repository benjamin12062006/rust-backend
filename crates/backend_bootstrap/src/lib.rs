pub mod bootstrap;

pub mod features {
    pub mod user {
        pub mod application {
            pub use backend_application::features::user::application::*;
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
        pub use crate::bootstrap::*;
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
