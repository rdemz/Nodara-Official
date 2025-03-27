
---

### Fichier : src/main.rs

```rust
use clap::{Parser, Subcommand};
use std::error::Error;
use std::process;

mod sdk;
mod governance_sdk;

/// Legendary Nodara CLI - Command Line Interface for Nodara BIOSPHÈRE QUANTIC
#[derive(Parser)]
#[command(author, version, about = "Nodara BIOSPHÈRE QUANTIC Legendary CLI", long_about = None)]
struct Cli {
    /// Activate verbose mode for detailed debugging output.
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Governance-related operations with legendary precision.
    Governance {
        #[command(subcommand)]
        subcommand: GovernanceCommands,
    },
    // Additional commands (e.g., Identity, Marketplace) can be added in the future.
}

#[derive(Subcommand)]
enum GovernanceCommands {
    /// Submit a new governance proposal.
    Submit {
        /// A concise description of the proposal.
        description: String,
        /// The network parameter to update.
        parameter: String,
        /// The new value for the parameter.
        value: String,
    },
    /// Cast a vote on an existing governance proposal.
    Vote {
        /// The unique identifier of the proposal.
        proposal_id: String,
        /// Vote: true for approval, false for rejection.
        vote: bool,
    },
    /// Execute an approved governance proposal.
    Execute {
        /// The unique identifier of the proposal.
        proposal_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Legendary verbose mode activated. Detailed debugging output enabled.");
    }

    match &cli.command {
        Commands::Governance { subcommand } => {
            // Retrieve the base API URL from configuration or environment.
            let base_url = std::env::var("NODARA_API_URL").unwrap_or_else(|_| "https://testnet.nodara.io/api".to_string());
            let governance_sdk = governance_sdk::NodaraGovernanceSDK::new(base_url);

            match subcommand {
                GovernanceCommands::Submit { description, parameter, value } => {
                    match governance_sdk.submit_proposal(description, parameter, value).await {
                        Ok(response) => {
                            println!("Proposal submitted successfully:\n{:#?}", response);
                        }
                        Err(e) => {
                            eprintln!("Error during proposal submission: {}", e);
                            process::exit(1);
                        }
                    }
                }
                GovernanceCommands::Vote { proposal_id, vote } => {
                    match governance_sdk.vote_proposal(proposal_id, *vote).await {
                        Ok(response) => {
                            println!("Vote recorded successfully:\n{:#?}", response);
                        }
                        Err(e) => {
                            eprintln!("Error during voting: {}", e);
                            process::exit(1);
                        }
                    }
                }
                GovernanceCommands::Execute { proposal_id } => {
                    match governance_sdk.execute_proposal(proposal_id).await {
                        Ok(response) => {
                            println!("Proposal executed successfully:\n{:#?}", response);
                        }
                        Err(e) => {
                            eprintln!("Error during proposal execution: {}", e);
                            process::exit(1);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
