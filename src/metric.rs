use crate::huawei_api::get_value_from_tag;
use anyhow::Result;
use prometheus_exporter_base::*;
use serde::Deserialize;
use std::convert::TryFrom;
#[derive(Debug, Deserialize, Clone)]
pub struct Metric {
    pub enabled: bool,
    pub path: String,
    pub xml_tag: String,
    pub name: String,
    pub help_text: String,
    pub metric_type: String,
}

impl Metric {
    pub async fn build(&mut self, xml: &str) -> Result<String> {
        let mut prom_metric = PrometheusMetric::build()
            .with_name(&self.name)
            .with_metric_type(MetricType::try_from(self.metric_type.as_str()).unwrap())
            .with_help(&self.help_text)
            .build();

        let metric_value = get_value_from_tag(&xml, &self.xml_tag).await?;

        prom_metric.render_and_append_instance(
            &PrometheusInstance::new()
                .with_value(metric_value.parse::<i32>().unwrap())
                .with_current_timestamp()
                .expect("error getting the current UNIX epoch"),
        );

        Ok(prom_metric.render())
    }
}
