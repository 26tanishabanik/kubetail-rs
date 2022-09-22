use kube::{api::{ListParams}, client::Client, runtime::controller::Action, runtime::Controller, Api,CustomResourceExt, Error};
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::api::core::v1::PodStatus;
use k8s_openapi::api::core::v1::PodCondition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use k8s_openapi::Metadata;
use kube::api::*;
use kube::api::{Patch, PatchParams,DeleteParams, ObjectMeta, PostParams, LogParams};
use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
use k8s_openapi::api::core::v1::{Container, ContainerPort, PodSpec, PodTemplateSpec};
use serde_json::{json, Value, Serializer};
use std::sync::{Arc,Mutex};
use futures::stream::StreamExt;
use kube::Resource;
use kube::ResourceExt;
use tokio::time::Duration;
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use std::ops::{Deref, DerefMut};
use json_patch;
use clap::{App, Arg};

async fn get_logs(client: Client, ns: &str) -> Result<(), Error>{
    // let pod_api: Api<Pod> = Api::namespaced(client, ns);
    let appLabel: &str = "app=nginx";
    let pods: Api<Pod> = Api::namespaced(client, ns);
    // let p: Pod = pods.get("nginx").await?;
    // println!("{:?}", p);
    let lp = ListParams::default().labels("app=nginx"); // for this app only
    println!("Hihi");
    for p in pods.list(&lp).await? {
        println!("Found Pod: {}", p.name_any());
    }
    Ok(())
    // Ok(p)
    // println!("Hi")
}



#[tokio::main]
async fn main() {
    let matches = App::new("kubetail rs")
        .version("0.1.0")
        .author("Tanisha Banik <26tanishabanik@gmail.com>")
        .about("Software Developer")
        .arg(Arg::with_name("namespace")
                 .short("n")
                 .long("namespace")
                 .takes_value(true)
                 .help("pod namespace"))
        .arg(Arg::with_name("podLabel")
                 .short("l")
                 .long("label")
                 .takes_value(true)
                 .help("pod label to search logs for"))
        .get_matches();
    let client: Client = Client::try_default().await.expect("Expected a valid KUBECONFIG env variable");
    get_logs(client, "default");
}
