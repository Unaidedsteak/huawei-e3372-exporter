use anyhow::{Context, Error};
use clap::{crate_name, crate_version, Arg};
use huawei_api::generic_request;
use log::debug;
use prometheus_exporter_base::*;
use std::collections::HashMap;
use std::fs;

mod config;
use config::Config;
mod huawei_api;
mod metric;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let config_path: String;
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .help("config file path")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("config") {
        config_path = matches.value_of("config").unwrap().to_string();
        debug!(
            "commandline flag for config file path specified, loading: {}",
            config_path
        )
    } else {
        debug!("no commandline flag for config file path specified, using default path");
        config_path = "./config.yml".to_string();
    }

    let config_file = fs::read_to_string(config_path).context("Config file not found")?;
    let config: Config = serde_yaml::from_str(&config_file).context("Config file invalid")?;

    let settings = config.settings;
    let metrics = config.metrics;
    let addr = (settings.listen_address, settings.listen_port).into();
    let mut api_results: HashMap<String, String> = HashMap::new();
    let mut api_session = huawei_api::Session::new(
        settings.dongle_ip.to_string(),
        settings.collection_timeout_seconds,
    );

    render_prometheus(addr, {}, |_request, _options| async move {
        api_session.refresh_session_token().await.unwrap();
        let mut prom_result = String::new();

        // Populate our hashmap with all the unique API paths we're going to query
        // Only those metrics deemed enabled require us to query the endpoint
        // This should hopefully on unecessary or duplicate queries.
        for metric in &metrics {
            if metric.enabled {
                if !api_results.contains_key(&metric.path) {
                    let response = generic_request(&api_session, &metric.path).await.unwrap();
                    api_results.insert(metric.path.clone(), response);
                }
            }
        }

        // Iterate over all the enabled metrics and build a Prometheus metric
        // based on the existing API response data.
        for mut metric in metrics {
            if metric.enabled {
                let xml_response = api_results
                    .get(&metric.path)
                    .expect(format!("No results found for API path: {}", &metric.path).as_str());
                prom_result.push_str(&metric.build(xml_response).await.unwrap());
            }
        }
        Ok(prom_result)
    })
    .await;
    Ok(())
}
