use std::{env, fs};
use zed_extension_api::{
    self as zed, Command, LanguageServerId, Result, Worktree, serde_json::Value,
    settings::LspSettings,
};

mod server;

/// Main extension for Motoko support in Zed
struct MotokoExtension {
    cached_script_path: Option<String>,
}

impl zed::Extension for MotokoExtension {
    fn new() -> Self {
        Self {
            cached_script_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        let server_path = self.server_script_path(language_server_id)?;

        let env = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.binary)
            .and_then(|binary| binary.env);

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                zed_ext::sanitize_windows_path(env::current_dir().unwrap())
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: env.into_iter().flat_map(|env| env.into_iter()).collect(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        let mut settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings);

        if !matches!(settings, Ok(Some(_))) {
            settings = self
                .language_server_initialization_options(server_id, worktree)
                .map(|initialization_options| {
                    initialization_options.and_then(|initialization_options| {
                        initialization_options.get("motoko").cloned()
                    })
                })
        }

        settings
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        let options = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.initialization_options)?;

        Ok(options)
    }
}

zed::register_extension!(MotokoExtension);

/// Extensions to the Zed extension API that have not yet stabilized.
mod zed_ext {
    /// Sanitizes the given path to remove the leading `/` on Windows.
    ///
    /// On macOS and Linux this is a no-op.
    ///
    /// This is a workaround for https://github.com/bytecodealliance/wasmtime/issues/10415.
    pub fn sanitize_windows_path(path: std::path::PathBuf) -> std::path::PathBuf {
        use zed_extension_api::{Os, current_platform};

        let (os, _arch) = current_platform();
        match os {
            Os::Mac | Os::Linux => path,
            Os::Windows => path
                .to_string_lossy()
                .to_string()
                .trim_start_matches('/')
                .into(),
        }
    }
}
