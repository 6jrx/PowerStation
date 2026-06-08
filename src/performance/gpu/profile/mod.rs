use std::io;

pub mod acpi;
pub mod platform;
pub mod ryzenadj;

#[derive(Debug)]
pub enum ProfileError {
    FeatureUnsupported,
    FailedOperation(String),
    InvalidArgument(String),
    IOError(String),
}

impl From<ProfileError> for String {
    fn from(_val: ProfileError) -> Self {
        todo!()
    }
}

impl From<io::Error> for ProfileError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

pub type ProfileResult<T> = Result<T, ProfileError>;
pub trait ProfileDevice: Sync + Send {
    async fn power_profile(&self) -> ProfileResult<String>;
    async fn power_profiles_available(&self) -> ProfileResult<Vec<String>>;
    async fn set_power_profile(&mut self, profile: String) -> ProfileResult<()>;
}
