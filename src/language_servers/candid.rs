use crate::*;

pub struct Candid {
    cached_binary_path: Option<String>,
}

impl Candid {
    pub const LANGUAGE_SERVER_ID: &'static str = "candid-language-server";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        let server_path = self.server_binary_path(language_server_id, worktree)?;

        let env = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|s| s.binary)
            .and_then(|binary| binary.env);

        Ok(zed::Command {
            command: env::current_dir()
                .unwrap()
                .join(&server_path)
                .to_string_lossy()
                .to_string(),
            args: vec![],
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

    /// Returns the path to the server.
    pub fn server_binary_path(&mut self, language_server_id: &LanguageServerId, worktree: &Worktree) -> Result<String> {
        if let Some(path) = worktree.which(Self::LANGUAGE_SERVER_ID) {
            return Ok(path);
        }
        if let Some(path) = &self.cached_binary_path
            && fs::metadata(path).is_ok_and(|stat| stat.is_file())
        {
            return Ok(path.clone());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "perforate-org/candid-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "candid-language-server-{version}-{arch}-{os}.{ext}",
            version = release.version,
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc",
            },
            ext = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir_name = format!("candid-language-server-{}", release.version);
        let version_dir = format!("candid-language-server/{version_dir_name}");
        let binary_path = format!("{version_dir}/candid-language-server");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir("candid-language-server").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir_name) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}
