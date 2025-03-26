/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::{get, response::Redirect};

/// Redirects to the main page.
///
/// # Returns
/// A redirect to the main page.
#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("https://xodium.org")
}
