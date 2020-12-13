# huawei-e3372-exporter
A Prometheus exporter for the Huawei E3372 4G Dongle

**Usage**
---

There is currently one optional option to let you specify the config file path
If this is not specified, the exporter will look for the default path of "./config.yml"

```
USAGE:
    huawei-e3372-exporter [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <config>        config file path
```

**Configuration**
---

The config file is split into two sections:

***Settings***


All fields under settings are _required_ to be set, the fields are as follows:


| Config Option  | Function |
| ------------- | ------------- |
| listen_address  | The local address to bind to and listen for requests on  |
| listen_port  | The local port to bind to and listen for requests on  |
| dongle_ip  | The ip address of the 4G dongle admin panel  |
| collection_timeout_seconds  | How long to allow a single request to the API to take before erroring  |

***Metrics***


The metrics section is a list or collection of all the metrics you wish to scrape, all fields for a single metric are _required_


| Config Option  | Data Type | Function |
| ------------- | ------------- | ------------- |
| name  | String | The name of the metric, this becomes the prometheus metric name  |
| enabled  | Boolean | Turn on or off the collection AND publishing of this metric for prometheus to scrape  |
| path  | String | The Huawei API path the metric is found under  |
| xml_tag  | String | The Huawei API xml tag the metric value is found under  |
| help_text | String | Prometheus help text / description for the metric  |
| metric_type | String | Prometheus metric type, counter, gauge, histogram or summary  |


Example Configuration:
```yaml 
settings:
    listen_address: 0.0.0.0
    listen_port: 9898
    dongle_ip: 192.168.8.1
    collection_timeout_seconds: 2

metrics:
  - name: current_connect_time
    enabled: true
    path: /monitoring/traffic-statistics
    xml_tag: CurrentConnectTime
    help_text: Duration the device has been connected to the mobile network
    metric_type: counter
    
  - name: current_download_total
    enabled: true
    path: /monitoring/traffic-statistics
    xml_tag: CurrentDownload
    help_text: Total bytes downloaded since being connected to the mobile network
    metric_type: counter
    .....
```


**Example Scrape**
---

``` 
curl localhost:9898/metrics
# HELP current_connect_time Duration the device has been connected to the mobile network
# TYPE current_connect_time counter
current_connect_time 358 1607894539316
# HELP current_download_total Total bytes downloaded since being connected to the mobile network
# TYPE current_download_total counter
current_download_total 17591 1607894539316
# HELP current_upload_total Total bytes uploaded since being connected to the mobile network
# TYPE current_upload_total counter
current_upload_total 12699 1607894539316
# HELP current_download_rate Current bits per second download rate
# TYPE current_download_rate gauge
current_download_rate 0 1607894539316
# HELP current_upload_rate Current bits per second upload rate
# TYPE current_upload_rate gauge
current_upload_rate 0 1607894539316
```
