use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::Value;

use crate::errors::Result;
use crate::request::{get, put, Body};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[serde(default)]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CAConfig {
    Provider: String,
    Config: Value,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[serde(default)]
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CARootList {
    ActiveRootID: String,
    TrustDomain: String,
    Roots: Vec<CARoot>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CARoot {
    ID: String,
    Name: String,
    RootCert: String,
    Active: bool,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[async_trait]
pub trait ConnectCA {
    async fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)>;
    async fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)>;
    async fn ca_set_config(&self, conf: &CAConfig, q: Option<&WriteOptions>) -> Result<((), WriteMeta)>;
}

#[async_trait]
impl ConnectCA for Client {
    /// https://www.consul.io/api/connect/ca.html#list-ca-root-certificates
    async fn ca_roots(&self, q: Option<&QueryOptions>) -> Result<(CARootList, QueryMeta)> {
        get("/v1/connect/ca/roots", &self.config, HashMap::new(), q).await
    }

    /// https://www.consul.io/api/connect/ca.html#get-ca-configuration
    async fn ca_get_config(&self, q: Option<&QueryOptions>) -> Result<(CAConfig, QueryMeta)> {
        get(
            "/v1/connect/ca/configuration",
            &self.config,
            HashMap::new(),
            q,
        )
        .await
    }

    /// https://www.consul.io/api/connect/ca.html#update-ca-configuration
    async fn ca_set_config(&self, conf: &CAConfig, q: Option<&WriteOptions>) -> Result<((), WriteMeta)> {
        put(
            "/v1/connect/ca/configuration",
            Some(Body::AsJson(conf)),
            &self.config,
            HashMap::new(),
            q,
        )
        .await
    }
}
