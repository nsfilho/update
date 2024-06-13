use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceContainerPrivileges {
    #[serde(rename = "CredentialSpec")]
    pub credential_spec: Option<HashMap<String, String>>,
    #[serde(rename = "SELinuxContext")]
    pub selinux_context: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceContainerMountVolumeOptionsDriverConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceContainerMountVolumeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "DriverConfig")]
    pub driver_config: Option<ServiceContainerMountVolumeOptionsDriverConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceContainerMount {
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "VolumeOptions")]
    pub volume_options: Option<ServiceContainerMountVolumeOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceContainerSpecConfigFile {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "UID")]
    pub uid: String,
    #[serde(rename = "GID")]
    pub gid: String,
    #[serde(rename = "Mode")]
    pub mode: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceContainerSpecConfig {
    #[serde(rename = "File")]
    pub file: Option<ServiceContainerSpecConfigFile>,
    #[serde(rename = "ConfigID")]
    pub config_id: String,
    #[serde(rename = "ConfigName")]
    pub config_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceContainerSpecHealthCheck {
    #[serde(rename = "Test")]
    pub test: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Interval")]
    pub interval: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Timeout")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Retries")]
    pub retries: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceContainerSpec {
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Privileges")]
    pub privileges: Option<ServiceContainerPrivileges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<ServiceContainerMount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Configs")]
    pub configs: Option<Vec<ServiceContainerSpecConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Healthcheck")]
    pub health_check: Option<ServiceContainerSpecHealthCheck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Isolation")]
    pub isolation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTaskTemplatePlacement {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Constraints")]
    pub constraints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Platforms")]
    pub platforms: Option<Vec<HashMap<String, String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTaskTemplateNetworks {
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTaskTemplateResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Limits")]
    pub limits: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Reservations")]
    pub reservations: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTaskTemplate {
    #[serde(rename = "ContainerSpec")]
    pub container_spec: ServiceContainerSpec,
    #[serde(rename = "Resources")]
    pub resources: Option<ServiceTaskTemplateResources>,
    #[serde(rename = "Placement")]
    pub placement: Option<ServiceTaskTemplatePlacement>,
    #[serde(rename = "Networks")]
    pub networks: Option<Vec<ServiceTaskTemplateNetworks>>,
    #[serde(rename = "ForceUpdate")]
    pub force_update: Option<u64>,
    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSpecModeReplicated {
    #[serde(rename = "Replicas")]
    pub replicas: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSpecModeGlobal {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSpecMode {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Replicated")]
    pub replicated: Option<ServiceSpecModeReplicated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Global")]
    pub global: Option<ServiceSpecModeGlobal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceEndpointSpecPortConfig {
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "TargetPort")]
    pub target_port: u64,
    #[serde(rename = "PublishedPort")]
    pub published_port: Option<u64>,
    #[serde(rename = "PublishMode")]
    pub publish_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceEndpointSpec {
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<ServiceEndpointSpecPortConfig>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSpec {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "TaskTemplate")]
    pub task_template: ServiceTaskTemplate,
    #[serde(rename = "Mode")]
    pub mode: Option<ServiceSpecMode>,
    #[serde(rename = "EndpointSpec")]
    pub endpoint_spec: Option<ServiceEndpointSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceVersion {
    #[serde(rename = "Index")]
    pub index: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceEndpointVirtualIP {
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "Addr")]
    pub addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    #[serde(rename = "Spec")]
    pub spec: ServiceEndpointSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "VirtualIPs")]
    pub virtual_ips: Option<Vec<ServiceEndpointVirtualIP>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<ServiceEndpointSpecPortConfig>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceUpdateStatus {
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "StartedAt")]
    pub started_at: DateTime<Utc>,
    #[serde(rename = "CompletedAt")]
    pub completed_at: DateTime<Utc>,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Version")]
    pub version: ServiceVersion,
    #[serde(rename = "CreatedAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "Spec")]
    pub spec: ServiceSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "PreviousSpec")]
    pub previous_spec: Option<ServiceSpec>,
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<ServiceEndpoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "UpdateStatus")]
    pub update_status: Option<ServiceUpdateStatus>,
}
