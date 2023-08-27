#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

/// A snapshot of the ratelimit status for a key.
#[derive(Debug, Clone, Deserialize)]
pub struct RatelimitState {
    /// The number of burstable requests allowed.
    pub limit: usize,

    /// The remaining requests in this burst window.
    pub remaining: usize,

    /// The unix timestamp in ms when the next window starts.
    pub reset: usize,
}

/// Different rate limit types implemented by unkey.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RatelimitType {
    /// Quick because each edge location maintains its own ratelimit,
    /// meaning users can theoretically exceed the ratelimit if
    /// their requests go through different locations.
    Fast,

    /// All ratelimit operations go through a single service,
    /// meaning consistent ratelimits.
    Consistent,
}

/// A ratelimit imposed on an api key.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ratelimit {
    /// The type for this ratelimit.
    #[serde(rename = "type")]
    pub ratelimit_type: RatelimitType,

    /// The rate at which the ratelimit refills, per interval.
    pub refill_rate: usize,

    /// The interval at which to refill, in milliseconds.
    pub refill_interval: usize,

    /// Total number of burstable requests.
    pub limit: usize,
}

impl Ratelimit {
    /// Creates a new ratelimit.
    ///
    /// # Arguments
    /// - `ratelimit_type`: The type for this ratelimit.
    /// - `refill_rate`: The rate at which the ratelimit refills, per interval.
    /// - `refill_interval`: The interval at which to refill, in milliseconds.
    /// - `limit`: Total number of burstable requests.
    ///
    /// # Returns
    /// The new ratelimit.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::Ratelimit;
    /// # use unkey::models::RatelimitType;
    /// let r = Ratelimit::new(
    ///     RatelimitType::Fast,
    ///     10,
    ///     10000,
    ///     100,
    /// );
    ///
    /// assert_eq!(r.ratelimit_type, RatelimitType::Fast);
    /// assert_eq!(r.refill_rate, 10);
    /// assert_eq!(r.refill_interval, 10000);
    /// assert_eq!(r.limit, 100);
    /// ```
    #[must_use]
    pub fn new(
        ratelimit_type: RatelimitType,
        refill_rate: usize,
        refill_interval: usize,
        limit: usize,
    ) -> Self {
        Self {
            ratelimit_type,
            refill_rate,
            refill_interval,
            limit,
        }
    }
}
