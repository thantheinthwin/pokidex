#[cfg(any(target_os = "macos", target_os = "windows"))]
use anyhow::Context;
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn pick_image_file() -> Result<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let script = r#"
            set selectedFile to choose file with prompt "Select a Pokémon image" of type {"public.image"}
            POSIX path of selectedFile
        "#;

        let output = Command::new("osascript")
            .args(["-e", script])
            .output()
            .context("Failed to launch macOS file picker (osascript)")?;

        if !output.status.success() {
            return Err(anyhow!("Image selection was cancelled or failed on macOS"));
        }

        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if path.is_empty() {
            return Err(anyhow!("No file was selected"));
        }

        return Ok(PathBuf::from(path));
    }

    #[cfg(target_os = "windows")]
    {
        let script = r#"
Add-Type -AssemblyName System.Windows.Forms
$dialog = New-Object System.Windows.Forms.OpenFileDialog
$dialog.Filter = "Image files (*.png;*.jpg;*.jpeg;*.webp;*.gif)|*.png;*.jpg;*.jpeg;*.webp;*.gif|All files (*.*)|*.*"
$dialog.Title = "Select a Pokémon image"
if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
  Write-Output $dialog.FileName
}
        "#;

        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", script])
            .output()
            .context("Failed to launch Windows file picker (PowerShell)")?;

        if !output.status.success() {
            return Err(anyhow!(
                "Image selection was cancelled or failed on Windows"
            ));
        }

        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if path.is_empty() {
            return Err(anyhow!("No file was selected"));
        }

        return Ok(PathBuf::from(path));
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(path) = pick_file_with_zenity()? {
            return Ok(path);
        }

        if let Some(path) = pick_file_with_kdialog()? {
            return Ok(path);
        }

        return Err(anyhow!(
            "Could not open a file chooser on Linux. Install 'zenity' or 'kdialog', or use: pokidex identify-image <path>"
        ));
    }

    #[allow(unreachable_code)]
    Err(anyhow!(
        "Interactive file picker is not supported on this platform. Use: pokidex identify-image <path>"
    ))
}

#[cfg(target_os = "linux")]
fn pick_file_with_zenity() -> Result<Option<PathBuf>> {
    let output = match Command::new("zenity")
        .args([
            "--file-selection",
            "--title=Select a Pokémon image",
            "--file-filter=Image files | *.png *.jpg *.jpeg *.webp *.gif",
        ])
        .output()
    {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };

    if !output.status.success() {
        return Ok(None);
    }

    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if selected.is_empty() {
        Ok(None)
    } else {
        Ok(Some(PathBuf::from(selected)))
    }
}

#[cfg(target_os = "linux")]
fn pick_file_with_kdialog() -> Result<Option<PathBuf>> {
    let output = match Command::new("kdialog")
        .args([
            "--getopenfilename",
            ".",
            "*.png *.jpg *.jpeg *.webp *.gif|Image files",
            "--title",
            "Select a Pokémon image",
        ])
        .output()
    {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };

    if !output.status.success() {
        return Ok(None);
    }

    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if selected.is_empty() {
        Ok(None)
    } else {
        Ok(Some(PathBuf::from(selected)))
    }
}
