#![allow(clippy::too_many_arguments)]

pub mod drivers;
mod error;

pub mod handlers {
    use crate::drivers;
    use rocket::Route;

    pub fn handlers() -> Vec<Route> {
        drivers::handlers()
    }
}
