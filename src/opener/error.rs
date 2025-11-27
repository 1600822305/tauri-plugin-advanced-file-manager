// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("unknown program {0}")]
    UnknownProgramName(String),
    #[error("Not allowed to open path {}{}", .path, .with.as_ref().map(|w| format!(" with {w}")).unwrap_or_default())]
    ForbiddenPath { path: String, with: Option<String> },
    #[error("Not allowed to open url {}{}", .url, .with.as_ref().map(|w| format!(" with {w}")).unwrap_or_default())]
    ForbiddenUrl { url: String, with: Option<String> },
    #[error("API not supported on the current platform")]
    UnsupportedPlatform,
    #[error(transparent)]
    #[cfg(windows)]
    Win32Error(#[from] windows::core::Error),
    #[error("Path doesn't have a parent: {0}")]
    NoParent(PathBuf),
    #[cfg(windows)]
    #[error("Failed to convert path '{0}' to ITEMIDLIST")]
    FailedToConvertPathToItemIdList(PathBuf),
    #[error("Failed to convert path to file:// url")]
    FailedToConvertPathToFileUrl,
    #[error(transparent)]
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    Zbus(#[from] zbus::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// 添加与主模块 Error 的转换
impl From<crate::Error> for Error {
    fn from(err: crate::Error) -> Self {
        match err {
            crate::Error::Io(e) => Error::Io(e),
            crate::Error::Json(e) => Error::Json(e),
            crate::Error::Tauri(e) => Error::Tauri(e),
            _ => Error::Io(std::io::Error::new(std::io::ErrorKind::Other, err.to_string())),
        }
    }
}
