use core::option::Option::{None, Some};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::performance::gpu::{
    database::tdp_limits::TdpLimits,
    tdp::{HardwareAccess, TDPDevice, TDPError, TDPResult},
};

/// Implementation of TDP control for Intel GPUs
pub struct IntelRaplTdp {
    //pub path: String,
    hardware: Option<TdpLimits>,
    base_path: Option<PathBuf>,
}

impl HardwareAccess for IntelRaplTdp {
    fn hardware(&self) -> Option<&TdpLimits> {
        self.hardware.as_ref()
    }
}

impl IntelRaplTdp {
    pub fn new(_path: String) -> IntelRaplTdp {
        let hardware = match TdpLimits::new() {
            Some(hardware) => {
                log::info!("Found Hardware interface for TDP control");
                Some(hardware)
            }
            None => None,
        };

        // Discover the package domain path
        let mut base_path = None;
        if let Ok(mut rapl_dir) = fs::read_dir("/sys/class/powercap/intel-rapl") {
            while let Some(Ok(entry)) = rapl_dir.next() {
                let Ok(file_type) = entry.file_type() else {
                    continue;
                };
                if !file_type.is_dir() {
                    continue;
                }
                let file_name = entry.file_name();
                let Some(file_name) = file_name.to_str() else {
                    continue;
                };
                if !file_name.starts_with("intel-rapl:") {
                    continue;
                }
                let domain_path = entry.path();
                let name_path = domain_path.join("name");
                let Ok(name) = fs::read_to_string(name_path) else {
                    continue;
                };
                if !name.as_str().trim().starts_with("package") {
                    continue;
                }
                base_path = Some(domain_path);
                break;
            }
        }

        IntelRaplTdp {
            hardware,
            base_path,
        }
    }
}

impl TDPDevice for IntelRaplTdp {
    async fn tdp(&self) -> TDPResult<f64> {
        let Some(base_path) = self.base_path.as_ref() else {
            return Err(TDPError::FeatureUnsupported);
        };
        let path = base_path.join("constraint_0_power_limit_uw");
        let result = fs::read_to_string(path);
        let content = result.map_err(|err| TDPError::IOError(err.to_string()))?;
        let content = content.trim();

        // Parse the output to get the long TDP
        let long_tdp = match content.parse::<f64>() {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                return Err(TDPError::FailedOperation(e.to_string()));
            }
        };

        Ok(long_tdp / 1000000.0)
    }

    async fn set_tdp(&mut self, value: f64) -> TDPResult<()> {
        let Some(base_path) = self.base_path.as_ref() else {
            return Err(TDPError::FeatureUnsupported);
        };
        if value < 1.0 {
            let err = "Cowardly refusing to set TDP less than 1";
            log::warn!("{}", err);
            return Err(TDPError::InvalidArgument(String::from(err)));
        }

        // Get the current boost value so the peak tdp can be set *boost*
        // distance away.
        let mut boost = self.boost().await?;
        if boost < 0.0 {
            log::warn!("Boost is less than 0, setting to 0");
            boost = 0.0;
        }

        // Open the sysfs file to write to
        let path = base_path.join("constraint_0_power_limit_uw");
        let file = OpenOptions::new().write(true).open(path);

        // Convert the value to a writable string
        let value = format!("{}", value * 1000000.0);

        // Write the value
        file.map_err(|err| TDPError::FailedOperation(err.to_string()))?
            .write_all(value.as_bytes())
            .map_err(|err| TDPError::IOError(err.to_string()))?;

        // Update the boost value
        self.set_boost(boost).await
    }

    async fn boost(&self) -> TDPResult<f64> {
        let Some(base_path) = self.base_path.as_ref() else {
            return Err(TDPError::FeatureUnsupported);
        };
        let path = base_path.join("constraint_1_power_limit_uw");
        let result = fs::read_to_string(path);
        let content = result.map_err(|err| TDPError::IOError(err.to_string()))?;
        let content = content.trim();

        // Parse the output to get the peak TDP
        let peak_tdp = match content.parse::<f64>() {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                return Err(TDPError::FailedOperation(e.to_string()));
            }
        };

        let tdp = self.tdp().await?;
        Ok((peak_tdp / 1000000.0) - tdp)
    }

    async fn set_boost(&mut self, value: f64) -> TDPResult<()> {
        let Some(base_path) = self.base_path.as_ref() else {
            return Err(TDPError::FeatureUnsupported);
        };
        log::debug!("Setting Boost: {}", value);
        if value < 0.0 {
            let err = "Cowardly refusing to set TDP Boost less than 0";
            log::warn!("{}", err);
            return Err(TDPError::InvalidArgument(String::from(err)));
        }

        let tdp = self.tdp().await?;
        let boost = value;
        let short_tdp = if boost > 0.0 {
            (boost + tdp) * 1000000.0
        } else {
            tdp * 1000000.0
        };

        // Write the short tdp
        let path = base_path.join("constraint_1_power_limit_uw");
        let file = OpenOptions::new().write(true).open(path);
        let value = format!("{}", short_tdp);
        file.map_err(|err| TDPError::FailedOperation(err.to_string()))?
            .write_all(value.as_bytes())
            .map_err(|err| TDPError::IOError(err.to_string()))
    }

    async fn thermal_throttle_limit_c(&self) -> TDPResult<f64> {
        log::error!("Thermal throttling not supported on intel gpu");
        Err(TDPError::FeatureUnsupported)
    }

    async fn set_thermal_throttle_limit_c(&mut self, _limit: f64) -> TDPResult<()> {
        log::error!("Thermal throttling not supported on intel gpu");
        Err(TDPError::FeatureUnsupported)
    }
}
