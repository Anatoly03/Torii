//! This module contains the localization API. It manages communicating with the
//! frontend as well as providing backend localization.

use serde_json::Value;

/// A localization file definition. These are stored in the resources directory of
/// the Torii application.
///
/// # Example
///
/// ```json
/// // resources/locales/jp.json
/// {
///     "locale-meta": {
///         "name": "Japanese",
///         "native-name": "日本語",
///         "direction": "ltr" // reserved
///     },
///     "app": {
///        "title": "トリイ",
///        "welcome": "ようこそ"
///     }
/// }
/// ```
pub struct LocaleManager {
    // TODO make serde connect to this
    value: Value,
}

impl LocaleManager {
    /// Reads the locale object and returns the the language name for this
    /// localization in English. For example, the English locale will return
    /// "English" and the Japanese locale will return "Japanese".
    pub fn get_locale_name(&self) -> Option<&str> {
        self.value
            .get("locale-meta")
            .and_then(|meta| meta.get("name"))
            .and_then(|name| name.as_str())
    }

    /// Reads the locale object and returns the the language name for this
    /// localization in the native language. For example, the English locale
    /// will return "English" and the Japanese locale will return "日本語".
    pub fn get_locale_native_name(&self) -> Option<&str> {
        self.value
            .get("locale-meta")
            .and_then(|meta| meta.get("name"))
            .and_then(|name| name.as_str())
    }
}
