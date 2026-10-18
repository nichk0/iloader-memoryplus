use std::{path::PathBuf, sync::Mutex};

use crate::{
    device::{DeviceInfoMutex, get_provider, get_provider_from_connection, get_usbmuxd},
    error::AppError,
    operation::Operation,
    pairing::{get_sidestore_info, place_file},
};
use isideload::{
    sideload::{application::SpecialApp, sideloader::Sideloader},
    util::callbacks::MaxCertsCallbackBox,
};
use tauri::{AppHandle, Manager, State, Window};

pub type SideloaderMutex = Mutex<Option<Sideloader<MaxCertsCallbackBox>>>;

pub struct SideloaderGuard<'a> {
    state: &'a SideloaderMutex,
    sideloader: Option<Sideloader<MaxCertsCallbackBox>>,
}

impl<'a> SideloaderGuard<'a> {
    pub fn take(state: &'a SideloaderMutex) -> Result<Self, AppError> {
        let mut guard = state.lock().unwrap();
        let sideloader = guard.take().ok_or(AppError::NotLoggedIn)?;
        Ok(Self {
            state,
            sideloader: Some(sideloader),
        })
    }

    pub fn get_mut(&mut self) -> &mut Sideloader<MaxCertsCallbackBox> {
        self.sideloader
            .as_mut()
            .expect("Sideloader should be present")
    }
}

impl Drop for SideloaderGuard<'_> {
    fn drop(&mut self) {
        let mut guard = self.state.lock().unwrap();
        *guard = self.sideloader.take();
    }
}

pub async fn sideload(
    device_state: State<'_, DeviceInfoMutex>,
    sideloader_state: State<'_, SideloaderMutex>,
    app_path: String,
) -> Result<Option<SpecialApp>, AppError> {
    let device = {
        let device_lock = device_state.lock().unwrap();
        match &*device_lock {
            Some(d) => d.clone(),
            None => return Err(AppError::NoDeviceSelected),
        }
    };

    let provider = get_provider(&device.info).await?;

    let mut sideloader = SideloaderGuard::take(&sideloader_state)?;

    let special = sideloader
        .get_mut()
        .install_app(
            &provider,
            app_path.into(),
            // MemoryPlus: request "Increased Memory Limit" for every installed app
            true,
            None::<fn(f32) -> std::future::Ready<()>>,
            Some(&device.pairing),
        )
        .await?;

    Ok(special)
}

#[tauri::command]
pub async fn sideload_operation(
    window: Window,
    device_state: State<'_, DeviceInfoMutex>,
    sideloader_state: State<'_, SideloaderMutex>,
    app_path: String,
) -> Result<(), AppError> {
    let op = Operation::new("sideload".to_string(), &window);
    op.start("install")?;
    op.fail_if_err(
        "install",
        sideload(device_state, sideloader_state, app_path).await,
    )?;
    op.complete("install")?;
    Ok(())
}

#[tauri::command]
pub async fn install_sidestore_operation(
    handle: AppHandle,
    window: Window,
    device_state: State<'_, DeviceInfoMutex>,
    sideloader_state: State<'_, SideloaderMutex>,
    nightly: bool,
    live_container: bool,
) -> Result<(), AppError> {
    let op = Operation::new("install_sidestore".to_string(), &window);
    op.start("download")?;
    // TODO: Cache & check version to avoid re-downloading
    let (filename, url) = if live_container {
        if nightly {
            (
                "LiveContainerSideStore-Nightly.ipa",
                "https://github.com/LiveContainer/LiveContainer/releases/download/nightly/LiveContainer+SideStore.ipa",
            )
        } else {
            (
                "LiveContainerSideStore.ipa",
                "https://github.com/LiveContainer/LiveContainer/releases/latest/download/LiveContainer+SideStore.ipa",
            )
        }
    } else if nightly {
        (
            "SideStore-Nightly.ipa",
            "https://github.com/SideStore/SideStore/releases/download/nightly/SideStore.ipa",
        )
    } else {
        (
            "SideStore.ipa",
            "https://github.com/SideStore/SideStore/releases/latest/download/SideStore.ipa",
        )
    };

    let dest = handle
        .path()
        .temp_dir()
        .map_err(|e| AppError::Filesystem("Failed to get temp dir".into(), e.to_string()))?
        .join(filename);
    op.fail_if_err("download", download(url, &dest).await)?;
    op.move_on("download", "install")?;
    let device = {
        let device_guard = device_state.lock().unwrap();
        match &*device_guard {
            Some(d) => d.clone(),
            None => return op.fail("install", AppError::NoDeviceSelected),
        }
    };
    op.fail_if_err(
        "install",
        sideload(
            device_state,
            sideloader_state,
            dest.to_string_lossy().to_string(),
        )
        .await,
    )?;
    op.move_on("install", "pairing")?;
    let sidestore_info = op.fail_if_err(
        "pairing",
        get_sidestore_info(&device.info, live_container).await,
    )?;
    if let Some(info) = sidestore_info {
        let mut usbmuxd = op.fail_if_err("pairing", get_usbmuxd().await)?;

        let provider = op.fail_if_err(
            "pairing",
            get_provider_from_connection(&device.info, &mut usbmuxd).await,
        )?;

        op.fail_if_err(
            "pairing",
            place_file(device.pairing, &provider, info.bundle_id, info.path).await,
        )?;
    } else {
        return op.fail(
            "pairing",
            AppError::HouseArrest(
                "SideStore's not found".into(),
                "The device did not report SideStore's bundle ID as installed".into(),
            ),
        );
    }

    op.complete("pairing")?;
    Ok(())
}

pub async fn download(url: impl AsRef<str>, dest: &PathBuf) -> Result<(), AppError> {
    let response = reqwest::get(url.as_ref())
        .await
        .map_err(|e| AppError::Download(e.to_string()))?;
    if !response.status().is_success() {
        return Err(AppError::Download(format!(
            "Failed to download file: HTTP {}",
            response.status()
        )));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| AppError::Download(e.to_string()))?;
    tokio::fs::write(dest, &bytes).await.map_err(|e| {
        AppError::Filesystem("Failed to write downloaded file".into(), e.to_string())
    })?;

    Ok(())
}

// MemoryPlus: one-click install of the latest official Madeira release
// (https://github.com/willfaust/Madeira), signed with Increased Memory Limit.

const MADEIRA_LATEST_RELEASE: &str =
    "https://api.github.com/repos/willfaust/Madeira/releases/latest";

/// Folder (inside the system temp dir) that holds the extracted Madeira bundle.
/// Madeira ships Wine's Windows DLLs, which antivirus software often flags as a
/// false positive, so this is the one folder users may need to exclude.
pub const MADEIRA_WORK_DIR: &str = "iloader-memoryplus-madeira";

#[tauri::command]
pub fn madeira_work_dir() -> String {
    std::env::temp_dir()
        .join(MADEIRA_WORK_DIR)
        .to_string_lossy()
        .to_string()
}

#[tauri::command]
pub async fn install_madeira_operation(
    window: Window,
    device_state: State<'_, DeviceInfoMutex>,
    sideloader_state: State<'_, SideloaderMutex>,
) -> Result<(), AppError> {
    let op = Operation::new("install_madeira".to_string(), &window);
    op.start("download")?;
    let ipa = op.fail_if_err("download", download_latest_madeira().await)?;

    op.move_on("download", "extract")?;
    let work_dir = std::env::temp_dir().join(MADEIRA_WORK_DIR);
    let extract_dir = work_dir.clone();
    let app_dir = op.fail_if_err(
        "extract",
        tokio::task::spawn_blocking(move || extract_ipa(ipa, &extract_dir))
            .await
            .map_err(|e| AppError::Filesystem("Failed to extract Madeira".into(), e.to_string()))
            .and_then(|r| r),
    )?;

    op.move_on("extract", "install")?;
    // The extracted .app is signed in place, then removed once it is installed
    let result = sideload(
        device_state,
        sideloader_state,
        app_dir.to_string_lossy().to_string(),
    )
    .await;
    let _ = tokio::fs::remove_dir_all(&work_dir).await;
    op.fail_if_err("install", result)?;
    op.complete("install")?;
    Ok(())
}

/// Downloads the newest Madeira IPA from GitHub and checks it against the
/// SHA-256 file published next to it.
async fn download_latest_madeira() -> Result<Vec<u8>, AppError> {
    let client = reqwest::Client::builder()
        .user_agent("iloader-memoryplus")
        .build()
        .map_err(|e| AppError::Download(e.to_string()))?;

    let release = get_text(&client, MADEIRA_LATEST_RELEASE).await?;
    let release: serde_json::Value = serde_json::from_str(&release)
        .map_err(|e| AppError::Download(format!("Unexpected GitHub response: {e}")))?;
    let assets = release["assets"].as_array().cloned().unwrap_or_default();
    let asset_url = |name: &str| {
        assets
            .iter()
            .find(|a| a["name"].as_str() == Some(name))
            .and_then(|a| a["browser_download_url"].as_str())
            .map(str::to_string)
    };

    let ipa_name = assets
        .iter()
        .filter_map(|a| a["name"].as_str())
        .find(|n| n.ends_with(".ipa"))
        .map(str::to_string)
        .ok_or_else(|| AppError::Download("The latest Madeira release has no IPA".into()))?;
    let ipa_url = asset_url(&ipa_name).unwrap_or_default();
    tracing::info!("Downloading {ipa_name} from {ipa_url}");

    let ipa = client
        .get(&ipa_url)
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| AppError::Download(e.to_string()))?
        .bytes()
        .await
        .map_err(|e| AppError::Download(e.to_string()))?
        .to_vec();

    match asset_url(&format!("{ipa_name}.sha256")) {
        Some(sha_url) => {
            let expected = get_text(&client, &sha_url).await?;
            let expected = expected.split_whitespace().next().unwrap_or("").to_lowercase();
            let actual = hex_sha256(&ipa);
            if expected != actual {
                return Err(AppError::Download(format!(
                    "{ipa_name} does not match its published SHA-256 (expected {expected}, got {actual})"
                )));
            }
            tracing::info!("{ipa_name} SHA-256 verified: {actual}");
        }
        None => tracing::warn!("{ipa_name} has no published SHA-256, skipping the check"),
    }

    Ok(ipa)
}

async fn get_text(client: &reqwest::Client, url: &str) -> Result<String, AppError> {
    client
        .get(url)
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| AppError::Download(e.to_string()))?
        .text()
        .await
        .map_err(|e| AppError::Download(e.to_string()))
}

fn hex_sha256(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(data)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Extracts the IPA into `work_dir` and returns the path of its `.app` bundle.
fn extract_ipa(ipa: Vec<u8>, work_dir: &PathBuf) -> Result<PathBuf, AppError> {
    let blocked_hint = format!(
        "Could not extract Madeira. If Windows Security blocked a file, that is a false positive on Madeira's Wine DLLs: add a temporary exclusion for the folder {} and try again",
        work_dir.display()
    );
    if work_dir.exists() {
        std::fs::remove_dir_all(work_dir)
            .map_err(|e| AppError::Filesystem(blocked_hint.clone(), e.to_string()))?;
    }
    std::fs::create_dir_all(work_dir)
        .map_err(|e| AppError::Filesystem(blocked_hint.clone(), e.to_string()))?;

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(ipa))
        .map_err(|e| AppError::Download(format!("The Madeira IPA is not a valid archive: {e}")))?;
    archive
        .extract(work_dir)
        .map_err(|e| AppError::Filesystem(blocked_hint.clone(), e.to_string()))?;

    std::fs::read_dir(work_dir.join("Payload"))
        .map_err(|e| AppError::Filesystem("The Madeira IPA has no Payload folder".into(), e.to_string()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| path.is_dir() && path.extension().is_some_and(|ext| ext == "app"))
        .ok_or_else(|| {
            AppError::Filesystem(
                "The Madeira IPA has no app bundle".into(),
                work_dir.display().to_string(),
            )
        })
}
