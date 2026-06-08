pub mod product_infrastructure;
pub mod shared_infrastructure;
pub mod user_infrastructure;

pub mod features {
    pub mod product {
        pub mod infrastructure {
            #[allow(unused_imports)]
            pub use crate::product_infrastructure::*;
        }
    }

    pub mod user {
        pub mod infrastructure {
            pub use crate::user_infrastructure::*;
        }

        pub mod ports {
            pub use backend_application::features::user::ports::*;
        }
    }
}

pub mod shared {
    pub mod common {
        pub mod error {
            pub use backend_common::common::error::*;
        }
    }

    pub mod configuration {
        pub use backend_config::configuration::*;
    }

    pub mod infrastructure {
        pub use crate::shared_infrastructure::*;
    }
}
