/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1Volume : Volume represents a named volume in a pod that may be accessed by any container in the pod.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1Volume {
  #[serde(rename = "awsElasticBlockStore")]
  aws_elastic_block_store: Option<::models::IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource>,
  #[serde(rename = "azureDisk")]
  azure_disk: Option<::models::IoK8sApiCoreV1AzureDiskVolumeSource>,
  #[serde(rename = "azureFile")]
  azure_file: Option<::models::IoK8sApiCoreV1AzureFileVolumeSource>,
  #[serde(rename = "cephfs")]
  cephfs: Option<::models::IoK8sApiCoreV1CephFsVolumeSource>,
  #[serde(rename = "cinder")]
  cinder: Option<::models::IoK8sApiCoreV1CinderVolumeSource>,
  #[serde(rename = "configMap")]
  config_map: Option<::models::IoK8sApiCoreV1ConfigMapVolumeSource>,
  #[serde(rename = "downwardAPI")]
  downward_api: Option<::models::IoK8sApiCoreV1DownwardApiVolumeSource>,
  #[serde(rename = "emptyDir")]
  empty_dir: Option<::models::IoK8sApiCoreV1EmptyDirVolumeSource>,
  #[serde(rename = "fc")]
  fc: Option<::models::IoK8sApiCoreV1FcVolumeSource>,
  #[serde(rename = "flexVolume")]
  flex_volume: Option<::models::IoK8sApiCoreV1FlexVolumeSource>,
  #[serde(rename = "flocker")]
  flocker: Option<::models::IoK8sApiCoreV1FlockerVolumeSource>,
  #[serde(rename = "gcePersistentDisk")]
  gce_persistent_disk: Option<::models::IoK8sApiCoreV1GcePersistentDiskVolumeSource>,
  #[serde(rename = "gitRepo")]
  git_repo: Option<::models::IoK8sApiCoreV1GitRepoVolumeSource>,
  #[serde(rename = "glusterfs")]
  glusterfs: Option<::models::IoK8sApiCoreV1GlusterfsVolumeSource>,
  #[serde(rename = "hostPath")]
  host_path: Option<::models::IoK8sApiCoreV1HostPathVolumeSource>,
  #[serde(rename = "iscsi")]
  iscsi: Option<::models::IoK8sApiCoreV1IscsiVolumeSource>,
  /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "nfs")]
  nfs: Option<::models::IoK8sApiCoreV1NfsVolumeSource>,
  #[serde(rename = "persistentVolumeClaim")]
  persistent_volume_claim: Option<::models::IoK8sApiCoreV1PersistentVolumeClaimVolumeSource>,
  #[serde(rename = "photonPersistentDisk")]
  photon_persistent_disk: Option<::models::IoK8sApiCoreV1PhotonPersistentDiskVolumeSource>,
  #[serde(rename = "portworxVolume")]
  portworx_volume: Option<::models::IoK8sApiCoreV1PortworxVolumeSource>,
  #[serde(rename = "projected")]
  projected: Option<::models::IoK8sApiCoreV1ProjectedVolumeSource>,
  #[serde(rename = "quobyte")]
  quobyte: Option<::models::IoK8sApiCoreV1QuobyteVolumeSource>,
  #[serde(rename = "rbd")]
  rbd: Option<::models::IoK8sApiCoreV1RbdVolumeSource>,
  #[serde(rename = "scaleIO")]
  scale_io: Option<::models::IoK8sApiCoreV1ScaleIoVolumeSource>,
  #[serde(rename = "secret")]
  secret: Option<::models::IoK8sApiCoreV1SecretVolumeSource>,
  #[serde(rename = "storageos")]
  storageos: Option<::models::IoK8sApiCoreV1StorageOsVolumeSource>,
  #[serde(rename = "vsphereVolume")]
  vsphere_volume: Option<::models::IoK8sApiCoreV1VsphereVirtualDiskVolumeSource>
}

impl IoK8sApiCoreV1Volume {
  /// Volume represents a named volume in a pod that may be accessed by any container in the pod.
  pub fn new(name: String) -> IoK8sApiCoreV1Volume {
    IoK8sApiCoreV1Volume {
      aws_elastic_block_store: None,
      azure_disk: None,
      azure_file: None,
      cephfs: None,
      cinder: None,
      config_map: None,
      downward_api: None,
      empty_dir: None,
      fc: None,
      flex_volume: None,
      flocker: None,
      gce_persistent_disk: None,
      git_repo: None,
      glusterfs: None,
      host_path: None,
      iscsi: None,
      name: name,
      nfs: None,
      persistent_volume_claim: None,
      photon_persistent_disk: None,
      portworx_volume: None,
      projected: None,
      quobyte: None,
      rbd: None,
      scale_io: None,
      secret: None,
      storageos: None,
      vsphere_volume: None
    }
  }

  pub fn set_aws_elastic_block_store(&mut self, aws_elastic_block_store: ::models::IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource) {
    self.aws_elastic_block_store = Some(aws_elastic_block_store);
  }

  pub fn with_aws_elastic_block_store(mut self, aws_elastic_block_store: ::models::IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource) -> IoK8sApiCoreV1Volume {
    self.aws_elastic_block_store = Some(aws_elastic_block_store);
    self
  }

  pub fn aws_elastic_block_store(&self) -> Option<&::models::IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource> {
    self.aws_elastic_block_store.as_ref()
  }

  pub fn reset_aws_elastic_block_store(&mut self) {
    self.aws_elastic_block_store = None;
  }

  pub fn set_azure_disk(&mut self, azure_disk: ::models::IoK8sApiCoreV1AzureDiskVolumeSource) {
    self.azure_disk = Some(azure_disk);
  }

  pub fn with_azure_disk(mut self, azure_disk: ::models::IoK8sApiCoreV1AzureDiskVolumeSource) -> IoK8sApiCoreV1Volume {
    self.azure_disk = Some(azure_disk);
    self
  }

  pub fn azure_disk(&self) -> Option<&::models::IoK8sApiCoreV1AzureDiskVolumeSource> {
    self.azure_disk.as_ref()
  }

  pub fn reset_azure_disk(&mut self) {
    self.azure_disk = None;
  }

  pub fn set_azure_file(&mut self, azure_file: ::models::IoK8sApiCoreV1AzureFileVolumeSource) {
    self.azure_file = Some(azure_file);
  }

  pub fn with_azure_file(mut self, azure_file: ::models::IoK8sApiCoreV1AzureFileVolumeSource) -> IoK8sApiCoreV1Volume {
    self.azure_file = Some(azure_file);
    self
  }

  pub fn azure_file(&self) -> Option<&::models::IoK8sApiCoreV1AzureFileVolumeSource> {
    self.azure_file.as_ref()
  }

  pub fn reset_azure_file(&mut self) {
    self.azure_file = None;
  }

  pub fn set_cephfs(&mut self, cephfs: ::models::IoK8sApiCoreV1CephFsVolumeSource) {
    self.cephfs = Some(cephfs);
  }

  pub fn with_cephfs(mut self, cephfs: ::models::IoK8sApiCoreV1CephFsVolumeSource) -> IoK8sApiCoreV1Volume {
    self.cephfs = Some(cephfs);
    self
  }

  pub fn cephfs(&self) -> Option<&::models::IoK8sApiCoreV1CephFsVolumeSource> {
    self.cephfs.as_ref()
  }

  pub fn reset_cephfs(&mut self) {
    self.cephfs = None;
  }

  pub fn set_cinder(&mut self, cinder: ::models::IoK8sApiCoreV1CinderVolumeSource) {
    self.cinder = Some(cinder);
  }

  pub fn with_cinder(mut self, cinder: ::models::IoK8sApiCoreV1CinderVolumeSource) -> IoK8sApiCoreV1Volume {
    self.cinder = Some(cinder);
    self
  }

  pub fn cinder(&self) -> Option<&::models::IoK8sApiCoreV1CinderVolumeSource> {
    self.cinder.as_ref()
  }

  pub fn reset_cinder(&mut self) {
    self.cinder = None;
  }

  pub fn set_config_map(&mut self, config_map: ::models::IoK8sApiCoreV1ConfigMapVolumeSource) {
    self.config_map = Some(config_map);
  }

  pub fn with_config_map(mut self, config_map: ::models::IoK8sApiCoreV1ConfigMapVolumeSource) -> IoK8sApiCoreV1Volume {
    self.config_map = Some(config_map);
    self
  }

  pub fn config_map(&self) -> Option<&::models::IoK8sApiCoreV1ConfigMapVolumeSource> {
    self.config_map.as_ref()
  }

  pub fn reset_config_map(&mut self) {
    self.config_map = None;
  }

  pub fn set_downward_api(&mut self, downward_api: ::models::IoK8sApiCoreV1DownwardApiVolumeSource) {
    self.downward_api = Some(downward_api);
  }

  pub fn with_downward_api(mut self, downward_api: ::models::IoK8sApiCoreV1DownwardApiVolumeSource) -> IoK8sApiCoreV1Volume {
    self.downward_api = Some(downward_api);
    self
  }

  pub fn downward_api(&self) -> Option<&::models::IoK8sApiCoreV1DownwardApiVolumeSource> {
    self.downward_api.as_ref()
  }

  pub fn reset_downward_api(&mut self) {
    self.downward_api = None;
  }

  pub fn set_empty_dir(&mut self, empty_dir: ::models::IoK8sApiCoreV1EmptyDirVolumeSource) {
    self.empty_dir = Some(empty_dir);
  }

  pub fn with_empty_dir(mut self, empty_dir: ::models::IoK8sApiCoreV1EmptyDirVolumeSource) -> IoK8sApiCoreV1Volume {
    self.empty_dir = Some(empty_dir);
    self
  }

  pub fn empty_dir(&self) -> Option<&::models::IoK8sApiCoreV1EmptyDirVolumeSource> {
    self.empty_dir.as_ref()
  }

  pub fn reset_empty_dir(&mut self) {
    self.empty_dir = None;
  }

  pub fn set_fc(&mut self, fc: ::models::IoK8sApiCoreV1FcVolumeSource) {
    self.fc = Some(fc);
  }

  pub fn with_fc(mut self, fc: ::models::IoK8sApiCoreV1FcVolumeSource) -> IoK8sApiCoreV1Volume {
    self.fc = Some(fc);
    self
  }

  pub fn fc(&self) -> Option<&::models::IoK8sApiCoreV1FcVolumeSource> {
    self.fc.as_ref()
  }

  pub fn reset_fc(&mut self) {
    self.fc = None;
  }

  pub fn set_flex_volume(&mut self, flex_volume: ::models::IoK8sApiCoreV1FlexVolumeSource) {
    self.flex_volume = Some(flex_volume);
  }

  pub fn with_flex_volume(mut self, flex_volume: ::models::IoK8sApiCoreV1FlexVolumeSource) -> IoK8sApiCoreV1Volume {
    self.flex_volume = Some(flex_volume);
    self
  }

  pub fn flex_volume(&self) -> Option<&::models::IoK8sApiCoreV1FlexVolumeSource> {
    self.flex_volume.as_ref()
  }

  pub fn reset_flex_volume(&mut self) {
    self.flex_volume = None;
  }

  pub fn set_flocker(&mut self, flocker: ::models::IoK8sApiCoreV1FlockerVolumeSource) {
    self.flocker = Some(flocker);
  }

  pub fn with_flocker(mut self, flocker: ::models::IoK8sApiCoreV1FlockerVolumeSource) -> IoK8sApiCoreV1Volume {
    self.flocker = Some(flocker);
    self
  }

  pub fn flocker(&self) -> Option<&::models::IoK8sApiCoreV1FlockerVolumeSource> {
    self.flocker.as_ref()
  }

  pub fn reset_flocker(&mut self) {
    self.flocker = None;
  }

  pub fn set_gce_persistent_disk(&mut self, gce_persistent_disk: ::models::IoK8sApiCoreV1GcePersistentDiskVolumeSource) {
    self.gce_persistent_disk = Some(gce_persistent_disk);
  }

  pub fn with_gce_persistent_disk(mut self, gce_persistent_disk: ::models::IoK8sApiCoreV1GcePersistentDiskVolumeSource) -> IoK8sApiCoreV1Volume {
    self.gce_persistent_disk = Some(gce_persistent_disk);
    self
  }

  pub fn gce_persistent_disk(&self) -> Option<&::models::IoK8sApiCoreV1GcePersistentDiskVolumeSource> {
    self.gce_persistent_disk.as_ref()
  }

  pub fn reset_gce_persistent_disk(&mut self) {
    self.gce_persistent_disk = None;
  }

  pub fn set_git_repo(&mut self, git_repo: ::models::IoK8sApiCoreV1GitRepoVolumeSource) {
    self.git_repo = Some(git_repo);
  }

  pub fn with_git_repo(mut self, git_repo: ::models::IoK8sApiCoreV1GitRepoVolumeSource) -> IoK8sApiCoreV1Volume {
    self.git_repo = Some(git_repo);
    self
  }

  pub fn git_repo(&self) -> Option<&::models::IoK8sApiCoreV1GitRepoVolumeSource> {
    self.git_repo.as_ref()
  }

  pub fn reset_git_repo(&mut self) {
    self.git_repo = None;
  }

  pub fn set_glusterfs(&mut self, glusterfs: ::models::IoK8sApiCoreV1GlusterfsVolumeSource) {
    self.glusterfs = Some(glusterfs);
  }

  pub fn with_glusterfs(mut self, glusterfs: ::models::IoK8sApiCoreV1GlusterfsVolumeSource) -> IoK8sApiCoreV1Volume {
    self.glusterfs = Some(glusterfs);
    self
  }

  pub fn glusterfs(&self) -> Option<&::models::IoK8sApiCoreV1GlusterfsVolumeSource> {
    self.glusterfs.as_ref()
  }

  pub fn reset_glusterfs(&mut self) {
    self.glusterfs = None;
  }

  pub fn set_host_path(&mut self, host_path: ::models::IoK8sApiCoreV1HostPathVolumeSource) {
    self.host_path = Some(host_path);
  }

  pub fn with_host_path(mut self, host_path: ::models::IoK8sApiCoreV1HostPathVolumeSource) -> IoK8sApiCoreV1Volume {
    self.host_path = Some(host_path);
    self
  }

  pub fn host_path(&self) -> Option<&::models::IoK8sApiCoreV1HostPathVolumeSource> {
    self.host_path.as_ref()
  }

  pub fn reset_host_path(&mut self) {
    self.host_path = None;
  }

  pub fn set_iscsi(&mut self, iscsi: ::models::IoK8sApiCoreV1IscsiVolumeSource) {
    self.iscsi = Some(iscsi);
  }

  pub fn with_iscsi(mut self, iscsi: ::models::IoK8sApiCoreV1IscsiVolumeSource) -> IoK8sApiCoreV1Volume {
    self.iscsi = Some(iscsi);
    self
  }

  pub fn iscsi(&self) -> Option<&::models::IoK8sApiCoreV1IscsiVolumeSource> {
    self.iscsi.as_ref()
  }

  pub fn reset_iscsi(&mut self) {
    self.iscsi = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> IoK8sApiCoreV1Volume {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_nfs(&mut self, nfs: ::models::IoK8sApiCoreV1NfsVolumeSource) {
    self.nfs = Some(nfs);
  }

  pub fn with_nfs(mut self, nfs: ::models::IoK8sApiCoreV1NfsVolumeSource) -> IoK8sApiCoreV1Volume {
    self.nfs = Some(nfs);
    self
  }

  pub fn nfs(&self) -> Option<&::models::IoK8sApiCoreV1NfsVolumeSource> {
    self.nfs.as_ref()
  }

  pub fn reset_nfs(&mut self) {
    self.nfs = None;
  }

  pub fn set_persistent_volume_claim(&mut self, persistent_volume_claim: ::models::IoK8sApiCoreV1PersistentVolumeClaimVolumeSource) {
    self.persistent_volume_claim = Some(persistent_volume_claim);
  }

  pub fn with_persistent_volume_claim(mut self, persistent_volume_claim: ::models::IoK8sApiCoreV1PersistentVolumeClaimVolumeSource) -> IoK8sApiCoreV1Volume {
    self.persistent_volume_claim = Some(persistent_volume_claim);
    self
  }

  pub fn persistent_volume_claim(&self) -> Option<&::models::IoK8sApiCoreV1PersistentVolumeClaimVolumeSource> {
    self.persistent_volume_claim.as_ref()
  }

  pub fn reset_persistent_volume_claim(&mut self) {
    self.persistent_volume_claim = None;
  }

  pub fn set_photon_persistent_disk(&mut self, photon_persistent_disk: ::models::IoK8sApiCoreV1PhotonPersistentDiskVolumeSource) {
    self.photon_persistent_disk = Some(photon_persistent_disk);
  }

  pub fn with_photon_persistent_disk(mut self, photon_persistent_disk: ::models::IoK8sApiCoreV1PhotonPersistentDiskVolumeSource) -> IoK8sApiCoreV1Volume {
    self.photon_persistent_disk = Some(photon_persistent_disk);
    self
  }

  pub fn photon_persistent_disk(&self) -> Option<&::models::IoK8sApiCoreV1PhotonPersistentDiskVolumeSource> {
    self.photon_persistent_disk.as_ref()
  }

  pub fn reset_photon_persistent_disk(&mut self) {
    self.photon_persistent_disk = None;
  }

  pub fn set_portworx_volume(&mut self, portworx_volume: ::models::IoK8sApiCoreV1PortworxVolumeSource) {
    self.portworx_volume = Some(portworx_volume);
  }

  pub fn with_portworx_volume(mut self, portworx_volume: ::models::IoK8sApiCoreV1PortworxVolumeSource) -> IoK8sApiCoreV1Volume {
    self.portworx_volume = Some(portworx_volume);
    self
  }

  pub fn portworx_volume(&self) -> Option<&::models::IoK8sApiCoreV1PortworxVolumeSource> {
    self.portworx_volume.as_ref()
  }

  pub fn reset_portworx_volume(&mut self) {
    self.portworx_volume = None;
  }

  pub fn set_projected(&mut self, projected: ::models::IoK8sApiCoreV1ProjectedVolumeSource) {
    self.projected = Some(projected);
  }

  pub fn with_projected(mut self, projected: ::models::IoK8sApiCoreV1ProjectedVolumeSource) -> IoK8sApiCoreV1Volume {
    self.projected = Some(projected);
    self
  }

  pub fn projected(&self) -> Option<&::models::IoK8sApiCoreV1ProjectedVolumeSource> {
    self.projected.as_ref()
  }

  pub fn reset_projected(&mut self) {
    self.projected = None;
  }

  pub fn set_quobyte(&mut self, quobyte: ::models::IoK8sApiCoreV1QuobyteVolumeSource) {
    self.quobyte = Some(quobyte);
  }

  pub fn with_quobyte(mut self, quobyte: ::models::IoK8sApiCoreV1QuobyteVolumeSource) -> IoK8sApiCoreV1Volume {
    self.quobyte = Some(quobyte);
    self
  }

  pub fn quobyte(&self) -> Option<&::models::IoK8sApiCoreV1QuobyteVolumeSource> {
    self.quobyte.as_ref()
  }

  pub fn reset_quobyte(&mut self) {
    self.quobyte = None;
  }

  pub fn set_rbd(&mut self, rbd: ::models::IoK8sApiCoreV1RbdVolumeSource) {
    self.rbd = Some(rbd);
  }

  pub fn with_rbd(mut self, rbd: ::models::IoK8sApiCoreV1RbdVolumeSource) -> IoK8sApiCoreV1Volume {
    self.rbd = Some(rbd);
    self
  }

  pub fn rbd(&self) -> Option<&::models::IoK8sApiCoreV1RbdVolumeSource> {
    self.rbd.as_ref()
  }

  pub fn reset_rbd(&mut self) {
    self.rbd = None;
  }

  pub fn set_scale_io(&mut self, scale_io: ::models::IoK8sApiCoreV1ScaleIoVolumeSource) {
    self.scale_io = Some(scale_io);
  }

  pub fn with_scale_io(mut self, scale_io: ::models::IoK8sApiCoreV1ScaleIoVolumeSource) -> IoK8sApiCoreV1Volume {
    self.scale_io = Some(scale_io);
    self
  }

  pub fn scale_io(&self) -> Option<&::models::IoK8sApiCoreV1ScaleIoVolumeSource> {
    self.scale_io.as_ref()
  }

  pub fn reset_scale_io(&mut self) {
    self.scale_io = None;
  }

  pub fn set_secret(&mut self, secret: ::models::IoK8sApiCoreV1SecretVolumeSource) {
    self.secret = Some(secret);
  }

  pub fn with_secret(mut self, secret: ::models::IoK8sApiCoreV1SecretVolumeSource) -> IoK8sApiCoreV1Volume {
    self.secret = Some(secret);
    self
  }

  pub fn secret(&self) -> Option<&::models::IoK8sApiCoreV1SecretVolumeSource> {
    self.secret.as_ref()
  }

  pub fn reset_secret(&mut self) {
    self.secret = None;
  }

  pub fn set_storageos(&mut self, storageos: ::models::IoK8sApiCoreV1StorageOsVolumeSource) {
    self.storageos = Some(storageos);
  }

  pub fn with_storageos(mut self, storageos: ::models::IoK8sApiCoreV1StorageOsVolumeSource) -> IoK8sApiCoreV1Volume {
    self.storageos = Some(storageos);
    self
  }

  pub fn storageos(&self) -> Option<&::models::IoK8sApiCoreV1StorageOsVolumeSource> {
    self.storageos.as_ref()
  }

  pub fn reset_storageos(&mut self) {
    self.storageos = None;
  }

  pub fn set_vsphere_volume(&mut self, vsphere_volume: ::models::IoK8sApiCoreV1VsphereVirtualDiskVolumeSource) {
    self.vsphere_volume = Some(vsphere_volume);
  }

  pub fn with_vsphere_volume(mut self, vsphere_volume: ::models::IoK8sApiCoreV1VsphereVirtualDiskVolumeSource) -> IoK8sApiCoreV1Volume {
    self.vsphere_volume = Some(vsphere_volume);
    self
  }

  pub fn vsphere_volume(&self) -> Option<&::models::IoK8sApiCoreV1VsphereVirtualDiskVolumeSource> {
    self.vsphere_volume.as_ref()
  }

  pub fn reset_vsphere_volume(&mut self) {
    self.vsphere_volume = None;
  }

}


