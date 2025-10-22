//! Unit tests for gRPC module

use docxology::grpc::{GrpcClient, GrpcError};
use std::time::Duration;

#[test]
fn test_grpc_client_creation() {
    let client = GrpcClient::new("https://test.endpoint", 30);

    assert_eq!(client.timeout, Duration::from_secs(30));
    assert!(client.public_client.is_none());
    assert!(client.private_client.is_none());
}

#[test]
fn test_grpc_client_default() {
    let client = GrpcClient::default();

    assert_eq!(client.timeout, Duration::from_secs(30));
}

#[test]
fn test_grpc_error_variants() {
    let not_connected = GrpcError::NotConnected("test".to_string());
    let invalid_endpoint = GrpcError::InvalidEndpoint("test".to_string());
    let connection_failed = GrpcError::ConnectionFailed("test".to_string());
    let request_failed = GrpcError::RequestFailed("test".to_string());
    let api_error = GrpcError::ApiError("test".to_string());
    let timeout = GrpcError::Timeout("test".to_string());
    let invalid_response = GrpcError::InvalidResponse("test".to_string());

    // Test that all error variants can be created
    assert!(matches!(not_connected, GrpcError::NotConnected(_)));
    assert!(matches!(invalid_endpoint, GrpcError::InvalidEndpoint(_)));
    assert!(matches!(connection_failed, GrpcError::ConnectionFailed(_)));
    assert!(matches!(request_failed, GrpcError::RequestFailed(_)));
    assert!(matches!(api_error, GrpcError::ApiError(_)));
    assert!(matches!(timeout, GrpcError::Timeout(_)));
    assert!(matches!(invalid_response, GrpcError::InvalidResponse(_)));
}

#[test]
fn test_grpc_error_display() {
    let error = GrpcError::NotConnected("public API".to_string());
    let error_msg = format!("{}", error);

    assert!(error_msg.contains("Not connected"));
    assert!(error_msg.contains("public API"));
}

#[tokio::test]
async fn test_public_client_creation() {
    // This test will fail since we don't have a real endpoint
    // In TDD, we expect this to fail initially
    let result = docxology::grpc::public_client(Some("https://invalid.endpoint")).await;

    // The test should fail with a connection error
    assert!(result.is_err());

    match result.unwrap_err() {
        GrpcError::ConnectionFailed(_) => {},
        other => panic!("Expected ConnectionFailed error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_private_client_creation() {
    // This test will fail since we don't have a real private endpoint
    // In TDD, we expect this to fail initially
    let result = docxology::grpc::private_client("http://127.0.0.1:8081").await;

    // The test should fail with a connection error
    assert!(result.is_err());

    match result.unwrap_err() {
        GrpcError::ConnectionFailed(_) => {},
        other => panic!("Expected ConnectionFailed error, got: {:?}", other),
    }
}

#[test]
fn test_grpc_client_methods_without_connection() {
    let mut client = GrpcClient::new("https://test.endpoint", 30);

    // These should all fail since we're not connected
    let balance_result = tokio_test::block_on(async {
        client.get_balance("test_address").await
    });

    assert!(matches!(balance_result, Err(GrpcError::NotConnected(_))));
}
