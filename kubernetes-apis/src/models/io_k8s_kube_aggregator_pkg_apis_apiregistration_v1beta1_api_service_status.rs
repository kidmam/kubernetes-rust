/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceStatus : APIServiceStatus contains derived information about an API server

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceStatus {
  /// Current service state of apiService.
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceCondition>>
}

impl IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceStatus {
  /// APIServiceStatus contains derived information about an API server
  pub fn new() -> IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceStatus {
    IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceStatus {
      conditions: None
    }
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceCondition>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceCondition>) -> IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceStatus {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::IoK8sKubeAggregatorPkgApisApiregistrationV1beta1ApiServiceCondition>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

}


