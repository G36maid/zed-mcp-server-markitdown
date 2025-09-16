use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "markitdown-mcp-npx";
const PACKAGE_VERSION: &str = "latest";

struct MarkitDownModelContextExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct MarkitDownModelContextExtensionSettings {
    #[serde(default)]
    package_version: Option<String>,
}

impl zed::Extension for MarkitDownModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("mcp-server-markitdown", project)?;

        let ext_settings: Option<MarkitDownModelContextExtensionSettings> =
            if let Some(settings_value) = settings.settings {
                match serde_json::from_value(settings_value) {
                    Ok(s) => Some(s),
                    Err(e) => return Err(format!("Failed to parse settings: {}", e).into()),
                }
            } else {
                None
            };

        // Determine package version from settings or use default
        let version = if let Some(settings) = &ext_settings {
            settings
                .package_version
                .as_deref()
                .unwrap_or(PACKAGE_VERSION)
        } else {
            PACKAGE_VERSION
        };

        // Check if the package is installed and install/update if needed
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed_version.as_deref() != Some(version) {
            zed::npm_install_package(PACKAGE_NAME, version)?;
        }

        // Run the installed package binary directly
        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![std::env::current_dir()
                .unwrap()
                .join("node_modules")
                .join(".bin")
                .join("markitdown-mcp-npx")
                .to_string_lossy()
                .to_string()],
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        let settings = ContextServerSettings::for_project("mcp-server-markitdown", project);

        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                if let Ok(markitdown_settings) = serde_json::from_value::<
                    MarkitDownModelContextExtensionSettings,
                >(settings_value)
                {
                    if let Some(package_version) = markitdown_settings.package_version {
                        default_settings = default_settings
                            .replace("\"latest\"", &format!("\"{}\"", package_version));
                    }
                }
            }
        }

        let settings_schema = serde_json::to_string(&schemars::schema_for!(
            MarkitDownModelContextExtensionSettings
        ))
        .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(MarkitDownModelContextExtension);
