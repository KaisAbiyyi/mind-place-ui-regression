use gpui::{AssetSource, Result, SharedString};
use std::borrow::Cow;

pub struct EmbeddedAssets;

const BRAND_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <circle cx="12" cy="12" r="2.2" fill="currentColor"/>
  <circle cx="12" cy="3.5" r="1.4" fill="currentColor"/>
  <circle cx="19.2" cy="7.5" r="1.4" fill="currentColor"/>
  <circle cx="19.2" cy="16.5" r="1.4" fill="currentColor"/>
  <circle cx="12" cy="20.5" r="1.4" fill="currentColor"/>
  <circle cx="4.8" cy="16.5" r="1.4" fill="currentColor"/>
  <circle cx="4.8" cy="7.5" r="1.4" fill="currentColor"/>
  <path d="M12 5v4.8m5.9-1.3-4.1 2.4m4.1 5.1-4.1-2.4M12 19v-4.8m-5.9 1.3 4.1-2.4m-4.1-5.1 4.1 2.4"/>
</svg>"#;

const COLLAPSE_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <rect x="3" y="3" width="18" height="18" rx="2"/>
  <path d="M9 3v18"/>
</svg>"#;

const NAV_HOME_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <rect x="3" y="3" width="7" height="7" rx="1.5"/>
  <rect x="14" y="3" width="7" height="7" rx="1.5"/>
  <rect x="14" y="14" width="7" height="7" rx="1.5"/>
  <rect x="3" y="14" width="7" height="7" rx="1.5"/>
</svg>"#;

const NAV_CONVERSATIONS_SVG: &[u8] = br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <path d="M4 5.5A2.5 2.5 0 0 1 6.5 3h11A2.5 2.5 0 0 1 20 5.5v8a2.5 2.5 0 0 1-2.5 2.5H10l-5 4v-4.6A2.5 2.5 0 0 1 4 13V5.5Z"/>
  <path d="M8 8h8M8 11.5h5"/>
</svg>"#;

const NAV_MEMORY_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <path d="m12 3 8 4.5-8 4.5-8-4.5L12 3Z"/>
  <path d="m4 12 8 4.5 8-4.5M4 16.5 12 21l8-4.5"/>
</svg>"#;

const NAV_HIVE_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <circle cx="12" cy="12" r="2.2" fill="currentColor"/>
  <circle cx="5" cy="6" r="1.5" fill="currentColor"/>
  <circle cx="19" cy="5" r="1.5" fill="currentColor"/>
  <circle cx="19" cy="18" r="1.5" fill="currentColor"/>
  <circle cx="5" cy="19" r="1.5" fill="currentColor"/>
  <path d="m6.2 7.1 4.1 3.6m3.3-.1 4.2-4.4m.1 6.8-4.3 2.7m-3.3.1-4.1 2.7"/>
</svg>"#;

const NAV_FILES_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <path d="M4 7a2 2 0 0 1 2-2h4l2 2h6a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V7Z"/>
  <path d="M4 10h16"/>
</svg>"#;

const NAV_PROVIDERS_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <path d="M9 15 15 9"/>
  <path d="M7 17H6a4 4 0 0 1 0-8h4m4-2h4a4 4 0 0 1 0 8h-1"/>
</svg>"#;

const NAV_SETUP_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <circle cx="12" cy="12" r="8.5"/>
  <path d="M12 7v5l3 2"/>
</svg>"#;

const NAV_SETTINGS_SVG: &[u8] = br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <circle cx="12" cy="12" r="3"/>
  <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1Z"/>
</svg>"#;

const SEARCH_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="11" cy="11" r="7"/>
  <path d="m16.5 16.5 4.5 4.5"/>
</svg>"#;

const THEME_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>
</svg>"#;

const ASK_AI_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <path d="M12 3v3m0 12v3M3 12h3m12 0h3"/>
  <circle cx="12" cy="12" r="4"/>
</svg>"#;

const LOCK_SVG: &[u8] =
    br#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
  <rect x="4" y="11" width="16" height="10" rx="2"/>
  <path d="M8 11V7a4 4 0 0 1 8 0v4"/>
</svg>"#;

impl AssetSource for EmbeddedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let bytes: Option<&'static [u8]> = match path {
            "icons/brand.svg" => Some(BRAND_SVG),
            "icons/collapse.svg" => Some(COLLAPSE_SVG),
            "icons/nav_home.svg" => Some(NAV_HOME_SVG),
            "icons/nav_conversations.svg" => Some(NAV_CONVERSATIONS_SVG),
            "icons/nav_memory.svg" => Some(NAV_MEMORY_SVG),
            "icons/nav_hive.svg" => Some(NAV_HIVE_SVG),
            "icons/nav_files.svg" => Some(NAV_FILES_SVG),
            "icons/nav_providers.svg" => Some(NAV_PROVIDERS_SVG),
            "icons/nav_setup.svg" => Some(NAV_SETUP_SVG),
            "icons/nav_settings.svg" => Some(NAV_SETTINGS_SVG),
            "icons/search.svg" => Some(SEARCH_SVG),
            "icons/theme.svg" => Some(THEME_SVG),
            "icons/ask_ai.svg" => Some(ASK_AI_SVG),
            "icons/lock.svg" => Some(LOCK_SVG),
            _ => None,
        };
        Ok(bytes.map(Cow::Borrowed))
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![
            "icons/brand.svg".into(),
            "icons/collapse.svg".into(),
            "icons/nav_home.svg".into(),
            "icons/nav_conversations.svg".into(),
            "icons/nav_memory.svg".into(),
            "icons/nav_hive.svg".into(),
            "icons/nav_files.svg".into(),
            "icons/nav_providers.svg".into(),
            "icons/nav_setup.svg".into(),
            "icons/nav_settings.svg".into(),
            "icons/search.svg".into(),
            "icons/theme.svg".into(),
            "icons/ask_ai.svg".into(),
            "icons/lock.svg".into(),
        ])
    }
}
