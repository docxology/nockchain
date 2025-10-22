//! gRPC client functionality for Nockchain
//!
//! Provides clients for both public and private Nockchain APIs using tonic.

use nockapp_grpc::{
    pb::{
        common::v1::{Base58Hash, Hash, RawTransaction, Acknowledged},
        public::v1::{
            nockchain_service_client::NockchainServiceClient,
            WalletGetBalanceRequest, WalletGetBalanceResponse,
            WalletSendTransactionRequest, WalletSendTransactionResponse,
            TransactionAcceptedRequest, TransactionAcceptedResponse,
        },
    },
    public_nockchain::{self, PublicNockchainClient},
};
use std::time::Duration;
use tonic::{transport::Channel, Status};
use tracing::{info, error, warn};

/// gRPC client for Nockchain APIs
#[derive(Debug, Clone)]
pub struct GrpcClient {
    /// Public API client
    public_client: Option<NockchainServiceClient<Channel>>,
    /// Private API client (if available)
    private_client: Option<tonic::client::Grpc<Channel>>,
    /// Default timeout for requests
    timeout: Duration,
}

impl GrpcClient {
    /// Create a new gRPC client with the given endpoint
    pub fn new(endpoint: &str, timeout_secs: u64) -> Self {
        Self {
            public_client: None,
            private_client: None,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    /// Connect to the public API
    pub async fn connect_public(&mut self, endpoint: &str) -> Result<(), GrpcError> {
        info!("Connecting to public Nockchain API at: {}", endpoint);

        let channel = Channel::from_shared(endpoint.to_string())
            .map_err(|e| GrpcError::InvalidEndpoint(e.to_string()))?
            .connect_timeout(self.timeout)
            .timeout(self.timeout);

        let channel = channel
            .connect()
            .await
            .map_err(|e| GrpcError::ConnectionFailed(e.to_string()))?;

        self.public_client = Some(NockchainServiceClient::new(channel));
        Ok(())
    }

    /// Connect to the private API (if endpoint is local)
    pub async fn connect_private(&mut self, endpoint: &str) -> Result<(), GrpcError> {
        info!("Connecting to private Nockchain API at: {}", endpoint);

        let channel = Channel::from_shared(endpoint.to_string())
            .map_err(|e| GrpcError::InvalidEndpoint(e.to_string()))?
            .connect_timeout(self.timeout)
            .timeout(self.timeout);

        let channel = channel
            .connect()
            .await
            .map_err(|e| GrpcError::ConnectionFailed(e.to_string()))?;

        self.private_client = Some(tonic::client::Grpc::new(channel));
        Ok(())
    }

    /// Get wallet balance for an address
    pub async fn get_balance(&mut self, address: &str) -> Result<u64, GrpcError> {
        let client = self.public_client.as_mut()
            .ok_or_else(|| GrpcError::NotConnected("public API".to_string()))?;

        let request = WalletGetBalanceRequest {
            address: address.to_string(),
            page: None,
        };

        let response = client
            .wallet_get_balance(request)
            .await
            .map_err(|e| GrpcError::RequestFailed(e.to_string()))?;

        match response.into_inner().result {
            Some(wallet_get_balance_response::Result::Balance(balance_data)) => {
                // Sum up all note amounts
                let total_balance = balance_data.entries.iter()
                    .map(|entry| entry.amount)
                    .sum();
                Ok(total_balance)
            }
            Some(wallet_get_balance_response::Result::Error(error)) => {
                Err(GrpcError::ApiError(format!("API error: {}", error.message)))
            }
            None => Err(GrpcError::ApiError("No balance data returned".to_string())),
        }
    }

    /// Send a transaction
    pub async fn send_transaction(&mut self, tx: &crate::wallet::SignedTransaction) -> Result<String, GrpcError> {
        let client = self.public_client.as_mut()
            .ok_or_else(|| GrpcError::NotConnected("public API".to_string()))?;

        // Convert transaction to gRPC format
        let raw_tx = RawTransaction {
            // This would need proper conversion from our transaction format
            // For now, using placeholder
            payload: tx.signature.as_bytes().to_vec(),
        };

        let request = WalletSendTransactionRequest {
            tx_id: Some(Hash {
                data: tx.transaction.id.as_bytes().to_vec(),
            }),
            raw_tx: Some(raw_tx),
        };

        let response = client
            .wallet_send_transaction(request)
            .await
            .map_err(|e| GrpcError::RequestFailed(e.to_string()))?;

        match response.into_inner().result {
            Some(wallet_send_transaction_response::Result::Ack(_)) => {
                Ok(tx.transaction.id.clone())
            }
            Some(wallet_send_transaction_response::Result::Error(error)) => {
                Err(GrpcError::ApiError(format!("Send failed: {}", error.message)))
            }
            None => Err(GrpcError::ApiError("No response from send transaction".to_string())),
        }
    }

    /// Wait for transaction acceptance
    pub async fn wait_for_transaction(&mut self, tx_id: &str) -> Result<(), GrpcError> {
        let client = self.public_client.as_mut()
            .ok_or_else(|| GrpcError::NotConnected("public API".to_string()))?;

        let request = TransactionAcceptedRequest {
            tx_id: Some(Base58Hash {
                data: tx_id.as_bytes().to_vec(),
            }),
        };

        let response = client
            .transaction_accepted(request)
            .await
            .map_err(|e| GrpcError::RequestFailed(e.to_string()))?;

        match response.into_inner().result {
            Some(transaction_accepted_response::Result::Accepted(_)) => {
                info!("Transaction {} accepted", tx_id);
                Ok(())
            }
            Some(transaction_accepted_response::Result::Error(error)) => {
                Err(GrpcError::ApiError(format!("Transaction not accepted: {}", error.message)))
            }
            None => Err(GrpcError::ApiError("No response from transaction accepted".to_string())),
        }
    }
}

/// Errors that can occur during gRPC operations
#[derive(Debug, thiserror::Error)]
pub enum GrpcError {
    #[error("Not connected to {0} API")]
    NotConnected(String),

    #[error("Invalid endpoint: {0}")]
    InvalidEndpoint(String),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),

    #[error("Status error: {0}")]
    StatusError(#[from] Status),
}

/// Create a public API client with the default endpoint
///
/// # Arguments
/// * `endpoint` - Optional custom endpoint (defaults to public Zorp endpoint)
///
/// # Returns
/// A gRPC client connected to the public API
///
/// # Example
/// ```rust,no_run
/// use docxology::public_client;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut client = public_client(None).await?;
/// let balance = client.get_balance("your_address").await?;
/// println!("Balance: {}", balance);
/// # Ok(())
/// # }
/// ```
pub async fn public_client(endpoint: Option<&str>) -> Result<GrpcClient, GrpcError> {
    let endpoint = endpoint.unwrap_or("https://nockchain-api.zorp.io");
    let mut client = GrpcClient::new(endpoint, 30);
    client.connect_public(endpoint).await?;
    Ok(client)
}

/// Create a private API client (requires local node)
///
/// # Arguments
/// * `endpoint` - Local endpoint for the private API
///
/// # Returns
/// A gRPC client connected to the private API
///
/// # Example
/// ```rust,no_run
/// use docxology::private_client;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut client = private_client("http://127.0.0.1:8081").await?;
/// // Use private API methods
/// # Ok(())
/// # }
/// ```
pub async fn private_client(endpoint: &str) -> Result<GrpcClient, GrpcError> {
    let mut client = GrpcClient::new(endpoint, 30);
    client.connect_private(endpoint).await?;
    Ok(client)
}

impl Default for GrpcClient {
    fn default() -> Self {
        Self::new("https://nockchain-api.zorp.io", 30)
    }
}

/// Python wrapper for public_client function
#[cfg(feature = "python")]
#[pyo3::pyfunction]
pub fn public_client_py(endpoint: Option<&str>) -> pyo3::PyResult<()> {
    pyo3::Python::with_gil(|py| {
        pyo3::Py::from(0) // Placeholder - would need async runtime integration
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_client_creation() {
        let client = GrpcClient::new("https://test.endpoint", 30);
        assert_eq!(client.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_grpc_error_variants() {
        let not_connected = GrpcError::NotConnected("test".to_string());
        let invalid_endpoint = GrpcError::InvalidEndpoint("test".to_string());
        let connection_failed = GrpcError::ConnectionFailed("test".to_string());

        assert!(matches!(not_connected, GrpcError::NotConnected(_)));
        assert!(matches!(invalid_endpoint, GrpcError::InvalidEndpoint(_)));
        assert!(matches!(connection_failed, GrpcError::ConnectionFailed(_)));
    }
}
