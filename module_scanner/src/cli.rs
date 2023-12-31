use futures::stream;
use futures::StreamExt;
use reqwest::Client;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::{Duration, Instant};

use crate::dns;
use crate::modules::HttpModule;
use crate::modules::Subdomain;
use crate::ports;
use crate::{modules, Error};

pub fn modules() {
    let http_modules = modules::all_http_modules();
    let subdomains_modules = modules::all_subdomains_modules();

    println!("Subdomains modules");
    for module in subdomains_modules {
        println!("  {}: {}", module.name(), module.description());
    }

    println!("Http modules");
    for module in http_modules {
        println!("  {}: {}", module.name(), module.description());
    }
}

pub fn scan(target: &str) -> Result<(), Error> {
    log::info!("Scanning: {}", target);

    //set up the client and dns resolver
    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;
    let dns_resolver = dns::new_resolver();

    //set up params for the concurrency
    let subdomains_concurrency = 50;
    let dns_concurrency = 100;
    let ports_concurrency = 200;
    let vulnerabilities_concurrency = 20;

    //for later tracking of how long the scan took
    let scan_start = Instant::now();

    //retrieve all the subdomain modules
    let subdomains_modules = modules::all_subdomains_modules();

    //start the tokio runtime for async enablement
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Building tokio's runtime");

    //block the current thread until it finishes executing
    runtime.block_on(async move {
        //1. Concurrently scan subdomains
        let mut subdomains: Vec<String> = stream::iter(subdomains_modules.into_iter())
            .map(|module| async move {
                match module.enumerate(target).await {
                    Ok(new_subdomains) => {
                        println!(
                            "Found new subdomains {:?} using module {}",
                            new_subdomains,
                            module.name()
                        );
                        Some(new_subdomains)
                    }
                    Err(err) => {
                        log::error!("subdomains/{}: {}", module.name(), err);
                        None
                    }
                }
            })
            .buffer_unordered(subdomains_concurrency)
            .filter_map(|domain| async { domain })
            .collect::<Vec<Vec<String>>>()
            .await
            .into_iter()
            .flatten()
            .collect();

        subdomains.push(target.to_string());

        //2. Deduplicate, clean and convert results to Vec type of Subdomain
        let subdomains: Vec<Subdomain> = HashSet::<String>::from_iter(subdomains.into_iter())
            .into_iter()
            .filter(|subdomain| subdomain.contains(target))
            .map(|domain| Subdomain {
                domain,
                open_ports: Vec::new(),
            })
            .collect();

        log::info!("Found {} domains", subdomains.len());

        //3. Concurrently filter unresolvable domains using DNS Resolver
        let subdomains: Vec<Subdomain> = stream::iter(subdomains.into_iter())
            .map(|domain| dns::resolves(&dns_resolver, domain))
            .buffer_unordered(dns_concurrency)
            .filter_map(|domain| async move { domain })
            .collect()
            .await;

        //4. Concurrently scan ports within each available and resolvable subdomain
        let subdomains: Vec<Subdomain> = stream::iter(subdomains.into_iter())
            .map(|domain| {
                log::info!("Scanning ports for {}", &domain.domain);
                ports::scan_ports(ports_concurrency, domain)
            })
            .buffer_unordered(10)
            .collect()
            .await;

        for subdomain in &subdomains {
            println!("{}", subdomain.domain);
            println!("     open ports:");
            for port in &subdomain.open_ports {
                println!("      {}", port.port);
            }
        }

        println!("---------------------Vulnerabilities------------------");

        //5. Concurrently scan for vulnerabilities
        let mut targets: Vec<(Box<dyn HttpModule>, String)> = Vec::new();
        for subdomain in subdomains {
            for port in subdomain.open_ports {
                let http_modules = modules::all_http_modules();
                for http_module in http_modules {
                    let target = format!("http://{}:{}", &subdomain.domain, port.port);
                    targets.push((http_module, target));
                }
            }
        }

        stream::iter(targets.into_iter())
            .for_each_concurrent(vulnerabilities_concurrency, move |(module, target)| {
                let http_client = http_client.clone();
                async move {
                    match module.scan(&http_client, &target).await {
                        Ok(Some(finding)) => println!("{:?}", &finding),
                        Ok(None) => (),
                        // println!("No Vulnerability found for target {} and module {}", target, module.name()),
                        Err(err) => log::debug!("Error: {}", err),
                    }
                }
            })
            .await;
    });

    let scan_duration = scan_start.elapsed();
    log::info!("Scan completed in {:?}", scan_duration);

    Ok(())
}
