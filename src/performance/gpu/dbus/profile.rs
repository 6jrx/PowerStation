use std::sync::Arc;
use zbus::fdo;
use zbus_macros::interface;

use tokio::sync::Mutex;

use crate::performance::gpu::{
    dbus::devices::ProfileDevices,
    profile::{ProfileError, ProfileResult},
};

pub struct GPUProfileDbusIface {
    dev: Arc<Mutex<ProfileDevices>>,
}

impl From<ProfileError> for fdo::Error {
    fn from(val: ProfileError) -> Self {
        match &val {
            ProfileError::FailedOperation(err) => fdo::Error::Failed(err.to_string()),
            ProfileError::FeatureUnsupported => {
                fdo::Error::Failed(String::from("Unsupported feature"))
            }
            ProfileError::InvalidArgument(err) => fdo::Error::Failed(err.to_string()),
            ProfileError::IOError(err) => fdo::Error::IOError(err.to_string()),
        }
    }
}

impl GPUProfileDbusIface {
    pub fn new(dev: Arc<Mutex<ProfileDevices>>) -> GPUProfileDbusIface {
        GPUProfileDbusIface { dev }
    }
}

#[interface(name = "org.shadowblip.GPU.Card.Platform")]
impl GPUProfileDbusIface {
    #[zbus(property)]
    async fn power_profile(&self) -> fdo::Result<String> {
        match self.dev.lock().await.power_profile().await {
            ProfileResult::Ok(result) => Ok(result),
            ProfileResult::Err(err) => Err(err.into()),
        }
    }

    #[zbus(property)]
    async fn set_power_profile(&mut self, profile: String) -> fdo::Result<()> {
        match self.dev.lock().await.set_power_profile(profile).await {
            ProfileResult::Ok(result) => Ok(result),
            ProfileResult::Err(err) => Err(err.into()),
        }
    }

    #[zbus(property)]
    async fn power_profiles_available(&self) -> fdo::Result<Vec<String>> {
        match self.dev.lock().await.power_profiles_available().await {
            ProfileResult::Ok(result) => Ok(result),
            ProfileResult::Err(err) => Err(err.into()),
        }
    }
}
