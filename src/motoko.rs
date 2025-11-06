use std::{env, fs};
use zed_extension_api::{
    self as zed, Command, LanguageServerId, Result, Worktree, serde_json::Value,
    settings::LspSettings,
};

mod language_servers;

use crate::language_servers::{Motoko, Candid};

/// Main extension for Motoko support in Zed
struct MotokoExtension {
    motoko: Option<Motoko>,
    candid: Option<Candid>,
}

impl zed::Extension for MotokoExtension {
    fn new() -> Self {
        Self { motoko: None, candid: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        match language_server_id.as_ref() {
            Motoko::LANGUAGE_SERVER_ID => {
                let motoko = self.motoko.get_or_insert_with(Motoko::new);
                motoko.language_server_command(language_server_id, worktree)
            }
            Candid::LANGUAGE_SERVER_ID => {
                let candid = self.candid.get_or_insert_with(Candid::new);
                candid.language_server_command(language_server_id, worktree)
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        if server_id.as_ref() == Motoko::LANGUAGE_SERVER_ID && let Some(motoko) = self.motoko.as_mut() {
            return motoko.language_server_workspace_configuration(worktree)
        }
        if server_id.as_ref() == Candid::LANGUAGE_SERVER_ID && let Some(candid) = self.candid.as_mut() {
            return candid.language_server_workspace_configuration(worktree)
        }

        Ok(None)
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        match language_server_id.as_ref() {
            Motoko::LANGUAGE_SERVER_ID => {
                let motoko = self.motoko.get_or_insert_with(Motoko::new);
                motoko.language_server_initialization_options(worktree)
            }
            Candid::LANGUAGE_SERVER_ID => {
                let candid = self.candid.get_or_insert_with(Candid::new);
                candid.language_server_initialization_options(worktree)
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }
}

zed::register_extension!(MotokoExtension);
