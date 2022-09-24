extern crate colored;
use colored::*;
use kube::{api::{ListParams, LogParams}, client::Client, Api, config::{KubeConfigOptions}};
use k8s_openapi::api::core::v1::Pod;
use kube::ResourceExt;
use futures::{TryStreamExt};
use clap::{Parser};
use rand::Rng;

#[derive(Parser)]
#[clap(author = "Tanisha Banik", version="1.0.0", about="Software Developer")]
struct Kubetail{
    #[clap(short, long, value_parser, help="list all matching pods and containers names", default_value_t=false)]
    dry_run: bool,
    #[clap(short, long, value_parser, help="pod namespace", default_value="default")]
    namespace: String,
    #[clap(short, long, value_parser, help="pod label selector", default_value="")]
    label: String,
    #[clap(short, long, value_parser, help="returns previous terminated container logs. Defaults to false", default_value_t=false)]
    previous: bool,
    #[clap(short, long, value_parser, help="follows the log stream of the pod. Defaults to false", default_value_t=false)]
    follow: bool,
    #[clap(short, long, value_parser, help="number of lines from the end of the logs to show. Defaults to 0, shows streams right from the creation of the container", default_value_t=0)]
    tail: i64,
    #[clap(short='k', long, value_parser, help="pod name regex", default_value = "")]
    name: String,
    #[clap(short='c', long, value_parser, help="container name", default_value = "")]
    container: String,
    // #[clap(short='b', long, value_parser, help="number of bytes to read from the server before terminating the log output")]
    // limit_bytes: i64,
    #[clap(short= 'm', long, value_parser, help="pretty print output", default_value_t=false)]
    pretty: bool,
    #[clap(short, long, value_parser, help="relative time in seconds before the current time from which to show logs", default_value_t=20)]
    since: u64,
}
#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let args = Kubetail::parse();
    
    let kube_config = kube::config::KubeConfigOptions{cluster:Some("CainaticsDev".to_owned()),..KubeConfigOptions::default()};
    let config = kube::Config::from_kubeconfig(&kube_config).await?;
    let client:  Client = Client::try_from(config)?;
    //let client: Client = Client::try_default().await.expect("Expected a valid KUBECONFIG env variable");
    let mut namespace = "default";
    let mut previous = false;
    let mut pretty = false;
    
    let mut follow_log = true;
    let mut tail: Option<i64> = Some(0);
    // let mut limit_bytes: Option<i64> = Some(-1);
    let mut lp = ListParams::default();
    let mut app_name = "";
    let mut since_time: Option<i64> = Some(20);

    if args.namespace != "default" {
        namespace = &args.namespace;
    }
    if args.name != ""{
        app_name = &args.name;
    }
    if args.since != 20 {
        since_time = Some(args.since as i64);
    }
    if args.label != ""{
        lp = ListParams::default().labels(&args.label);
    }
    if args.follow {
        follow_log = args.follow;
    }
    if args.pretty{
        pretty = args.pretty;
    }
    if args.previous {
        previous = args.previous;
    }
    if args.tail != 0 {
        tail = Some(args.tail);
        println!("Tail: {:?}", tail);
    }
    // if args.limit_bytes != -1{
    //     limit_bytes = Some(args.limit_bytes);
    // }

    let pods_api: Api<Pod> = Api::namespaced(client, namespace);
    
    // let lp_with_labels = ListParams::default().labels("app=iat-dev");
    if args.dry_run {
        println!("No dry run");
        for p in pods_api.list(&lp).await{
            for pod in p.items{
                if pod.name_any().contains(app_name) {
                    let mut rng = rand::thread_rng();
                    let mut owned_string: String = "Pod Name: ".to_owned();
                    owned_string.push_str(&pod.name_any()[..]);
                    println!("{}", owned_string.truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                    let containers = pod.spec.unwrap().containers;
                    for cont in containers{
                        let mut owned_string: String = "Container Name: ".to_owned();
                        let cont_name: &str = &cont.name[..];
                        owned_string.push_str(cont_name);
                        println!("{}", owned_string.truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))); 
                    }
                }
            }
        }
    }else{
        for p in pods_api.list(&lp).await {
            for pod in p.items{
                if pod.name_any().contains(app_name){
                    
                    let mut rng = rand::thread_rng();
                    let mut owned_string: String = "Pod Name: ".to_owned();
                    let pod_name:  &str = &pod.name_any()[..];
                    owned_string.push_str(&pod.name_any()[..]);
                    
                    println!("{}", owned_string.truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                    
                    let containers = pod.spec.unwrap().containers;
                    let default_container = &containers[1];
        
                    
                    if containers.len() >= 1 || args.container != ""{
                        for cont in &containers{
                            if cont.name == args.container{
                                let mut owned_string: String = "Container Name: ".to_owned();
                                let cont_name: &str = &cont.name[..];
                                owned_string.push_str(cont_name);
                                println!("{}", owned_string.truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                                let container_name: Option<String> = Some(cont.name.to_owned());
                                if args.tail == 0{
                                    let mut logs = pods_api.log_stream(pod_name, &LogParams{
                                        follow: follow_log,
                                        container: container_name.to_owned(),
                                        since_seconds:since_time,
                                        pretty: pretty,
                                        previous: previous,
                                        ..LogParams::default()
                                    }).await?;
                                    while let Some(line) = logs.try_next().await? {
                                        println!("Logs: {}", String::from_utf8_lossy(&line).truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                                    }
                                }else{
                                    let mut logs = pods_api.log_stream(pod_name, &LogParams{
                                        follow: follow_log,
                                        container: container_name.to_owned(),
                                        since_seconds:since_time,
                                        pretty: pretty,
                                        previous: previous,
                                        // limit_bytes: limit_bytes,
                                        tail_lines: tail,
                                        ..LogParams::default()
                                    }).await?;
                                    while let Some(line) = logs.try_next().await? {
                                        println!("Logs: {}", String::from_utf8_lossy(&line).truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                                    }
                                }
                                     
                            }
                            else{
                                if args.tail == 0{
                                    let mut logs = pods_api.log_stream(pod_name, &LogParams{
                                        follow: follow_log,
                                        container: Some(default_container.name.to_owned()),
                                        since_seconds:since_time,
                                        pretty: pretty,
                                        previous: previous,
                                        ..LogParams::default()
                                    }).await?;
                                    println!("here");
                                    while let Some(line) = logs.try_next().await? {
                                        println!("Logs: {}", String::from_utf8_lossy(&line).truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                                    }

                                }else{
                                    let mut logs = pods_api.log_stream(pod_name, &LogParams{
                                        follow: follow_log,
                                        container: Some(default_container.name.to_owned()),
                                        since_seconds:since_time,
                                        pretty: pretty,
                                        previous: previous,
                                        tail_lines: tail,
                                        ..LogParams::default()
                                    }).await?;
                                    
                                    while let Some(line) = logs.try_next().await? {
                                        println!("Logs: {}", String::from_utf8_lossy(&line).truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                                    }
                                }
                                
                            }
                            

                        }
                    }else {
                        println!("Only one container");
                        // if containers[0].name == "ira-server"{
                        //     let mut owned_string: String = "Container Name: ".to_owned();
                        //     let cont_name: &str = &containers[0].name[..];
                        //     owned_string.push_str(cont_name);
                        //     println!("{}", owned_string.truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                        //     let container_name: Option<String> = Some(containers[0].name.to_owned());
                        if args.tail == 0{
                            let mut logs = pods_api.log_stream(pod_name, &LogParams{
                                follow: follow_log,
                                since_seconds:since_time,
                                pretty: pretty,
                                previous: previous,
                                ..LogParams::default()
                            }).await?;
                            while let Some(line) = logs.try_next().await? {
                                println!("Logs: {}", String::from_utf8_lossy(&line).truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                            }
                        }else{
                            let mut logs = pods_api.log_stream(pod_name, &LogParams{
                                follow: follow_log,
                                since_seconds:since_time,
                                pretty: pretty,
                                previous: previous, 
                                tail_lines: tail,
                                ..LogParams::default()
                            }).await?;
                            while let Some(line) = logs.try_next().await? {
                                println!("Logs: {}", String::from_utf8_lossy(&line).truecolor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
                            }
                        }   
                        
                        // }
                    }
                    // let mut logs = pods_api.log_stream(pod_name, &LogParams{
                    //     follow: true,
                    //     container: container_name.to_owned(),
                    //     ..LogParams::default()
                    // }).await?;
                    // while let Some(line) = logs.try_next().await? {
                    //     println!("{:?}", String::from_utf8_lossy(&line));
                    // }
                }
            }        
        }

    }
    
    Ok(())
        
}
    
