use crate::watchers::{WatchConfigError, Watcher};
use near_o11y::{reload_log_layer, ReloadError};
use serde::{Deserialize, Serialize};

/// Configures logging.
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub(crate) struct LogConfig {
    /// Comma-separated list of EnvFitler directives.
    pub rust_log: Option<String>,
    /// Some("") enables global debug logging.
    /// Some("module") enables debug logging for "module".
    pub verbose_module: Option<String>,
}

impl Watcher for LogConfig {
    fn reload(instance: Option<Self>) -> Result<(), WatchConfigError> {
        if let Some(LogConfig { rust_log, verbose_module }) = instance {
            Ok(reload_log_layer(rust_log.as_deref(), verbose_module.as_deref())
                .map_err(|_e| into_config_err())?)
        } else {
            Ok(reload_log_layer(None, None).map_err(|_e| into_config_err())?)
        }
    }
}

fn into_config_err() -> WatchConfigError {
    WatchConfigError::Reload("Reload log config error".into())
}
