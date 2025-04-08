/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::get;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

/// Index route handler.
#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "xBIM - BIM Models Solution",
            message: "Your all-in-one solution for BIM models"
        },
    )
}
