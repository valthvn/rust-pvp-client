//! Construct [`AuthService`] in the composition layer and pass it down nothing
//! in here reaches for a global or a database
//! Accounts persist to `auth.json`

mod data;
mod diagnostics;
mod error;
mod msa;
mod offline;
mod service;
mod store;

pub use data::{
    AccountKind, BrowserLogin, DeviceCodeLogin, MicrosoftLoginSession, MinecraftAccount,
};
pub use diagnostics::{AuthErrorGuidance, AuthErrorSample, diagnose_auth_error, preview_samples};
pub use error::{AuthError, AuthResult, MinecraftAuthError, MinecraftAuthStep};
pub use msa::PendingBrowserLogin;
pub use offline::{offline_account, offline_uuid, validate_offline_username};
pub use service::{AuthService, MICROSOFT_LOGIN_PROGRESS};
pub use store::CredentialsStore;

/// The Ember launcher's offline developer mode: an offline account without any Microsoft account, playing
/// single player only. Only in a development build and with `EMBER_DEV_OFFLINE=1` (Horizon's
/// `HORIZON_LAUNCHER_DEVELOPER_OFFLINE` convention); a release, or a run without it, keeps upstream's rule.
pub fn dev_offline_allowed() -> bool {
    cfg!(debug_assertions) && std::env::var_os("EMBER_DEV_OFFLINE").as_deref() == Some("1".as_ref())
}
