/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket_governor::{Method, Quota, RocketGovernable};

/// RateLimit Guard.
pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_second(Self::nonzero(1u32))
    }
}
