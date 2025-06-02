/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use rocket_governor::{Method, Quota, RocketGovernable};

/// RateLimit Guard.
pub struct RateLimitGuard;

impl RocketGovernable<'_> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_second(Self::nonzero(1u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket_governor::{Method, Quota};
    use std::num::NonZeroU32;

    #[test]
    fn test_rate_limit_quota_get() {
        let quota = RateLimitGuard::quota(Method::Get, "test_route");
        let expected = Quota::per_second(NonZeroU32::new(1).unwrap());

        assert_eq!(quota, expected);
    }

    #[test]
    fn test_rate_limit_quota_different_methods() {
        let get_quota = RateLimitGuard::quota(Method::Get, "test_route");
        let post_quota = RateLimitGuard::quota(Method::Post, "test_route");
        let put_quota = RateLimitGuard::quota(Method::Put, "test_route");

        assert_eq!(get_quota, post_quota);
        assert_eq!(post_quota, put_quota);
    }

    #[test]
    fn test_rate_limit_quota_different_routes() {
        let route1_quota = RateLimitGuard::quota(Method::Get, "route1");
        let route2_quota = RateLimitGuard::quota(Method::Get, "route2");

        assert_eq!(route1_quota, route2_quota);
    }

    #[test]
    fn test_nonzero_conversion() {
        assert_eq!(RateLimitGuard::nonzero(5u32), NonZeroU32::new(5).unwrap());
        assert_eq!(RateLimitGuard::nonzero(1u32), NonZeroU32::new(1).unwrap());
    }
}
