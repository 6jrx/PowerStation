use std::sync::Arc;

use crate::performance::gpu::{
    gpu_device::{amd, intel, GPUDevice, GPUResult},
    monitor,
    profile::{self, ProfileDevice, ProfileResult},
    tdp::{self, TDPDevice, TDPResult},
};
use tokio::sync::Mutex;

#[allow(clippy::large_enum_variant)]
pub enum TDPDevices {
    Hwmon(tdp::hwmon::HwmonTdp),
    IntelRapl(tdp::intelrapl::IntelRaplTdp),
    RyzenAdj(tdp::ryzenadj::RyzenAdjTdp),
}

impl TDPDevices {
    pub async fn tdp(&self) -> TDPResult<f64> {
        match self {
            Self::Hwmon(dev) => dev.tdp().await,
            Self::IntelRapl(dev) => dev.tdp().await,
            Self::RyzenAdj(dev) => dev.tdp().await,
        }
    }

    pub async fn min_tdp(&self) -> TDPResult<f64> {
        match self {
            Self::Hwmon(dev) => dev.min_tdp().await,
            Self::IntelRapl(dev) => dev.min_tdp().await,
            Self::RyzenAdj(dev) => dev.min_tdp().await,
        }
    }

    pub async fn max_tdp(&self) -> TDPResult<f64> {
        match self {
            Self::Hwmon(dev) => dev.max_tdp().await,
            Self::IntelRapl(dev) => dev.max_tdp().await,
            Self::RyzenAdj(dev) => dev.max_tdp().await,
        }
    }

    pub async fn set_tdp(&mut self, value: f64) -> TDPResult<()> {
        match self {
            Self::Hwmon(dev) => dev.set_tdp(value).await,
            Self::IntelRapl(dev) => dev.set_tdp(value).await,
            Self::RyzenAdj(dev) => dev.set_tdp(value).await,
        }
    }

    pub async fn boost(&self) -> TDPResult<f64> {
        match self {
            Self::Hwmon(dev) => dev.boost().await,
            Self::IntelRapl(dev) => dev.boost().await,
            Self::RyzenAdj(dev) => dev.boost().await,
        }
    }

    pub async fn max_boost(&self) -> TDPResult<f64> {
        match self {
            Self::Hwmon(dev) => dev.max_boost().await,
            Self::IntelRapl(dev) => dev.max_boost().await,
            Self::RyzenAdj(dev) => dev.max_boost().await,
        }
    }

    pub async fn set_boost(&mut self, value: f64) -> TDPResult<()> {
        match self {
            Self::Hwmon(dev) => dev.set_boost(value).await,
            Self::IntelRapl(dev) => dev.set_boost(value).await,
            Self::RyzenAdj(dev) => dev.set_boost(value).await,
        }
    }

    pub async fn thermal_throttle_limit_c(&self) -> TDPResult<f64> {
        match self {
            Self::Hwmon(dev) => dev.thermal_throttle_limit_c().await,
            Self::IntelRapl(dev) => dev.thermal_throttle_limit_c().await,
            Self::RyzenAdj(dev) => dev.thermal_throttle_limit_c().await,
        }
    }

    pub async fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> TDPResult<()> {
        match self {
            Self::Hwmon(dev) => dev.set_thermal_throttle_limit_c(limit).await,
            Self::IntelRapl(dev) => dev.set_thermal_throttle_limit_c(limit).await,
            Self::RyzenAdj(dev) => dev.set_thermal_throttle_limit_c(limit).await,
        }
    }
}

pub enum ProfileDevices {
    RyzenAdj(profile::ryzenadj::RyzenAdjProfile),
}

impl ProfileDevices {
    pub async fn power_profile(&self) -> ProfileResult<String> {
        match self {
            Self::RyzenAdj(dev) => dev.power_profile().await,
        }
    }

    pub async fn set_power_profile(&mut self, profile: String) -> ProfileResult<()> {
        match self {
            Self::RyzenAdj(dev) => dev.set_power_profile(profile).await,
        }
    }

    pub async fn power_profiles_available(&self) -> ProfileResult<Vec<String>> {
        match self {
            Self::RyzenAdj(dev) => dev.power_profiles_available().await,
        }
    }
}

pub enum GPUDevices {
    AmdGpu(amd::AmdGpu),
    IntelGpu(intel::IntelGPU),
}

impl GPUDevices {
    pub async fn get_tdp_interface(&self) -> Option<Arc<Mutex<TDPDevices>>> {
        match self {
            Self::AmdGpu(dev) => dev.get_tdp_interface().await,
            Self::IntelGpu(dev) => dev.get_tdp_interface().await,
        }
    }

    pub async fn get_gpu_path(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.get_gpu_path().await,
            Self::IntelGpu(dev) => dev.get_gpu_path().await,
        }
    }

    pub async fn name(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.name().await,
            Self::IntelGpu(dev) => dev.name().await,
        }
    }

    pub async fn path(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.path().await,
            Self::IntelGpu(dev) => dev.path().await,
        }
    }

    pub async fn class(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.class().await,
            Self::IntelGpu(dev) => dev.class().await,
        }
    }

    pub async fn class_id(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.class_id().await,
            Self::IntelGpu(dev) => dev.class_id().await,
        }
    }

    pub async fn vendor(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.vendor().await,
            Self::IntelGpu(dev) => dev.vendor().await,
        }
    }

    pub async fn vendor_id(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.vendor_id().await,
            Self::IntelGpu(dev) => dev.vendor_id().await,
        }
    }

    pub async fn device(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.device().await,
            Self::IntelGpu(dev) => dev.device().await,
        }
    }

    pub async fn device_id(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.device_id().await,
            Self::IntelGpu(dev) => dev.device_id().await,
        }
    }

    pub async fn subdevice(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.subdevice().await,
            Self::IntelGpu(dev) => dev.subdevice().await,
        }
    }

    pub async fn subdevice_id(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.subdevice_id().await,
            Self::IntelGpu(dev) => dev.subdevice_id().await,
        }
    }

    pub async fn subvendor_id(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.subvendor_id().await,
            Self::IntelGpu(dev) => dev.subvendor_id().await,
        }
    }

    pub async fn revision_id(&self) -> String {
        match self {
            Self::AmdGpu(dev) => dev.revision_id().await,
            Self::IntelGpu(dev) => dev.revision_id().await,
        }
    }

    pub async fn clock_limit_mhz_min(&self) -> GPUResult<f64> {
        match self {
            Self::AmdGpu(dev) => dev.clock_limit_mhz_min().await,
            Self::IntelGpu(dev) => dev.clock_limit_mhz_min().await,
        }
    }

    pub async fn clock_limit_mhz_max(&self) -> GPUResult<f64> {
        match self {
            Self::AmdGpu(dev) => dev.clock_limit_mhz_max().await,
            Self::IntelGpu(dev) => dev.clock_limit_mhz_max().await,
        }
    }

    pub async fn clock_value_mhz_min(&self) -> GPUResult<f64> {
        match self {
            Self::AmdGpu(dev) => dev.clock_value_mhz_min().await,
            Self::IntelGpu(dev) => dev.clock_value_mhz_min().await,
        }
    }

    pub async fn set_clock_value_mhz_min(&mut self, value: f64) -> GPUResult<()> {
        match self {
            Self::AmdGpu(dev) => dev.set_clock_value_mhz_min(value).await,
            Self::IntelGpu(dev) => dev.set_clock_value_mhz_min(value).await,
        }
    }

    pub async fn clock_value_mhz_max(&self) -> GPUResult<f64> {
        match self {
            Self::AmdGpu(dev) => dev.clock_value_mhz_max().await,
            Self::IntelGpu(dev) => dev.clock_value_mhz_max().await,
        }
    }

    pub async fn set_clock_value_mhz_max(&mut self, value: f64) -> GPUResult<()> {
        match self {
            Self::AmdGpu(dev) => dev.set_clock_value_mhz_max(value).await,
            Self::IntelGpu(dev) => dev.set_clock_value_mhz_max(value).await,
        }
    }

    pub async fn manual_clock(&self) -> GPUResult<bool> {
        match self {
            Self::AmdGpu(dev) => dev.manual_clock().await,
            Self::IntelGpu(dev) => dev.manual_clock().await,
        }
    }

    pub async fn set_manual_clock(&mut self, enabled: bool) -> GPUResult<()> {
        match self {
            Self::AmdGpu(dev) => dev.set_manual_clock(enabled).await,
            Self::IntelGpu(dev) => dev.set_manual_clock(enabled).await,
        }
    }

    pub async fn get_gpu_busy_percent(&self) -> GPUResult<u8> {
        match self {
            Self::AmdGpu(dev) => dev.get_gpu_busy_percent().await,
            Self::IntelGpu(dev) => dev.get_gpu_busy_percent().await,
        }
    }
}

enum MonitorDevices {
    Intel(monitor::intel::IntelMonitorGPU),
}

impl MonitorDevices {}
