use std::future::Future;

use candid::CandidType;
use candid::utils::ArgumentEncoder;
use serde::de::DeserializeOwned;

use crate::CanisterClientResult;

/// Generic client for interacting with a canister.
/// This is used to abstract away the differences between the IC Agent and the
/// IC Canister.
/// The IC Agent is used for interaction through the dfx tool, while the IC
/// Canister is used for interacting with the EVM canister in wasm environments.
pub trait CanisterClient: Send + Clone {
    /// Call an update method on the canister.
    ///
    /// # Arguments
    ///
    /// * `method` - The method name.
    /// * `args` - The arguments to the method.
    ///
    /// # Returns
    ///
    /// The result of the method call.
    fn update<T, R>(
        &self,
        method: &str,
        args: T,
    ) -> impl Future<Output = CanisterClientResult<R>> + Send
    where
        T: ArgumentEncoder + Send + Sync,
        R: DeserializeOwned + CandidType + Send;

    /// Call a query method on the canister.
    ///
    /// # Arguments
    ///
    /// * `method` - The method name.
    /// * `args` - The arguments to the method.
    ///
    /// # Returns
    ///
    /// The result of the method call.
    fn query<T, R>(
        &self,
        method: &str,
        args: T,
    ) -> impl Future<Output = CanisterClientResult<R>> + Send
    where
        T: ArgumentEncoder + Send + Sync,
        R: DeserializeOwned + CandidType + Send;

    /// Submit an asynchronous call to the canister.
    ///
    /// This method is only available when the `pocket-ic` feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `method` - The method name.
    /// * `args` - The arguments to the method.
    ///
    /// # Returns
    ///
    /// A message ID that can be used to await the result.
    #[cfg(feature = "pocket-ic")]
    fn submit_call<T>(
        &self,
        method: &str,
        args: T,
    ) -> impl Future<Output = CanisterClientResult<pocket_ic::common::rest::RawMessageId>> + Send
    where
        T: ArgumentEncoder + Send + Sync,
    {
        async move {
            let _ = (method, args);
            Err(crate::CanisterClientError::CandidError(candid::Error::msg(
                "submit_call not supported by this client implementation",
            )))
        }
    }

    /// Await the result of a previously submitted call.
    ///
    /// This method is only available when the `pocket-ic` feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `msg_id` - The message ID returned by submit_call.
    ///
    /// # Returns
    ///
    /// The result of the method call.
    #[cfg(feature = "pocket-ic")]
    fn await_call<R>(
        &self,
        msg_id: pocket_ic::common::rest::RawMessageId,
    ) -> impl Future<Output = CanisterClientResult<R>> + Send
    where
        R: DeserializeOwned + CandidType + Send,
    {
        async move {
            let _ = msg_id;
            Err(crate::CanisterClientError::CandidError(candid::Error::msg(
                "await_call not supported by this client implementation",
            )))
        }
    }
}
