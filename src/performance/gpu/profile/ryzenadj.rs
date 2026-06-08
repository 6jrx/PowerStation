#![cfg(target_arch = "x86_64")]
use std::error::Error;

use libryzenadj::RyzenAdj;

use crate::performance::gpu::profile::{ProfileDevice, ProfileError, ProfileResult};

/// Implementation of Platform Profiles for AMD GPUs
pub struct RyzenAdjProfile {
    pub device_id: String,
    pub profile: String,
    ryzenadj: RyzenAdj,
}

unsafe impl Sync for RyzenAdjProfile {} // implementor (RyzenAdj) may be unsafe
unsafe impl Send for RyzenAdjProfile {} // implementor (RyzenAdj) may be unsafe

impl RyzenAdjProfile {
    /// Create a new Profile instance
    pub fn new(_path: String, device_id: String) -> Result<RyzenAdjProfile, Box<dyn Error>> {
        // Currently there is no known way to read this value
        let profile = String::from("low-power");
        let ryzenadj = RyzenAdj::new().map_err(|err| err.to_string())?;

        Ok(RyzenAdjProfile {
            device_id,
            profile,
            ryzenadj,
        })
    }

    /// Set the power profile to the given profile
    fn set_power_profile(&self, profile: String) -> Result<(), String> {
        log::debug!("Setting power profile");
        match profile.as_str() {
            "low-power" => self
                .ryzenadj
                .set_power_saving()
                .map_err(|err| err.to_string()),
            "performance" => self
                .ryzenadj
                .set_max_performance()
                .map_err(|err| err.to_string()),
            _ => Err(String::from(
                "Invalid power profile. Must be in [performance, low-power]",
            )),
        }
    }
}

impl ProfileDevice for RyzenAdjProfile {
    async fn power_profile(&self) -> ProfileResult<String> {
        Ok(self.profile.clone())
    }

    async fn set_power_profile(&mut self, profile: String) -> ProfileResult<()> {
        log::debug!("Setting power profile to: {}", profile);
        RyzenAdjProfile::set_power_profile(self, profile.clone())
            .map_err(|err| ProfileError::FailedOperation(err.to_string()))?;
        self.profile = profile;
        Ok(())
    }

    async fn power_profiles_available(&self) -> ProfileResult<Vec<String>> {
        Ok(vec![
            "max-performance".to_string(),
            "power-saving".to_string(),
        ])
    }
}
