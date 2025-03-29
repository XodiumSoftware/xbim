/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::Status,
    Data, Request,
};
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Mutex,
    time::{Duration, Instant},
};

pub struct RateLimitingFairing {
    clients: Mutex<HashMap<IpAddr, Vec<Instant>>>,
    rate: usize,
    per_seconds: u64,
}

impl RateLimitingFairing {
    pub fn new(rate: usize, per_seconds: u64) -> Self {
        RateLimitingFairing {
            clients: Mutex::new(HashMap::new()),
            rate,
            per_seconds,
        }
    }

    fn is_rate_limited(&self, ip: IpAddr) -> bool {
        let mut clients = self.clients.lock().unwrap();
        let now = Instant::now();
        let window_start = now - Duration::from_secs(self.per_seconds);
        let timestamps = clients.entry(ip).or_default();

        timestamps.retain(|&t| t >= window_start);

        if timestamps.len() >= self.rate {
            return true;
        }

        timestamps.push(now);
        false
    }
}

#[async_trait]
impl Fairing for RateLimitingFairing {
    fn info(&self) -> Info {
        Info {
            name: "Rate Limiter",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        if let Some(client_ip) = req.client_ip() {
            if self.is_rate_limited(client_ip) {
                req.set_error(Status::TooManyRequests);
            }
        }
    }
}
