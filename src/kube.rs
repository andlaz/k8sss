use futures_util::StreamExt;
use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::core::v1::Service;
use k8s_openapi::api::discovery::v1::EndpointSlice;
use kube::api::ListParams;
use kube::runtime::{watcher, WatchStreamExt};
use kube::{Api, Client};

use tracing::*;

pub(crate) async fn begin_watching_job(
    namespace: String,
    name: String,
    f: impl Fn(&Job) + Send + Sync + 'static,
) -> anyhow::Result<()> {
    if let Ok(incluster_config) = kube::Config::incluster_dns() {
        if let Ok(client) = kube::Client::try_from(incluster_config) {
            let api: Api<Job> = Api::namespaced(client, &namespace);

            let lp = ListParams::default().fields(&format!("metadata.name={}", name));
            watcher(api, lp)
                .applied_objects()
                .for_each(|event| async {
                    match event {
                        Ok(job) => f(&job),
                        Err(e) => error!("Error watching Job: {:?}", e),
                    }
                })
                .await;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Unable to create incluster client"))
        }
    } else {
        Err(anyhow::anyhow!("Unable to load incluster config"))
    }
}

// Takes a namespace name, a service name and a function that will be called
// by a watcher whenever the service is updated.
pub(crate) async fn begin_watching_service(
    namespace: String,
    name: String,
    f: impl Fn(&Service, &EndpointSlice) + Send + Sync + 'static,
) -> anyhow::Result<()> {
    if let Ok(incluster_config) = kube::Config::incluster_dns() {
        if let Ok(client) = kube::Client::try_from(incluster_config) {
            let api: Api<EndpointSlice> = Api::namespaced(client, &namespace);
            let services: Api<Service> = Api::namespaced(Client::try_default().await?, &namespace);

            let lp = ListParams::default().labels(&format!("kubernetes.io/service-name={}", name));
            watcher(api, lp)
                .applied_objects()
                .for_each(|event| async {
                    match event {
                        Ok(eps) => match services.get(&name).await {
                            Ok(service) => {
                                f(&service, &eps);
                            }
                            Err(e) => {
                                error!("Error getting EndpointSlices for Service: {:?}", e);
                            }
                        },
                        Err(e) => {
                            error!("Error watching Service: {:?}", e);
                        }
                    }
                })
                .await;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Unable to create incluster client"))
        }
    } else {
        Err(anyhow::anyhow!("Unable to load incluster config"))
    }
}
