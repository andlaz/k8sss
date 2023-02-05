use crate::cli::{
    Cli, Commands, WaitCommands, WaitUntilCommands, WaitUntilJobCommand, WaitUntilServiceCommands,
};
use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

mod cli;
mod kube;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let filter = match EnvFilter::try_from_env("RUST_LOG") {
        Ok(env_filter) => env_filter,
        _ => EnvFilter::try_new(tracing::Level::INFO.to_string())
            .expect("Failed to initialize default tracing level to INFO"),
    };

    let fmt = tracing_subscriber::fmt::layer();
    let registry = Registry::default().with(filter).with(fmt);

    registry.init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Wait { command } => {
            match command {
                WaitCommands::Until { until } => {
                    match until {
                        WaitUntilCommands::Service { service } => {
                            match service {
                                WaitUntilServiceCommands::UnavailableEndpoints {
                                    lte,
                                    name,
                                    namespace,
                                } => {
                                    // call begin_watching_service for the above service name and namespace
                                    // and pass a callback function that will resolve endpoints for the
                                    // service and exit the program with code 0 whenever this count falls
                                    // to or below the value specified in lte
                                    kube::begin_watching_service(
                                        namespace.unwrap_or("default".to_string()),
                                        name,
                                        move |_, endpoints| {
                                            if endpoints.items.is_empty() == false {
                                                let unready_count = endpoints
                                                    .items
                                                    .iter()
                                                    .flat_map(|eps| eps.endpoints.clone())
                                                    .filter(|ep| {
                                                        ep.conditions.clone().map_or(
                                                            false,
                                                            |conditions| {
                                                                conditions.ready.unwrap_or(false)
                                                            },
                                                        ) == false
                                                    })
                                                    .count();
                                                if unready_count <= lte as usize {
                                                    std::process::exit(0);
                                                }
                                            }
                                        },
                                    )
                                    .await?;
                                }
                                WaitUntilServiceCommands::AvailableEndpoints {
                                    gte,
                                    name,
                                    namespace,
                                } => {
                                    // call begin_watching_service for the above service name and namespace
                                    // and pass a function that will exit the program with code 0
                                    // whenever the Service's count of health endpoints rise to or above
                                    // the value specified in gte
                                    kube::begin_watching_service(
                                        namespace.unwrap_or("default".to_string()),
                                        name,
                                        move |_, endpoints| {
                                            if endpoints.items.is_empty() == false {
                                                let ready_count = endpoints
                                                    .items
                                                    .iter()
                                                    .flat_map(|eps| eps.endpoints.clone())
                                                    .filter(|ep| {
                                                        ep.conditions.clone().map_or(
                                                            false,
                                                            |conditions| {
                                                                conditions.ready.unwrap_or(false)
                                                            },
                                                        )
                                                    })
                                                    .count();
                                                if ready_count >= gte as usize {
                                                    std::process::exit(0);
                                                }
                                            }
                                        },
                                    )
                                    .await?;
                                }
                            }
                        }
                        WaitUntilCommands::Job { job } => {
                            match job {
                                WaitUntilJobCommand::Ready { name, namespace } => {
                                    // call begin_watching_job for the above job name and namespace
                                    // and pass a function that will exit the program with code 0
                                    // whenever the Job's status is Ready
                                    kube::begin_watching_job(
                                        namespace.unwrap_or("default".to_string()),
                                        name,
                                        |job| {
                                            if job
                                                .status
                                                .as_ref()
                                                .map(|status| status.ready)
                                                .flatten()
                                                .unwrap_or(0)
                                                >= 1
                                            {
                                                std::process::exit(0);
                                            }
                                        },
                                    )
                                    .await?;
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    Ok(())
}
