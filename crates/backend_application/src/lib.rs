pub mod product_application;
pub mod product_ports;
pub mod user_application;
pub mod user_ports;

pub mod features {
    pub mod product {
        pub mod application {
            #[allow(unused_imports)]
            pub use crate::product_application::*;
        }

        pub mod ports {
            #[allow(unused_imports)]
            pub use crate::product_ports::*;
        }
    }

    pub mod user {
        pub mod application {
            pub use crate::user_application::*;
        }

        pub mod domain {
            pub use backend_domain::features::user::domain::*;
        }

        pub mod ports {
            pub use crate::user_ports::*;
        }
    }
}

pub mod shared {
    pub mod common {
        pub mod error {
            pub use backend_common::common::error::*;
        }
    }
}
