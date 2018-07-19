//! Contains all of the API methods in the respective submodules.
//!
//! ## Note about naming
//! Rust prefers `snake_case` in the function names instead of `camelCase` used by the VK API,
//! which means all of the API method's corresponding functions are named using `snake_case`.
//!
//! **Example:** To call the `appWidgets.getAppImageUploadServer` API method, use the `rvk::methods::app_widgets::get_app_image_upload_server` function.
//!
//! ## Note: `execute`
//! The `execute` method has no category, so it is located in `rvk::methods::execute`.
//!
//! ## Note: `photos.move`
//! Since `move` is a Rust keyword, the function for calling `photos.move` API method is `rvk::methods::photos::move_` (**with the underscore!**)

/// Defines an API method category
macro_rules! api_category {
    ($name:expr) => {
        use heck::MixedCase;
        const CATEGORY: &str = $name;
    };
}

/// A macro which creates a default function that calls a specified VK API method
macro_rules! api_method {
    ($name:ident) => {
        api_method!(
            $name,
            &(CATEGORY.to_owned() + "." + &stringify!($name).to_mixed_case())
        );
    };
    ($func_name:ident, $method_name:expr) => {
        /// [generated] Calls the corresponding VK API method
        ///
        /// Generated by the `api_method!` macro.
        pub fn $func_name(
            api: &::api::APIClient,
            params: ::Params,
        ) -> ::error::Result<::serde_json::Value> {
            let method_name = $method_name;
            api.call_method(method_name, params)
        }
    };
}

api_method!(execute, "execute");

pub mod account;
pub mod ads;
pub mod app_widgets;
pub mod apps;
pub mod auth;
pub mod board;
pub mod database;
pub mod docs;
pub mod fave;
pub mod friends;
pub mod gifts;
pub mod groups;
pub mod leads;
pub mod likes;
pub mod market;
pub mod messages;
pub mod newsfeed;
pub mod notes;
pub mod notifications;
pub mod orders;
pub mod pages;
pub mod photos;
pub mod places;
pub mod polls;
pub mod search;
pub mod secure;
pub mod stats;
pub mod status;
pub mod storage;
pub mod stories;
pub mod streaming;
pub mod users;
pub mod utils;
pub mod video;
pub mod wall;
pub mod widgets;
