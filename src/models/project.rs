use strum::{Display, EnumIter};

pub const LIT_ROUTER_VERSION: &str = "^0.1.4";
pub const URL_POLYFILL_VERSION: &str = "^10.1.0";

pub const TYPESCRIPT_VERSION: &str = "^6.0.2";

/// Categories of optional extra features that can be included in the generated project
#[derive(Debug, Display, EnumIter, Clone, Copy, PartialEq)]
pub enum ExtraType {
    #[strum(
        serialize = "Utilities: Helper functions (isBool, isNum, isPlainObject, toSentenceCase, toTitleCase)"
    )]
    Utils,
    #[strum(serialize = "Shared: UI components and styles used across the application")]
    Shared,
    #[strum(
        serialize = "Documentation: Repository README, CHANGELOG, Code of Conduct, and Security documentation in markdown"
    )]
    Docs,
}

/// The active state of an extra feature in the project configuration
#[derive(Debug, Display, Clone, PartialEq)]
pub enum Extra {
    Utils(bool),
    Shared(bool),
    Docs(bool),
}

/// Supported programming languages for the generated project
#[derive(Debug, Display, EnumIter, Clone, Copy, PartialEq)]
pub enum Language {
    TypeScript,
    JavaScript,
}

impl Language {
    /// Returns the appropriate file extension for the language
    pub fn get_file_extension(&self) -> &str {
        return match self {
            Self::TypeScript => "ts",
            Self::JavaScript => "js",
        };
    }
}

/// Supported package managers for the generated project, which determines the structure of the generated package.json and the installation commands used during setup
#[derive(Debug, Display, EnumIter, Clone, Copy, PartialEq)]
pub enum PackageManager {
    #[strum(serialize = "npm")]
    Npm,
    #[strum(serialize = "pnpm")]
    Pnpm,
    #[strum(serialize = "yarn")]
    Yarn,
}

/// Configuration for the project to be generated. This struct is used to drive the generation logic and determine which templates and files to include in the final output
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectConfig {
    pub language: Language,
    pub package_manager: PackageManager,
    pub extras: Option<Vec<Extra>>,
    pub routing: bool,
}

impl ProjectConfig {
    /// Helper method to check if a specific extra feature is enabled in the project configuration
    pub fn has_extra(&self, e: fn(&Extra) -> bool) -> bool {
        return self
            .extras
            .as_ref()
            .is_some_and(|extras| extras.iter().any(e));
    }
}

/// Categories of templates that can be generated for the project
pub enum TemplateCategory {
    LitRouter,
}

impl TemplateCategory {
    /// Returns the folder name where the templates for this category are stored
    pub fn as_str(&self) -> &'static str {
        return match self {
            Self::LitRouter => "lit-router",
        };
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        models::{PathSource, StagingArea, staging::STAGING_PATH},
        types::StagingResult,
    };

    use super::*;
    use strum::IntoEnumIterator;
    use tempfile::tempdir;

    #[test]
    fn test_language_extensions() {
        assert_eq!(Language::TypeScript.get_file_extension(), "ts");
        assert_eq!(Language::JavaScript.get_file_extension(), "js");
    }

    #[test]
    fn test_package_manager_serialization() {
        assert_eq!(PackageManager::Npm.to_string(), "npm");
        assert_eq!(PackageManager::Pnpm.to_string(), "pnpm");
        assert_eq!(PackageManager::Yarn.to_string(), "yarn");
    }

    #[test]
    fn test_extra_type_serialization() {
        let cases = vec![
            (
                ExtraType::Utils,
                "Utilities: Helper functions (isBool, isNum, isPlainObject, toSentenceCase, toTitleCase)",
            ),
            (
                ExtraType::Shared,
                "Shared: UI components and styles used across the application",
            ),
            (
                ExtraType::Docs,
                "Documentation: Repository README, CHANGELOG, Code of Conduct, and Security documentation in markdown",
            ),
        ];

        for (variant, expected) in cases {
            assert_eq!(variant.to_string(), expected);
        }
    }

    #[test]
    fn test_enum_iterators() {
        // Ensures all variants are present in the iteration
        let languages: Vec<_> = Language::iter().collect();
        assert_eq!(languages.len(), 2);
        assert!(languages.contains(&Language::TypeScript));

        let managers: Vec<_> = PackageManager::iter().collect();
        assert_eq!(managers.len(), 3);

        let extras: Vec<_> = ExtraType::iter().collect();
        assert_eq!(extras.len(), 3);
    }

    #[test]
    fn test_extra_type_logic_mapping() {
        let selected_extra_indices = vec![0, 2];
        let extras: Vec<_> = ExtraType::iter().collect();

        let mut selected_extras = Vec::new();
        for idx in selected_extra_indices {
            assert!(idx < extras.len());
            let extra_type = extras[idx];
            selected_extras.push(extra_type);
        }

        assert_eq!(selected_extras.len(), 2);
        assert!(matches!(selected_extras[0], ExtraType::Utils));
        assert!(matches!(selected_extras[1], ExtraType::Docs));
    }

    #[test]
    fn test_path_source_as_str() {
        let static_path = PathSource::Static("static/path");
        let dynamic_path = PathSource::Dynamic("dynamic/path".to_string());

        assert_eq!(static_path.as_str(), "static/path");
        assert_eq!(dynamic_path.as_str(), "dynamic/path");
    }

    #[test]
    fn test_project_config_has_extras() {
        let project_config = ProjectConfig {
            language: Language::JavaScript,
            package_manager: PackageManager::Npm,
            routing: false,
            extras: Some(vec![Extra::Utils(true), Extra::Docs(true)]),
        };

        assert!(project_config.has_extra(|e| matches!(e, Extra::Utils(true))));
        assert!(project_config.has_extra(|e| matches!(e, Extra::Docs(true))));
        assert!(!project_config.has_extra(|e| matches!(e, Extra::Shared(_))));
    }

    #[test]
    fn test_project_staging_area_lifecycle() -> StagingResult<()> {
        let tmp_dir = tempdir()?;
        let destination_path = tmp_dir.path();
        let staging = StagingArea::new(destination_path)?;

        // Setup simulated project files
        let language = Language::TypeScript;
        let package_manager = PackageManager::Npm;

        let main_file = format!("src/index.{}", language.get_file_extension());
        let pkg_json_file = "package.json";

        let main_content = "console.log('Hello World');";
        let pkg_content = format!(
            r#"{{ "name": "lit-app", "packageManager": "{}" }}"#,
            package_manager
        );

        // Add to staging
        staging.add_file(&main_file, main_content)?;
        staging.add_file(pkg_json_file, &pkg_content)?;

        // Verify paths exist in staging
        assert!(staging.staging_path().join(&main_file).exists());
        assert!(staging.staging_path().join(pkg_json_file).exists());

        // Commit to the target destination
        staging.commit(destination_path)?;

        // Verify paths exist in target (final destination)
        assert!(
            destination_path.join(&main_file).exists(),
            "Source file should exist in final path"
        );
        assert!(
            destination_path.join(pkg_json_file).exists(),
            "package.json should exist in final path"
        );

        // Verify content is correct in the final destination
        let actual_main_file_content = std::fs::read_to_string(destination_path.join(&main_file))?;
        assert_eq!(actual_main_file_content, main_content);
        let actual_pkg_file_content = fs::read_to_string(destination_path.join(pkg_json_file))?;
        assert!(actual_pkg_file_content.contains(package_manager.to_string().as_str()));

        Ok(())
    }

    #[test]
    fn test_project_staging_area_cleanup_on_new() -> StagingResult<()> {
        let tmp_dir = tempdir()?;
        let target_path = tmp_dir.path();

        // Simulating leftover files from a previous failed generation attempt
        let staging_path = target_path.join(STAGING_PATH);
        fs::create_dir_all(&staging_path)?;
        let leftover_file = staging_path.join("leftover.txt");
        fs::write(&leftover_file, "delete me")?;

        assert!(leftover_file.exists());

        // Initialize new StagingArea
        // Trigger wiping the existing staging directory
        let _staging = StagingArea::new(target_path)?;

        // Verify drop/cleanup logic ran and the leftover file is gone, but the staging directory is recreated
        assert!(
            !leftover_file.exists(),
            "The leftover file should have been deleted by StagingArea::new"
        );
        assert!(
            staging_path.exists(),
            "The staging directory should have been recreated by StagingArea::new"
        );

        Ok(())
    }
}
