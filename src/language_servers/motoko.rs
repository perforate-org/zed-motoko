use crate::*;
use log::{info, warn};

const SERVER_PATH: &str = "extension/out/server.js";
const PACKAGE_NAME: &str = "vscode-motoko";

pub struct Motoko {
    cached_script_path: Option<String>,
}

impl Motoko {
    pub const LANGUAGE_SERVER_ID: &'static str = "motoko-language-server";
    pub fn new() -> Self {
        Self {
            cached_script_path: None,
        }
    }

    /// Checks if the server script exists at the given path.
    fn server_exists(&self, server_path: &str) -> bool {
        fs::metadata(server_path).is_ok_and(|stat| stat.is_file())
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        let server_path = self.server_script_path(language_server_id)?;

        let env = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|s| s.binary)
            .and_then(|binary| binary.env);

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: env.into_iter().flat_map(|env| env.into_iter()).collect(),
        })
    }

    pub fn language_server_workspace_configuration(
        &mut self,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        let mut settings = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .map(|lsp_settings| lsp_settings.settings);

        if !matches!(settings, Ok(Some(_))) {
            settings = self
                .language_server_initialization_options(worktree)
                .map(|initialization_options| {
                    initialization_options.and_then(|initialization_options| {
                        initialization_options.get(Self::LANGUAGE_SERVER_ID).cloned()
                    })
                })
        }

        settings
    }

    pub fn language_server_initialization_options(
        &mut self,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        let options = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .map(|lsp_settings| lsp_settings.initialization_options)?;

        Ok(options)
    }

    /// Returns the path to the server script.
    pub fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_script_path
            && self.server_exists(path)
        {
            return Ok(path.clone());
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
        let release_version = release
            .version
            .strip_prefix('v')
            .unwrap_or(&release.version);

        let latest_package_name = format!("{PACKAGE_NAME}-{release_version}");
        let package_dir = format!("{PACKAGE_NAME}/{latest_package_name}");
        let server_path = format!("{}/{}", &package_dir, SERVER_PATH);

        if !self.server_exists(&server_path) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            fs::create_dir_all(PACKAGE_NAME)
                .map_err(|e| format!("failed to create package directory {PACKAGE_NAME}: {e}"))?;

            let download_url = format!(
                "https://github.com/dfinity/vscode-motoko/releases/download/v{release_version}/{latest_package_name}.vsix"
            );

            zed::download_file(
                &download_url,
                &package_dir,
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
        let current_dir =
            env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?;
        let package_root = current_dir.join(PACKAGE_NAME);

        let entries = match fs::read_dir(&package_root) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(format!("failed to read directory: {e}")),
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    warn!("Failed to read directory entry: {e}");
                    continue;
                }
            };

            let path = entry.path();

            // Skip if not a directory
            if !path.is_dir() {
                continue;
            }

            let dir_name = match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };

            // Check if this is a vscode-motoko package directory, but not the latest one
            if dir_name.starts_with(PACKAGE_NAME) && dir_name != latest_package_name {
                info!(
                    "Removing old package directory: {} (full path: {})",
                    dir_name,
                    path.display()
                );

                // Ensure we're removing the entire vscode-motoko directory
                if let Err(e) = fs::remove_dir_all(&path) {
                    warn!("Failed to remove old package directory {dir_name}: {e}");
                } else {
                    info!("Successfully removed old package directory: {dir_name}");
                }
            }
        }

        Ok(())
    }
}
