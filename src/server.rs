use super::*;

const SERVER_PATH: &str = "extension/out/server.js";
const PACKAGE_NAME: &str = "vscode-motoko";

impl MotokoExtension {
    /// Checks if the server script exists at the given path.
    fn server_exists(&self, server_path: &str) -> bool {
        fs::metadata(server_path).is_ok_and(|stat| stat.is_file())
    }

    /// Returns the path to the server script.
    pub fn server_script_path(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_script_path {
            if self.server_exists(path) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "dfinity/vscode-motoko",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        // Strip 'v' prefix from release.version if present
        let release_version = release.version.strip_prefix('v').unwrap_or(&release.version);

        let latest_package_name = format!("{}-{}", PACKAGE_NAME, release_version);
        let server_path = format!("{}/{}", &latest_package_name, SERVER_PATH);

        if !self.server_exists(&server_path) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let download_url = format!(
                "https://github.com/dfinity/vscode-motoko/releases/download/v{}/{}.vsix",
                release_version, latest_package_name
            );

            zed::download_file(
                &download_url,
                &latest_package_name,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}, from: {download_url}, to: {server_path}, as: {latest_package_name}"))?;

            // Clean up old packages after successful download
            self.clean_old_packages(&latest_package_name)
                .map_err(|e| format!("failed to clean old packages: {e}"))?;
        }

        self.cached_script_path = Some(server_path.clone());
        Ok(server_path)
    }

    /// Removes old vscode-motoko packages but keeps the latest one
    fn clean_old_packages(&self, latest_package_name: &str) -> Result<()> {
        let current_dir = env::current_dir()
            .map_err(|e| format!("failed to get current directory: {e}"))?;

        let entries = fs::read_dir(&current_dir)
            .map_err(|e| format!("failed to read directory: {e}"))?;

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    println!("Warning: Failed to read directory entry: {e}");
                    continue;
                }
            };

            let path = entry.path();

            // Skip if not a directory
            if !path.is_dir() {
                continue;
            }

            let file_name = match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };

            // Check if this is a vscode-motoko package, but not the latest one
            if file_name.starts_with(PACKAGE_NAME) && file_name != latest_package_name {
                println!("Removing old package: {}", file_name);

                match fs::remove_dir_all(&path) {
                    Ok(_) => (),
                    Err(e) => println!("Warning: Failed to remove old package {}: {}", file_name, e),
                }
            }
        }

        Ok(())
    }
}
