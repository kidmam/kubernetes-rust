/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiExtensionsV1beta1RollbackConfig : DEPRECATED.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiExtensionsV1beta1RollbackConfig {
  /// The revision to rollback to. If set to 0, rollback to the last revision.
  #[serde(rename = "revision")]
  revision: Option<i64>
}

impl IoK8sApiExtensionsV1beta1RollbackConfig {
  /// DEPRECATED.
  pub fn new() -> IoK8sApiExtensionsV1beta1RollbackConfig {
    IoK8sApiExtensionsV1beta1RollbackConfig {
      revision: None
    }
  }

  pub fn set_revision(&mut self, revision: i64) {
    self.revision = Some(revision);
  }

  pub fn with_revision(mut self, revision: i64) -> IoK8sApiExtensionsV1beta1RollbackConfig {
    self.revision = Some(revision);
    self
  }

  pub fn revision(&self) -> Option<&i64> {
    self.revision.as_ref()
  }

  pub fn reset_revision(&mut self) {
    self.revision = None;
  }

}


