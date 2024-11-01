use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

pub static ISOGRAPH_FOLDER: &str = "__isograph";

use std::error::Error;

use colorize::AnsiColor;

#[derive(Debug)]
pub struct CompilerConfig {
    /// The folder where the compiler should look for Isograph literals
    pub project_root: PathBuf,
    /// The folder where the compiler should create artifacts
    pub artifact_directory: PathBuf,
    /// The absolute path to the GraphQL schema
    pub schema: PathBuf,
    /// The absolute path to the schema extensions
    pub schema_extensions: Vec<PathBuf>,

    /// Various options that are of lesser importance
    pub options: ConfigOptions,
}

#[derive(Default, Debug, Clone)]
pub struct ConfigOptions {
    pub on_invalid_id_type: OptionalValidationLevel,
    pub generate_file_extensions: OptionalGenerateFileExtensions,
}

#[derive(Debug, Clone)]
pub struct OptionalGenerateFileExtensions {
    extensions: HashMap<String, String>,
}

impl OptionalGenerateFileExtensions {
    pub fn get(&self, extension: &str) -> Option<&String> {
        let mapped_extension = self.extensions.get(extension);

        if mapped_extension.is_none() {
            eprintln!(
                "{}\nmissing .{} file extension in options.generate_file_extensions\n",
                "Warning:".yellow(),
                extension
            );
        };

        mapped_extension
    }
    pub fn ts(&self) -> &str {
        match self.get("ts") {
            None => "",
            Some(v) => v,
        }
    }
    pub fn tsx(&self) -> &str {
        match self.get("tsx") {
            None => "",
            Some(v) => v,
        }
    }
}

impl Default for OptionalGenerateFileExtensions {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("js".to_string(), "".to_string());
        map.insert("jsx".to_string(), "".to_string());
        map.insert("cjs".to_string(), "".to_string());
        map.insert("cjsx".to_string(), "".to_string());
        map.insert("mjs".to_string(), "".to_string());
        map.insert("mjsx".to_string(), "".to_string());

        map.insert("ts".to_string(), "".to_string());
        map.insert("tsx".to_string(), "".to_string());
        map.insert("cts".to_string(), "".to_string());
        map.insert("ctsx".to_string(), "".to_string());
        map.insert("mts".to_string(), "".to_string());
        map.insert("mtsx".to_string(), "".to_string());

        OptionalGenerateFileExtensions { extensions: map }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OptionalValidationLevel {
    /// If this validation error is encountered, it will be ignored
    Ignore,
    /// If this validation error is encountered, a warning will be issued
    Warn,
    /// If this validation error is encountered, the compilation will fail
    Error,
}

impl OptionalValidationLevel {
    pub fn on_failure<E>(self, on_error: impl FnOnce() -> E) -> Result<(), E>
    where
        E: Error,
    {
        match self {
            OptionalValidationLevel::Ignore => Ok(()),
            OptionalValidationLevel::Warn => {
                let warning = on_error();
                // TODO pass to some sort of warning gatherer, this is weird!
                // The fact that we know about colorize here is weird!
                eprintln!("{}\n{}\n", "Warning:".yellow(), warning);
                Ok(())
            }
            OptionalValidationLevel::Error => Err(on_error()),
        }
    }
}

impl Default for OptionalValidationLevel {
    fn default() -> Self {
        Self::Ignore
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigFile {
    /// The relative path to the folder where the compiler should look for Isograph literals
    pub project_root: PathBuf,
    /// The relative path to the folder where the compiler should create artifacts
    /// Defaults to the project_root directory.
    pub artifact_directory: Option<PathBuf>,
    /// The relative path to the GraphQL schema
    pub schema: PathBuf,
    /// The relative path to schema extensions
    #[serde(default)]
    pub schema_extensions: Vec<PathBuf>,

    /// Various that are of lesser importance
    #[serde(default = "Default::default")]
    pub options: ConfigFileOptions,
}

pub fn create_config(mut config_location: PathBuf) -> CompilerConfig {
    let config_contents = match std::fs::read_to_string(&config_location) {
        Ok(contents) => contents,
        Err(_) => match config_location.to_str() {
            Some(loc) => {
                panic!("Expected config to be found at {}", loc)
            }
            None => {
                panic!("Expected config to be found.")
            }
        },
    };

    let config_parsed: ConfigFile = serde_json::from_str(&config_contents)
        .unwrap_or_else(|e| panic!("Error parsing config. Error: {}", e));

    config_location.pop();
    let config_dir = config_location;

    let artifact_dir = config_dir
        .join(
            config_parsed
                .artifact_directory
                .as_ref()
                .unwrap_or(&config_parsed.project_root),
        )
        .join(ISOGRAPH_FOLDER);
    std::fs::create_dir_all(&artifact_dir).expect("Unable to create artifact directory");

    let project_root_dir = config_dir.join(&config_parsed.project_root);
    std::fs::create_dir_all(&project_root_dir).expect("Unable to create project root directory");

    CompilerConfig {
        project_root: project_root_dir.canonicalize().unwrap_or_else(|_| {
            panic!(
                "Unable to canonicalize project root at {:?}.",
                config_parsed.project_root
            )
        }),
        artifact_directory: artifact_dir.canonicalize().unwrap_or_else(|_| {
            panic!(
                "Unable to canonicalize artifact directory at {:?}.",
                config_parsed.artifact_directory
            )
        }),
        schema: config_dir
            .join(&config_parsed.schema)
            .canonicalize()
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to canonicalize schema path. Does {:?} exist?",
                    config_parsed.schema
                )
            }),
        schema_extensions: config_parsed
            .schema_extensions
            .into_iter()
            .map(|schema_extension| {
                config_dir
                    .join(&schema_extension)
                    .canonicalize()
                    .unwrap_or_else(|_| {
                        panic!(
                            "Unable to canonicalize schema extension path. Does {:?} exist?",
                            schema_extension
                        )
                    })
            })
            .collect(),
        options: create_options(config_parsed.options),
    }
}

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
struct ConfigFileOptions {
    on_invalid_id_type: ConfigFileOptionalValidationLevel,
    generate_file_extensions: ConfigFileOptionalGenerateFileExtensions,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum ConfigFileOptionalValidationLevel {
    /// If this validation error is encountered, it will be ignored
    Ignore,
    /// If this validation error is encountered, a warning will be issued
    Warn,
    /// If this validation error is encountered, the compilation will fail
    Error,
}

impl Default for ConfigFileOptionalValidationLevel {
    fn default() -> Self {
        Self::Error
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ConfigFileOptionalGenerateFileExtensions {
    Yes(HashMap<String, String>),
    No,
}

impl Default for ConfigFileOptionalGenerateFileExtensions {
    fn default() -> Self {
        Self::No
    }
}

fn create_options(options: ConfigFileOptions) -> ConfigOptions {
    ConfigOptions {
        on_invalid_id_type: create_optional_validation_level(options.on_invalid_id_type),
        generate_file_extensions: create_generate_file_extensions(options.generate_file_extensions),
    }
}

fn create_optional_validation_level(
    optional_validation_level: ConfigFileOptionalValidationLevel,
) -> OptionalValidationLevel {
    match optional_validation_level {
        ConfigFileOptionalValidationLevel::Ignore => OptionalValidationLevel::Ignore,
        ConfigFileOptionalValidationLevel::Warn => OptionalValidationLevel::Warn,
        ConfigFileOptionalValidationLevel::Error => OptionalValidationLevel::Error,
    }
}

fn create_generate_file_extensions(
    optional_generate_file_extensions: ConfigFileOptionalGenerateFileExtensions,
) -> OptionalGenerateFileExtensions {
    match optional_generate_file_extensions {
        ConfigFileOptionalGenerateFileExtensions::Yes(extensions) => {
            OptionalGenerateFileExtensions { extensions }
        }

        ConfigFileOptionalGenerateFileExtensions::No => OptionalGenerateFileExtensions::default(),
    }
}
