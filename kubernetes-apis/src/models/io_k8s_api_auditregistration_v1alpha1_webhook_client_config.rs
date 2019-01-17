/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAuditregistrationV1alpha1WebhookClientConfig : WebhookClientConfig contains the information to make a connection with the webhook

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAuditregistrationV1alpha1WebhookClientConfig {
  /// `caBundle` is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. If unspecified, system trust roots on the apiserver are used.
  #[serde(rename = "caBundle")]
  ca_bundle: Option<String>,
  #[serde(rename = "service")]
  service: Option<::models::IoK8sApiAuditregistrationV1alpha1ServiceReference>,
  /// `url` gives the location of the webhook, in standard URL form (`scheme://host:port/path`). Exactly one of `url` or `service` must be specified.  The `host` should not refer to a service running in the cluster; use the `service` field instead. The host might be resolved via external DNS in some apiservers (e.g., `kube-apiserver` cannot resolve in-cluster DNS as that would be a layering violation). `host` may also be an IP address.  Please note that using `localhost` or `127.0.0.1` as a `host` is risky unless you take great care to run this webhook on all hosts which run an apiserver which might need to make calls to this webhook. Such installs are likely to be non-portable, i.e., not easy to turn up in a new cluster.  The scheme must be \"https\"; the URL must begin with \"https://\".  A path is optional, and if present may be any string permissible in a URL. You may use the path to pass an arbitrary string to the webhook, for example, a cluster identifier.  Attempting to use a user or basic auth e.g. \"user:password@\" is not allowed. Fragments (\"#...\") and query parameters (\"?...\") are not allowed, either.
  #[serde(rename = "url")]
  url: Option<String>
}

impl IoK8sApiAuditregistrationV1alpha1WebhookClientConfig {
  /// WebhookClientConfig contains the information to make a connection with the webhook
  pub fn new() -> IoK8sApiAuditregistrationV1alpha1WebhookClientConfig {
    IoK8sApiAuditregistrationV1alpha1WebhookClientConfig {
      ca_bundle: None,
      service: None,
      url: None
    }
  }

  pub fn set_ca_bundle(&mut self, ca_bundle: String) {
    self.ca_bundle = Some(ca_bundle);
  }

  pub fn with_ca_bundle(mut self, ca_bundle: String) -> IoK8sApiAuditregistrationV1alpha1WebhookClientConfig {
    self.ca_bundle = Some(ca_bundle);
    self
  }

  pub fn ca_bundle(&self) -> Option<&String> {
    self.ca_bundle.as_ref()
  }

  pub fn reset_ca_bundle(&mut self) {
    self.ca_bundle = None;
  }

  pub fn set_service(&mut self, service: ::models::IoK8sApiAuditregistrationV1alpha1ServiceReference) {
    self.service = Some(service);
  }

  pub fn with_service(mut self, service: ::models::IoK8sApiAuditregistrationV1alpha1ServiceReference) -> IoK8sApiAuditregistrationV1alpha1WebhookClientConfig {
    self.service = Some(service);
    self
  }

  pub fn service(&self) -> Option<&::models::IoK8sApiAuditregistrationV1alpha1ServiceReference> {
    self.service.as_ref()
  }

  pub fn reset_service(&mut self) {
    self.service = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> IoK8sApiAuditregistrationV1alpha1WebhookClientConfig {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}


