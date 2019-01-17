/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiExtensionsV1beta1NetworkPolicyPeer : DEPRECATED 1.9 - This group version of NetworkPolicyPeer is deprecated by networking/v1/NetworkPolicyPeer.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiExtensionsV1beta1NetworkPolicyPeer {
  #[serde(rename = "ipBlock")]
  ip_block: Option<::models::IoK8sApiExtensionsV1beta1IpBlock>,
  #[serde(rename = "namespaceSelector")]
  namespace_selector: Option<::models::IoK8sApimachineryPkgApisMetaV1LabelSelector>,
  #[serde(rename = "podSelector")]
  pod_selector: Option<::models::IoK8sApimachineryPkgApisMetaV1LabelSelector>
}

impl IoK8sApiExtensionsV1beta1NetworkPolicyPeer {
  /// DEPRECATED 1.9 - This group version of NetworkPolicyPeer is deprecated by networking/v1/NetworkPolicyPeer.
  pub fn new() -> IoK8sApiExtensionsV1beta1NetworkPolicyPeer {
    IoK8sApiExtensionsV1beta1NetworkPolicyPeer {
      ip_block: None,
      namespace_selector: None,
      pod_selector: None
    }
  }

  pub fn set_ip_block(&mut self, ip_block: ::models::IoK8sApiExtensionsV1beta1IpBlock) {
    self.ip_block = Some(ip_block);
  }

  pub fn with_ip_block(mut self, ip_block: ::models::IoK8sApiExtensionsV1beta1IpBlock) -> IoK8sApiExtensionsV1beta1NetworkPolicyPeer {
    self.ip_block = Some(ip_block);
    self
  }

  pub fn ip_block(&self) -> Option<&::models::IoK8sApiExtensionsV1beta1IpBlock> {
    self.ip_block.as_ref()
  }

  pub fn reset_ip_block(&mut self) {
    self.ip_block = None;
  }

  pub fn set_namespace_selector(&mut self, namespace_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) {
    self.namespace_selector = Some(namespace_selector);
  }

  pub fn with_namespace_selector(mut self, namespace_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sApiExtensionsV1beta1NetworkPolicyPeer {
    self.namespace_selector = Some(namespace_selector);
    self
  }

  pub fn namespace_selector(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1LabelSelector> {
    self.namespace_selector.as_ref()
  }

  pub fn reset_namespace_selector(&mut self) {
    self.namespace_selector = None;
  }

  pub fn set_pod_selector(&mut self, pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) {
    self.pod_selector = Some(pod_selector);
  }

  pub fn with_pod_selector(mut self, pod_selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sApiExtensionsV1beta1NetworkPolicyPeer {
    self.pod_selector = Some(pod_selector);
    self
  }

  pub fn pod_selector(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1LabelSelector> {
    self.pod_selector.as_ref()
  }

  pub fn reset_pod_selector(&mut self) {
    self.pod_selector = None;
  }

}


