pub mod product;
pub mod user;

pub mod features {
    pub mod product {
        pub mod domain {
            #[allow(unused_imports)]
            pub use crate::product::*;
        }
    }

    pub mod user {
        pub mod domain {
            pub use crate::user::*;
        }
    }
}
