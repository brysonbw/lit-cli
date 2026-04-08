use std::{
    env::current_dir,
    fs,
    path::{Component, Path, PathBuf},
};

use crate::{
    commands::Command,
    models::{
        Asset, Extra, ExtraType, Language, PackageManager, PathSource, ProjectConfig, StagingArea,
        TemplateCategory,
        project::{LIT_ROUTER_VERSION, TYPESCRIPT_VERSION, URL_POLYFILL_VERSION},
    },
    types::{CommandResult, ValidateCommandResult},
    ui::UserInterface,
};

use chroma_print::print_success;
use clap::Args;
use serde_json::{Value, json};
use strum::IntoEnumIterator;

/// Command to create a new Lit project with various configuration options
#[derive(Args)]
pub struct NewCommand {
    /// The name of the project (also the destination directory for project initialization)
    pub name: String,
}

impl NewCommand {
    /// Resolves and validates the provided project name and returns the destination path for project initialization
    fn resolve_and_validate_project_name(&self, root: &Path) -> ValidateCommandResult<PathBuf> {
        // Resolve and canonicalize root
        let root = root
            .canonicalize()
            .map_err(|_| "Could not resolve root directory".to_string())?;

        let name = self.name.trim();
        if name.is_empty() {
            return Err("Project name cannot be empty".into());
        }

        // Checks to 'jail' the root to prevent directory traversal and ensure the destination path is within the root
        let mut depth: i32 = 0;
        for component in Path::new(name).components() {
            match component {
                Component::Normal(_) => depth += 1,
                Component::CurDir => {}
                Component::ParentDir => depth -= 1,
                _ => return Err("Path invalid. Please provide a valid project name".into()),
            }
            if depth < 0 {
                return Err("Path invalid. Please provide a valid project name".into());
            }
        }

        if depth == 0 {
            return Err(format!(
                "The provided path for project '{}' does not point to a valid location",
                self.name
            ));
        }

        // Validation checks for the destination path
        let destination_path = root.join(name);

        // Reject paths ending in a file (extension)
        if destination_path.extension().is_some() {
            return Err(format!(
                "Project name '{}' is a file. Please provide a directory name",
                self.name
            ));
        }

        // Check if it exists and a valid place to start
        if destination_path.exists() {
            // Check if it's a file
            if !destination_path.is_dir() {
                return Err(format!("'{}' already exists and is a file", self.name));
            }

            // Check if creating new project in an existing workspace (package.json in destination path)
            // TODO: Might check for another significant file to determine if it's a current project/workspace? For example, Angular CLI checks for `angular.json`
            let package_json_path = destination_path.join("package.json");
            if package_json_path.exists() {
                return Err(
                    "This command is not available when running the Lit CLI inside a workspace"
                        .to_string(),
                );
            }

            // Check if directory is empty
            // TODO: Strict empty directory check. But possibly remove to allow overriding in the future and then check for merge conflicts in `src/` (or some other destination folder/file path)?
            let entries = fs::read_dir(&destination_path)
                .map_err(|_| "Could not read directory".to_string())?;
            let dir_has_content = entries.filter_map(|e| e.ok()).any(|entry| {
                let name = entry.file_name();
                // Ignore `.git` directory if it exists
                let is_git_dir = name == ".git" && entry.path().is_dir();
                !is_git_dir
            });

            if dir_has_content {
                return Err(format!(
                    "Directory '{}' is not empty. Please provide an empty directory or remove existing files to continue",
                    self.name
                ));
            }
        }

        return Ok(destination_path);
    }

    /// Collects user input to configure the project setup based on prompt options
    fn configure_project(&self, ui: &dyn UserInterface) -> CommandResult<ProjectConfig> {
        // Collect language
        let lang_options: Vec<String> = Language::iter().map(|l| l.to_string()).collect();
        let lang_idx = ui.render_select("Select a variant:", &lang_options)?;
        let language = Language::iter().nth(lang_idx).unwrap();

        // Collect package manager
        let pm_options: Vec<String> = PackageManager::iter().map(|p| p.to_string()).collect();
        let pm_idx = ui.render_select("Select Package Manager:", &pm_options)?;
        let package_manager = PackageManager::iter().nth(pm_idx).unwrap();

        // Collect routing
        let routing = ui.render_confirm("Use `lit-router` for client side routing?")?;

        // Collect extras (multi-select)
        let extra_types: Vec<ExtraType> = ExtraType::iter().collect();
        let extra_options: Vec<String> = extra_types.iter().map(|e| e.to_string()).collect();
        let selected_e_indices = ui.render_multi_select(
            "Select extras to include (Press <space> to select, <enter> to continue):",
            &extra_options,
        )?;

        let mut extras = Vec::new();
        for idx in selected_e_indices {
            let extra_type: ExtraType = extra_types[idx];

            let extra: Extra = match extra_type {
                ExtraType::Utils => Extra::Utils(true),
                ExtraType::Shared => Extra::Shared(true),
                ExtraType::Docs => Extra::Docs(true),
            };

            extras.push(extra);
        }

        return Ok(ProjectConfig {
            language,
            package_manager,
            extras: Some(extras).filter(|v| !v.is_empty()),
            routing,
        });
    }

    /// Generates the files from the given project configuration based on prompt options
    fn generate_files(
        &self,
        destination_path: &Path,
        project_config: &ProjectConfig,
    ) -> ValidateCommandResult<()> {
        let modify_package_json =
            matches!(project_config.language, Language::TypeScript) || project_config.routing;

        let staging = StagingArea::new(destination_path)
            .map_err(|_| "Failed to initialize staging".to_string())?;

        let path_source_mappings = self.get_template_base_mappings(project_config);

        for (source_path, target_path) in path_source_mappings {
            let file_key = source_path.as_str();
            let target_str = target_path.as_str();

            // Get the template file from the binary
            let asset =
                Asset::get(file_key).ok_or_else(|| format!("Template not found: {}", file_key))?;

            // Convert bytes to String for manipulation
            let mut content = String::from_utf8(asset.data.to_vec())
                .map_err(|_| format!("Template {} is not valid", file_key))?;

            // Modifying `package.json`
            if target_str == "package.json" && modify_package_json {
                let mut json_data: Value = serde_json::from_str(&content)
                    .map_err(|_| format!("Failed to parse JSON in {:?}", target_str))?;

                if project_config.routing
                    && let Some(deps) = json_data
                        .get_mut("dependencies")
                        .and_then(|v| v.as_object_mut())
                {
                    deps.insert("@lit-labs/router".into(), json!(LIT_ROUTER_VERSION));
                    deps.insert("urlpattern-polyfill".into(), json!(URL_POLYFILL_VERSION));
                }

                if matches!(project_config.language, Language::TypeScript)
                    && let Some(dev_deps) = json_data
                        .get_mut("devDependencies")
                        .and_then(|v| v.as_object_mut())
                {
                    dev_deps.insert("typescript".into(), json!(TYPESCRIPT_VERSION));
                }

                content = serde_json::to_string_pretty(&json_data)
                    .map_err(|_| "Failed to serialize JSON".to_string())?;
            }

            // Modifying `index.html`
            if target_str == "index.html" {
                let script_name = if project_config.routing {
                    "app"
                } else {
                    "my-element"
                };
                let script_file = format!(
                    "{}.{}",
                    script_name,
                    project_config.language.get_file_extension()
                );
                let script_tag = format!(
                    r#"<script type="module" src="./src/{}"></script>"#,
                    script_file
                );
                let placeholder = r#"<script id="main"></script>"#;

                if content.contains(placeholder) {
                    content = content.replace(placeholder, &script_tag);
                // Fallback
                } else if content.contains("</head>") {
                    content = content.replace("</head>", &format!("{}\n</head>", script_tag));
                }
            }

            // Write to staging area
            staging
                .add_file(target_str, &content)
                .map_err(|_| format!("Failed to stage file: {}", target_str))?;
        }

        // Atomic commit folder/files
        println!("\n");
        staging
            .commit(destination_path)
            .map_err(|_| "Failed to finalize project generation".to_string())?;

        return Ok(());
    }

    // TODO: Refactor method and/or the building of folder/file mappings?
    /// Gets the base template mappings for the project based on the provided configuration and returns a list of source/target path pairs to generate from the templates
    fn get_template_base_mappings(
        &self,
        project_config: &ProjectConfig,
    ) -> Vec<(PathSource, PathSource)> {
        // Base mappings
        let mut mappings: Vec<(PathSource, PathSource)> = vec![
            (
                PathSource::Static("config/vite.config.js"),
                PathSource::Static("vite.config.js"),
            ),
            (
                PathSource::Static("config/.env.development.local.example"),
                PathSource::Static(".env.development.local"),
            ),
            (
                PathSource::Static("config/.env.local.example"),
                PathSource::Static(".env.local"),
            ),
            (
                PathSource::Static("config/.gitignore.example"),
                PathSource::Static(".gitignore"),
            ),
            (
                PathSource::Static("public/lit.svg"),
                PathSource::Static("public/lit.svg"),
            ),
            (
                PathSource::Static("public/robots.txt"),
                PathSource::Static("public/robots.txt"),
            ),
            (
                PathSource::Static("public/vite.svg"),
                PathSource::Static("public/vite.svg"),
            ),
            (
                PathSource::Static("json/package.json"),
                PathSource::Static("package.json"),
            ),
            (
                PathSource::Static("types/vite-env.d.ts"),
                PathSource::Static("src/vite-env.d.ts"),
            ),
            (
                PathSource::Static("css/main.css"),
                PathSource::Static("src/main.css"),
            ),
        ];

        // Conditional mappings
        // Routing
        match project_config.routing {
            true => {
                let lit_router_template_category = TemplateCategory::LitRouter;
                mappings.push((
                    PathSource::Dynamic(format!(
                        "{}/html/index.html",
                        lit_router_template_category.as_str()
                    )),
                    PathSource::Static("index.html"),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "{}/app.{}",
                        lit_router_template_category.as_str(),
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/app.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "{}/main.{}",
                        lit_router_template_category.as_str(),
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/main.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "{}/pages/not-found.{}",
                        lit_router_template_category.as_str(),
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/pages/not-found.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "{}/pages/about.{}",
                        lit_router_template_category.as_str(),
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/pages/about.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "{}/pages/home.{}",
                        lit_router_template_category.as_str(),
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/pages/home.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "components/header.{}",
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/components/header.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "components/footer.{}",
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/components/footer.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
            }
            _ => {
                mappings.push((
                    PathSource::Static("html/index.html"),
                    PathSource::Static("index.html"),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "components/my-element.{}",
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/my-element.{}",
                        project_config.language.get_file_extension()
                    )),
                ))
            }
        }

        // Language
        match project_config.language {
            Language::JavaScript => {
                mappings.push((
                    PathSource::Static("json/jsconfig.json"),
                    PathSource::Static("jsconfig.json"),
                ));
                mappings.push((
                    PathSource::Static("types/index.d.js"),
                    PathSource::Static("src/index.d.js"),
                ));
            }
            Language::TypeScript => {
                mappings.push((
                    PathSource::Static("json/tsconfig.json"),
                    PathSource::Static("tsconfig.json"),
                ));
                mappings.push((
                    PathSource::Static("types/index.ts"),
                    PathSource::Static("src/types/index.ts"),
                ));
            }
        }

        // Extras
        if project_config.extras.is_some() {
            // Utils
            if project_config.has_extra(|e| matches!(e, Extra::Utils(true))) {
                mappings.push((
                    PathSource::Dynamic(format!(
                        "utils/helpers.{}",
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/utils/helpers.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
            }
            // Shared
            if project_config.has_extra(|e| matches!(e, Extra::Shared(true))) {
                mappings.push((
                    PathSource::Dynamic(format!(
                        "shared/styles/buttonStyles.{}",
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/shared/styles/buttonStyles.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "shared/styles/linkStyles.{}",
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/shared/styles/linkStyles.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "shared/ui/card.{}",
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/shared/ui/card.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
                mappings.push((
                    PathSource::Dynamic(format!(
                        "shared/ui/loading-indicator.{}",
                        project_config.language.get_file_extension()
                    )),
                    PathSource::Dynamic(format!(
                        "src/shared/ui/loading-indicator.{}",
                        project_config.language.get_file_extension()
                    )),
                ));
            }
            // Docs
            if project_config.has_extra(|e| matches!(e, Extra::Docs(true))) {
                mappings.push((
                    PathSource::Static("docs/CHANGELOG.md"),
                    PathSource::Static("CHANGELOG.md"),
                ));
                mappings.push((
                    PathSource::Static("docs/CODE_OF_CONDUCT.md"),
                    PathSource::Static("CODE_OF_CONDUCT.md"),
                ));
                mappings.push((
                    PathSource::Static("docs/CONTRIBUTING.md"),
                    PathSource::Static("CONTRIBUTING.md"),
                ));
                mappings.push((
                    PathSource::Static("docs/README.md"),
                    PathSource::Static("README.md"),
                ));
                mappings.push((
                    PathSource::Static("docs/SECURITY.md"),
                    PathSource::Static("SECURITY.md"),
                ));
            }
        }

        return mappings;
    }
}

impl Command for NewCommand {
    fn run(self, ui: &dyn UserInterface) -> CommandResult<()> {
        let cwd = current_dir()?;
        let destination_path = self.resolve_and_validate_project_name(&cwd)?;

        let project_config: ProjectConfig =
            self.configure_project(ui)
                .map_err(|_| -> Box<dyn std::error::Error> {
                    "Failed to configure project".to_string().into()
                })?;

        self.generate_files(&destination_path, &project_config)
            .map_err(|_| -> Box<dyn std::error::Error> {
                "Failed to generate files".to_string().into()
            })?;

        print_success!("\nInitialized project successfully!");
        println!("\nRun the following commands to get started:");
        println!("cd {}", self.name);
        match project_config.package_manager {
            PackageManager::Yarn => {
                println!("yarn install");
                println!("yarn dev");
            }
            PackageManager::Npm => {
                println!("npm install");
                println!("npm run dev");
            }
            PackageManager::Pnpm => {
                println!("pnpm install");
                println!("pnpm run dev");
            }
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::MockUserInterface;
    use mockall::predicate::always as any;
    use mockall::predicate::{
        eq,
        str::{self, contains},
    };
    use std::fs;
    use tempfile::tempdir;

    pub const PROJECT_NAME: &str = "lit-app";

    #[test]
    fn test_resolve_and_validate_project_name_as_empty() {
        let tmp_cwd = tempdir().unwrap();

        let cmd = NewCommand {
            name: "".to_string(),
        };
        let result = cmd.resolve_and_validate_project_name(tmp_cwd.path());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Project name cannot be empty"
        );
    }

    #[test]
    fn test_resolve_and_validate_project_name_with_whitespace() {
        let tmp_cwd = tempdir().unwrap();

        let cmd = NewCommand {
            name: "   ".to_string(),
        };
        let result = cmd.resolve_and_validate_project_name(tmp_cwd.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_and_validate_project_name_file_extension_rejected() {
        let tmp_cwd = tempdir().unwrap();

        let cmd = NewCommand {
            name: format!("{}.txt", PROJECT_NAME),
        };
        let result = cmd.resolve_and_validate_project_name(tmp_cwd.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is a file"));
    }

    #[test]
    fn test_resolve_and_validate_project_name_existing_file_error() {
        let tmp_cwd = tempdir().unwrap();

        // Create a file where the directory should be
        fs::File::create(tmp_cwd.path().join(PROJECT_NAME)).unwrap();

        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let result = cmd.resolve_and_validate_project_name(tmp_cwd.path());
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("already exists and is a file")
        );
    }

    #[test]
    fn test_resolve_and_validate_project_name_directory_not_empty_error() {
        let tmp_cwd = tempdir().unwrap();

        let project_path = tmp_cwd.path().join(PROJECT_NAME);
        fs::create_dir(&project_path).unwrap();
        fs::File::create(project_path.join("index.js")).unwrap();

        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let result = cmd.resolve_and_validate_project_name(tmp_cwd.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is not empty"));
    }

    #[test]
    fn test_resolve_and_validate_project_name_git_initialized_ok() {
        let tmp_cwd = tempdir().unwrap();

        let destination_path = tmp_cwd.path().join(PROJECT_NAME);
        fs::create_dir(&destination_path).unwrap();
        fs::create_dir(destination_path.join(".git")).unwrap();

        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let result = cmd.resolve_and_validate_project_name(tmp_cwd.path());
        assert!(result.is_ok(), "Validation failed: {:?}", result.err());
    }

    #[test]
    fn test_resolve_and_validate_project_name_success_new_directory() {
        let tmp_cwd = tempdir().unwrap();

        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let result = cmd.resolve_and_validate_project_name(tmp_cwd.path());
        assert!(result.is_ok(), "Validation failed: {:?}", result.err());
    }

    #[test]
    fn test_configure_project_success() {
        let mut mock_ui = MockUserInterface::new();

        // Mock Language Selection: TypeScript (index 0)
        mock_ui
            .expect_render_select()
            .with(eq("Select a variant:"), any())
            .times(1)
            .returning(|_, _| Ok(0));

        // Mock Package Manager Selection: Pnpm (index 1)
        mock_ui
            .expect_render_select()
            .with(eq("Select Package Manager:"), any())
            .times(1)
            .returning(|_, _| Ok(1));

        // Mock Routing Confirmation: Yes (true)
        mock_ui
            .expect_render_confirm()
            .with(eq("Use `lit-router` for client side routing?"))
            .times(1)
            .returning(|_| Ok(true));

        // Mock Extras Multi-select: Utils and Docs (indices 0 and 2)
        mock_ui
            .expect_render_multi_select()
            .with(contains("Select extras"), any())
            .times(1)
            .returning(|_, _| Ok(vec![0, 2]));

        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };
        let result = cmd.configure_project(&mock_ui).unwrap();

        assert_eq!(result.language, Language::TypeScript);
        assert_eq!(result.package_manager, PackageManager::Pnpm);
        assert!(result.routing);

        let extras = result.extras.as_ref().expect("Extras should be present");
        assert_eq!(extras.len(), 2);

        assert!(
            result.has_extra(|e| matches!(e, Extra::Utils(true))),
            "Utils should be present"
        );
        assert!(
            result.has_extra(|e| matches!(e, Extra::Docs(true))),
            "Docs should be present"
        );
        assert!(
            !result.has_extra(|e| matches!(e, Extra::Shared(_))),
            "Shared should NOT be selected"
        );
    }

    #[test]
    fn test_generate_files_success() {
        let tmp_cwd = tempdir().unwrap();
        let destination_path = tmp_cwd.path().join(PROJECT_NAME);

        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let project_config = ProjectConfig {
            language: Language::JavaScript,
            package_manager: PackageManager::Npm,
            extras: None,
            routing: false,
        };

        let result = cmd.generate_files(&destination_path, &project_config);
        assert!(result.is_ok(), "File generation failed: {:?}", result.err());
    }

    #[test]
    fn test_get_js_element_template_base_mappings() {
        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let project_config = ProjectConfig {
            language: Language::JavaScript,
            package_manager: PackageManager::Npm,
            extras: None,
            routing: false,
        };

        let path_source_mappings = cmd.get_template_base_mappings(&project_config);
        let expected_mappings = vec![
            (
                PathSource::Static("types/index.d.js"),
                PathSource::Static("src/index.d.js"),
            ),
            (
                PathSource::Dynamic(format!(
                    "components/my-element.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/my-element.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Static("json/jsconfig.json"),
                PathSource::Static("jsconfig.json"),
            ),
        ];

        for expected in &expected_mappings {
            assert!(
                path_source_mappings.contains(expected),
                "Mappings missing expected pair: {:?}",
                expected
            );
        }
    }

    #[test]
    fn test_get_ts_element_template_base_mappings() {
        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let project_config = ProjectConfig {
            language: Language::TypeScript,
            package_manager: PackageManager::Npm,
            extras: None,
            routing: false,
        };

        let path_source_mappings = cmd.get_template_base_mappings(&project_config);
        let expected_mappings = vec![
            (
                PathSource::Static("types/index.ts"),
                PathSource::Static("src/types/index.ts"),
            ),
            (
                PathSource::Dynamic(format!(
                    "components/my-element.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/my-element.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Static("json/tsconfig.json"),
                PathSource::Static("tsconfig.json"),
            ),
        ];

        for expected in &expected_mappings {
            assert!(
                path_source_mappings.contains(expected),
                "Mappings missing expected pair: {:?}",
                expected
            );
        }
    }

    #[test]
    fn test_get_js_lit_router_template_base_mappings() {
        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let project_config = ProjectConfig {
            language: Language::JavaScript,
            package_manager: PackageManager::Npm,
            extras: None,
            routing: true,
        };

        let lit_router_template_category = TemplateCategory::LitRouter;
        let path_source_mappings = cmd.get_template_base_mappings(&project_config);
        let expected_mappings = vec![
            (
                PathSource::Dynamic(format!(
                    "{}/html/index.html",
                    lit_router_template_category.as_str()
                )),
                PathSource::Static("index.html"),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/app.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/app.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/main.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/main.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/pages/not-found.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/pages/not-found.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/pages/about.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/pages/about.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/pages/home.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/pages/home.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "components/header.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/components/header.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "components/footer.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/components/footer.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Static("types/index.d.js"),
                PathSource::Static("src/index.d.js"),
            ),
            (
                PathSource::Static("json/jsconfig.json"),
                PathSource::Static("jsconfig.json"),
            ),
        ];

        for expected in &expected_mappings {
            assert!(
                path_source_mappings.contains(expected),
                "Mappings missing expected pair: {:?}",
                expected
            );
        }
    }

    #[test]
    fn test_get_ts_lit_router_template_base_mappings() {
        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let project_config = ProjectConfig {
            language: Language::TypeScript,
            package_manager: PackageManager::Npm,
            extras: None,
            routing: true,
        };

        let lit_router_template_category = TemplateCategory::LitRouter;
        let path_source_mappings = cmd.get_template_base_mappings(&project_config);
        let expected_mappings = vec![
            (
                PathSource::Dynamic(format!(
                    "{}/html/index.html",
                    lit_router_template_category.as_str()
                )),
                PathSource::Static("index.html"),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/app.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/app.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/main.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/main.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/pages/not-found.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/pages/not-found.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/pages/about.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/pages/about.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "{}/pages/home.{}",
                    lit_router_template_category.as_str(),
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/pages/home.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "components/header.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/components/header.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "components/footer.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/components/footer.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Static("types/index.ts"),
                PathSource::Static("src/types/index.ts"),
            ),
            (
                PathSource::Static("json/tsconfig.json"),
                PathSource::Static("tsconfig.json"),
            ),
        ];

        for expected in &expected_mappings {
            assert!(
                path_source_mappings.contains(expected),
                "Mappings missing expected pair: {:?}",
                expected
            );
        }
    }

    #[test]
    fn test_get_template_base_mappings_with_extras() {
        let cmd = NewCommand {
            name: PROJECT_NAME.to_string(),
        };

        let project_config = ProjectConfig {
            language: Language::JavaScript,
            package_manager: PackageManager::Npm,
            extras: Some(vec![
                Extra::Utils(true),
                Extra::Shared(true),
                Extra::Docs(true),
            ]),
            routing: false,
        };

        let path_source_mappings = cmd.get_template_base_mappings(&project_config);
        let expected_mappings = vec![
            (
                PathSource::Dynamic(format!(
                    "utils/helpers.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/utils/helpers.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "shared/styles/buttonStyles.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/shared/styles/buttonStyles.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "shared/styles/linkStyles.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/shared/styles/linkStyles.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "shared/ui/card.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/shared/ui/card.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Dynamic(format!(
                    "shared/ui/loading-indicator.{}",
                    project_config.language.get_file_extension()
                )),
                PathSource::Dynamic(format!(
                    "src/shared/ui/loading-indicator.{}",
                    project_config.language.get_file_extension()
                )),
            ),
            (
                PathSource::Static("docs/CHANGELOG.md"),
                PathSource::Static("CHANGELOG.md"),
            ),
            (
                PathSource::Static("docs/CODE_OF_CONDUCT.md"),
                PathSource::Static("CODE_OF_CONDUCT.md"),
            ),
            (
                PathSource::Static("docs/CONTRIBUTING.md"),
                PathSource::Static("CONTRIBUTING.md"),
            ),
            (
                PathSource::Static("docs/README.md"),
                PathSource::Static("README.md"),
            ),
            (
                PathSource::Static("docs/SECURITY.md"),
                PathSource::Static("SECURITY.md"),
            ),
        ];

        for expected in &expected_mappings {
            assert!(
                path_source_mappings.contains(expected),
                "Mappings missing expected pair: {:?}",
                expected
            );
        }
    }
}
