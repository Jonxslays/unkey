#![allow(clippy::module_name_repetitions)]

use serde::Deserialize;
use serde::Serialize;

/// An update operation that can be performed.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RefillInterval {
    /// Refill daily.
    Daily,

    /// Refill monthly.
    Monthly,
}

/// The state of a keys automatic refills.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Refill {
    /// The number of verifications to refill.
    pub amount: usize,

    /// The interval at which to refill the verifications.
    pub interval: RefillInterval,

    /// The UNIX timestamp in milliseconds indicating when the key was last
    /// refilled, if it has been.
    #[serde(skip_serializing)]
    pub last_refilled_at: Option<usize>,
}

impl Refill {
    /// Creates a new refill.
    ///
    /// # Arguments
    /// - `amount`: The number of verifications to refill.
    /// - `interval`: The interval at which to refill the verifications.
    ///
    /// # Returns
    /// The refill struct.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::Refill;
    /// # use unkey::models::RefillInterval;
    /// let r = Refill::new(100, RefillInterval::Daily);
    ///
    /// assert_eq!(r.amount, 100);
    /// assert_eq!(r.interval, RefillInterval::Daily);
    /// ```
    #[must_use]
    #[rustfmt::skip]
    pub fn new(amount: usize, interval: RefillInterval) -> Self {
        Self { amount, interval, last_refilled_at: None }
    }
}
