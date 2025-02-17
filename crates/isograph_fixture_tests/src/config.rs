use std::path::{Path, PathBuf};

use intern::string_key::Intern;
use isograph_config::{absolute_and_relative_paths, CompilerConfig};

pub fn isograph_config_for_tests(current_working_directory: &Path) -> CompilerConfig {
    let mut path_root = "".to_string();

    if cfg!(windows) {
        // handle different disks C:\
        path_root.push_str(
            PathBuf::from(current_working_directory)
                .ancestors()
                .last()
                .expect("Expected root path")
                .to_str()
                .expect("Expected root path"),
        )
    } else {
        path_root.push('/')
    };

    let current_working_directory = current_working_directory.to_str().unwrap().intern().into();

    CompilerConfig {
        config_location: ([&path_root, "test-config-location"].iter().collect()),
        project_root: ([&path_root, "test-project-root"].iter().collect()),
        artifact_directory: absolute_and_relative_paths(
            current_working_directory,
            [&path_root, "test-artifact-directory"].iter().collect(),
        ),
        schema: absolute_and_relative_paths(
            current_working_directory,
            [&path_root, "test-schema"].iter().collect(),
        ),
        schema_extensions: vec![],
        options: Default::default(),
        current_working_directory,
    }
}
