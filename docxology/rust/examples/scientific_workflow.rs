//! Example: Complete Scientific Workflow with Nockchain
//!
//! This example demonstrates a complete scientific research workflow using Nockchain
//! for verifiable computation, data provenance, and collaborative research.

use docxology::{
    config::{NodeConfig, MinerConfig, WalletConfig},
    node::start_node,
    miner::start_miner,
    wallet::{create_wallet, KeySource},
    flows::{setup_and_start_miner, create_and_send_transaction},
    grpc::GrpcClient,
};
use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use tracing_subscriber;

/// Scientific research workflow using Nockchain
struct ScientificWorkflow {
    node_handle: Option<docxology::node::NodeHandle>,
    miner_handle: Option<docxology::miner::MinerHandle>,
    wallet: Option<docxology::wallet::WalletManager>,
    experiment_id: String,
}

impl ScientificWorkflow {
    /// Create a new scientific workflow
    fn new(experiment_id: &str) -> Self {
        Self {
            node_handle: None,
            miner_handle: None,
            wallet: None,
            experiment_id: experiment_id.to_string(),
        }
    }

    /// Initialize the research infrastructure
    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔬 Initializing scientific research infrastructure...");

        // Create data directories
        let node_data_dir = PathBuf::from(format!("/tmp/scientific_workflow_{}_node", self.experiment_id));
        let wallet_data_dir = PathBuf::from(format!("/tmp/scientific_workflow_{}_wallet", self.experiment_id));

        // Configure and start a research node
        let mut node_config = NodeConfig::default();
        node_config.data_dir = Some(node_data_dir);
        node_config.api.enable_public_api = true;
        node_config.api.public_api_addr = "127.0.0.1:8080".to_string();

        // Configure mining for computational rewards
        let miner_config = MinerConfig {
            enabled: true,
            threads: 2,
            pubkey: Some("research_mining_key".to_string()),
            ..Default::default()
        };
        node_config.mining = Some(miner_config.clone());

        // Start the research node
        let node_handle = start_node(node_config).await?;
        self.node_handle = Some(node_handle);

        println!("✅ Research node started");

        // Create research wallet
        let mut wallet_config = WalletConfig::default();
        wallet_config.data_dir = Some(wallet_data_dir);

        let wallet = create_wallet(wallet_config).await?;
        self.wallet = Some(wallet);

        println!("✅ Research wallet created");

        // Start miner for computational rewards
        let miner_handle = start_miner(miner_config).await?;
        self.miner_handle = Some(miner_handle);

        println!("✅ Research miner started");

        // Wait for blockchain to be ready
        sleep(Duration::from_secs(5)).await;

        Ok(())
    }

    /// Conduct verifiable experiment
    async fn conduct_experiment(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧪 Conducting verifiable scientific experiment...");

        let wallet = self.wallet.as_ref().unwrap();

        // Generate researcher's identity
        let keypair = wallet.keygen().await?;
        println!("🔑 Researcher identity generated: {}", keypair.public_key);

        // Define experimental protocol
        let protocol = ResearchProtocol {
            experiment_id: self.experiment_id.clone(),
            hypothesis: "Nockchain enables verifiable scientific computation".to_string(),
            methodology: "Distributed consensus with zero-knowledge proofs".to_string(),
            sample_size: 100,
            variables: vec![
                "computation_verifiability".to_string(),
                "consensus_reliability".to_string(),
                "incentive_alignment".to_string(),
            ],
        };

        // Record protocol on blockchain for verifiability
        let protocol_record = wallet.record_research_protocol(&protocol).await?;
        println!("📋 Research protocol recorded: {}", protocol_record);

        // Simulate data collection
        let experimental_data = self.collect_experimental_data().await?;
        println!("📊 Collected {} data points", experimental_data.len());

        // Perform verifiable computation
        let computation_result = self.perform_verifiable_computation(&experimental_data).await?;
        println!("🔍 Computation result: {:.4}", computation_result);

        // Store results with provenance
        let results_record = wallet.store_research_results(&protocol, &computation_result).await?;
        println!("💾 Research results stored: {}", results_record);

        Ok(())
    }

    /// Collect experimental data (simulated)
    async fn collect_experimental_data(&self) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        // Simulate collecting experimental measurements
        let mut data = Vec::new();

        for i in 0..100 {
            // Simulate some experimental measurements
            let measurement = (i as f64 * 0.1) + (rand::random::<f64>() * 0.1);
            data.push(measurement);

            // Simulate real-time data recording on blockchain
            sleep(Duration::from_millis(100)).await;
        }

        Ok(data)
    }

    /// Perform verifiable computation on experimental data
    async fn perform_verifiable_computation(&self, data: &[f64]) -> Result<f64, Box<dyn std::error::Error>> {
        // Calculate statistical measures with verifiability
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
        let std_dev = variance.sqrt();

        // Create zero-knowledge proof of computation
        let proof = ComputationProof::new()
            .input_data(data)
            .computation("statistical_analysis")
            .result(mean)
            .metadata("standard_deviation", std_dev);

        // Verify the computation was performed correctly
        let verification = proof.verify().await?;

        if verification.is_valid {
            println!("✅ Computation verified successfully");
            Ok(mean)
        } else {
            Err("Computation verification failed".into())
        }
    }

    /// Publish research findings
    async fn publish_findings(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 Publishing research findings...");

        let wallet = self.wallet.as_ref().unwrap();

        // Create research publication
        let publication = ResearchPublication {
            title: "Verifiable Scientific Computing with Nockchain".to_string(),
            authors: vec!["Dr. Researcher".to_string()],
            abstract: "This paper demonstrates how Nockchain enables verifiable scientific computation...".to_string(),
            experiment_id: self.experiment_id.clone(),
            results: "Statistical analysis shows significant improvements in computational verifiability".to_string(),
            conclusions: "Nockchain provides a robust foundation for reproducible scientific research".to_string(),
        };

        // Submit for peer review through distributed consensus
        let peer_review = DistributedPeerReview::new(&publication);

        // Collect reviews from qualified experts
        let reviews = peer_review.collect_reviews().await?;

        // Reach consensus on publication quality
        let consensus = peer_review.reach_consensus(reviews).await?;

        if consensus.approved {
            // Publish with cryptographic timestamp and review proofs
            let published_paper = wallet.publish_research(&publication, &consensus).await?;
            println!("🎉 Research published successfully: {}", published_paper.doi);
        } else {
            println!("❌ Research not approved for publication");
        }

        Ok(())
    }

    /// Clean up research infrastructure
    async fn cleanup(mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧹 Cleaning up research infrastructure...");

        if let Some(miner_handle) = self.miner_handle.take() {
            miner_handle.shutdown().await?;
            println!("✅ Miner stopped");
        }

        if let Some(node_handle) = self.node_handle.take() {
            node_handle.shutdown().await?;
            println!("✅ Node stopped");
        }

        // Clean up temporary data
        let _ = std::fs::remove_dir_all("/tmp/scientific_workflow_".to_string() + &self.experiment_id);

        println!("✅ Cleanup completed");
        Ok(())
    }
}

/// Research protocol definition
#[derive(Debug, Clone)]
struct ResearchProtocol {
    experiment_id: String,
    hypothesis: String,
    methodology: String,
    sample_size: usize,
    variables: Vec<String>,
}

/// Computation proof for verifiable calculations
#[derive(Debug)]
struct ComputationProof {
    // Placeholder for computation proof structure
}

impl ComputationProof {
    fn new() -> Self {
        Self {}
    }

    fn input_data(&mut self, _data: &[f64]) -> &mut Self {
        self
    }

    fn computation(&mut self, _comp: &str) -> &mut Self {
        self
    }

    fn result(&mut self, _result: f64) -> &mut Self {
        self
    }

    fn metadata(&mut self, _key: &str, _value: f64) -> &mut Self {
        self
    }

    async fn verify(&self) -> Result<VerificationResult, Box<dyn std::error::Error>> {
        // Simulate verification process
        sleep(Duration::from_secs(1)).await;
        Ok(VerificationResult { is_valid: true })
    }
}

/// Verification result
#[derive(Debug)]
struct VerificationResult {
    is_valid: bool,
}

/// Research publication structure
#[derive(Debug, Clone)]
struct ResearchPublication {
    title: String,
    authors: Vec<String>,
    abstract: String,
    experiment_id: String,
    results: String,
    conclusions: String,
}

/// Distributed peer review system
struct DistributedPeerReview {
    // Placeholder for peer review implementation
}

impl DistributedPeerReview {
    fn new(_publication: &ResearchPublication) -> Self {
        Self {}
    }

    async fn collect_reviews(&self) -> Result<Vec<Review>, Box<dyn std::error::Error>> {
        // Simulate collecting peer reviews
        sleep(Duration::from_secs(2)).await;
        Ok(vec![
            Review { score: 8.5, comments: "Excellent methodology".to_string() },
            Review { score: 7.8, comments: "Good results, minor concerns".to_string() },
            Review { score: 9.2, comments: "Outstanding contribution".to_string() },
        ])
    }

    async fn reach_consensus(&self, reviews: Vec<Review>) -> Result<Consensus, Box<dyn std::error::Error>> {
        let avg_score = reviews.iter().map(|r| r.score).sum::<f64>() / reviews.len() as f64;
        let approved = avg_score >= 8.0;

        Ok(Consensus {
            approved,
            average_score: avg_score,
            review_count: reviews.len(),
        })
    }
}

/// Individual review
#[derive(Debug)]
struct Review {
    score: f64,
    comments: String,
}

/// Consensus result
#[derive(Debug)]
struct Consensus {
    approved: bool,
    average_score: f64,
    review_count: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting Complete Scientific Workflow Example");
    println!("This demonstrates how Nockchain enables verifiable, reproducible scientific research");

    // Create scientific workflow
    let mut workflow = ScientificWorkflow::new("nockchain_science_demo_001");

    // Initialize research infrastructure
    workflow.initialize().await?;

    // Conduct the experiment
    workflow.conduct_experiment().await?;

    // Publish findings
    workflow.publish_findings().await?;

    // Clean up
    workflow.cleanup().await?;

    println!("🎉 Scientific workflow completed successfully!");
    println!("This demonstrates how Nockchain can transform scientific research by:");
    println!("  • Providing verifiable computation");
    println!("  • Ensuring data provenance");
    println!("  • Enabling distributed collaboration");
    println!("  • Creating incentive alignment");
    println!("  • Supporting reproducible research");

    Ok(())
}
