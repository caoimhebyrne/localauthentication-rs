//! A `LocalAuthentication.framework` binding library for Rust.
//!
//! `localauthentication-rs` provides a wrapper for [LAContext](https://developer.apple.com/documentation/localauthentication/lacontext) in Apple's [LocalAuthentication framework](https://developer.apple.com/documentation/localauthentication).
//!
//! This crate can be used to verify a user's identity in macOS (or iOS), being able to choose how via the [`LAPolicy`] enum.
//!
//! In the example below, we attempt to authenticate with the user via their biometrics (Touch ID or Face ID), Apple Watch, or in the "worst case scenario", their account password.
//! ```no_run
//! use localauthentication_rs::{LAPolicy, LocalAuthentication};
//!
//! fn main() {
//!     // Create a new instance of LocalAuthentication
//!     let local_authentication = LocalAuthentication::new();
//!
//!     // Try to authenticate the user
//!     let authenticated = local_authentication.evaluate_policy(
//!         LAPolicy::DeviceOwnerAuthenticationWithBiometrics,
//!         "authenticate your user",
//!     );
//!
//!     // Print the result
//!     if authenticated {
//!         println!("Welcome!");
//!     } else {
//!         println!("Not authenticated...");
//!     }
//! }
//! ```

use external::{lacontext_canEvaluatePolicy, lacontext_evaluatePolicy, lacontext_new, LAContext};
use swift_rs::{Int, SRObject, SRString};

mod external;

/// A wrapper for LocalAuthentication.framework's LAContext
pub struct LocalAuthentication {
    context: SRObject<LAContext>,
}

impl LocalAuthentication {
    /// Creates a new LocalAuthentication instance.
    pub fn new() -> Self {
        let context = unsafe { lacontext_new() };
        Self { context }
    }

    /// Checks if a [`LAPolicy`] can be evaluated.
    ///
    /// This will return `true` if it can be evaluated, or `false` if it cannot.
    ///
    /// # Arguments
    /// * `policy` - The policy to evaluate
    ///
    /// **Apple Developer Documentation**: <https://developer.apple.com/documentation/localauthentication/lacontext/1514149-canevaluatepolicy>
    ///
    /// # Examples
    /// ```
    /// use localauthentication_rs::{LAPolicy, LocalAuthentication};
    ///
    /// fn main() {
    ///     // Create a new instance of LocalAuthentication
    ///     let local_authentication = LocalAuthentication::new();
    ///
    ///     // See if we can authenticate via biometrics
    ///     let can_use_biometrics = local_authentication.can_evaluate_policy(LAPolicy::DeviceOwnerAuthenticationWithBiometrics);
    ///
    ///     // Print the result
    ///     if can_use_biometrics {
    ///         println!("Authenticating via biometrics!");
    ///     } else {
    ///         println!("Falling back to...");
    ///     }
    /// }
    /// ```
    pub fn can_evaluate_policy(&self, policy: LAPolicy) -> bool {
        let value = unsafe { lacontext_canEvaluatePolicy(&self.context, policy.into()) };
        return value.into();
    }

    /// Evaluates a [`LAPolicy`].
    ///
    /// This may prompt the user to enter their passcode, use biometrics, or authenticate with Apple Watch, depending on the [policy](LAPolicy).
    ///
    /// Before calling this, you should check if it can be evaluated with [`Self::can_evaluate_policy`]
    ///
    /// # Arguments
    /// * `policy` - The policy to evaluate
    /// * `reason` - The reason shown to the user as to why you are trying to authenticate. Will be formatted as follows: `[binary] is trying to {reason}`
    ///
    /// **Apple Developer Documentation**: <https://developer.apple.com/documentation/localauthentication/lacontext/1514176-evaluatepolicy>
    ///
    /// **Note**: This blocks the thread that this is executed on until it has either failed or succeeded!
    ///
    /// # Examples
    /// ```
    /// use localauthentication_rs::{LAPolicy, LocalAuthentication};
    ///
    /// fn main() {
    ///     // Create a new instance of LocalAuthentication
    ///     let local_authentication = LocalAuthentication::new();
    ///
    ///     // Try to authenticate the user
    ///     let success = local_authentication.evaluate_policy(
    ///         LAPolicy::DeviceOwnerAuthenticationWithBiometrics,
    ///         "authenticate your user",
    ///     );
    ///
    ///     // Print the result
    ///     if success {
    ///         println!("Welcome!");
    ///     } else {
    ///         println!("Not authenticated...");
    ///     }
    /// }
    /// ```
    pub fn evaluate_policy(&self, policy: LAPolicy, reason: &str) -> bool {
        let string: SRString = reason.into();
        return unsafe { lacontext_evaluatePolicy(&self.context, policy.into(), &string) }.into();
    }
}

/// The set of available local authentication policies.
pub enum LAPolicy {
    /// 1. User authentication with biometry.
    DeviceOwnerAuthenticationWithBiometrics,

    /// 2. User authentication with biometry, Apple Watch, or the device passcode.
    DeviceOwnerAuthentication,

    /// 3. User authentication with Apple Watch.
    DeviceOwnerAuthenticationWithWatch,

    /// 4. User authentication with either biometry or Apple Watch.
    DeviceOwnerAuthenticationWithBiometricsOrWatch,

    /// -1. This is not an implemented result, and you should also never receive it..
    Unknown,
}

impl From<Int> for LAPolicy {
    fn from(value: Int) -> Self {
        match value {
            1 => Self::DeviceOwnerAuthenticationWithBiometrics,
            2 => Self::DeviceOwnerAuthentication,
            3 => Self::DeviceOwnerAuthenticationWithWatch,
            4 => Self::DeviceOwnerAuthenticationWithBiometricsOrWatch,
            _ => Self::Unknown,
        }
    }
}

impl From<LAPolicy> for Int {
    fn from(value: LAPolicy) -> Self {
        match value {
            LAPolicy::DeviceOwnerAuthenticationWithBiometrics => 1,
            LAPolicy::DeviceOwnerAuthentication => 2,
            LAPolicy::DeviceOwnerAuthenticationWithWatch => 3,
            LAPolicy::DeviceOwnerAuthenticationWithBiometricsOrWatch => 4,
            _ => -1,
        }
    }
}
