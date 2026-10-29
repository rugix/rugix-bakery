//! Read-only discovery and machine-readable definition catalogs.

use reportify::ResultExt;
use serde::Serialize;

use crate::cli::{args, load_project};
use crate::config::catalog::{DefinitionCatalog, DefinitionInfo, DefinitionKind};
use crate::project::catalog::Catalog;
use crate::BakeryResult;

/// Inspect definitions without planning builds or constructing environments.
pub fn run(
    args: &args::Args,
    command: &args::DefinitionsCommand,
    kind: DefinitionKind,
) -> BakeryResult<()> {
    let project = load_project(args)?;
    let catalog = Catalog::new(project.repositories()?);
    let output = match command {
        args::DefinitionsCommand::List { repository, json } => {
            let available = catalog.available(kind, repository.as_deref())?;
            if *json {
                let definitions = match kind {
                    DefinitionKind::Rule => DefinitionCatalog::Rule(
                        available
                            .iter()
                            .map(|name| catalog.rule(name))
                            .collect::<BakeryResult<_>>()?,
                    ),
                    DefinitionKind::Recipe => DefinitionCatalog::Recipe(
                        available
                            .iter()
                            .map(|name| catalog.recipe(name))
                            .collect::<BakeryResult<_>>()?,
                    ),
                    DefinitionKind::Layer => DefinitionCatalog::Layer(
                        available
                            .iter()
                            .map(|name| catalog.layer(name))
                            .collect::<BakeryResult<_>>()?,
                    ),
                };
                serde_json::to_string_pretty(&definitions)
                    .whatever("unable to serialize definition catalog")?
            } else {
                available.into_iter().collect::<Vec<_>>().join("\n")
            }
        }
        args::DefinitionsCommand::Inspect { name, json } => {
            let info = match kind {
                DefinitionKind::Rule => DefinitionInfo::Rule(catalog.rule(name)?),
                DefinitionKind::Recipe => DefinitionInfo::Recipe(catalog.recipe(name)?),
                DefinitionKind::Layer => DefinitionInfo::Layer(catalog.layer(name)?),
            };
            if *json {
                serde_json::to_string_pretty(&info)
                    .whatever("unable to serialize definition information")?
            } else {
                match &info {
                    DefinitionInfo::Rule(info) => {
                        display(&info.name, &info.definition, info.documentation.as_deref())?
                    }
                    DefinitionInfo::Recipe(info) => {
                        display(&info.name, &info.definition, info.documentation.as_deref())?
                    }
                    DefinitionInfo::Layer(info) => {
                        display(&info.name, &info.definition, info.documentation.as_deref())?
                    }
                }
            }
        }
    };
    rugix_cli::suspend(|| println!("{output}"));
    Ok(())
}

/// Keep human-readable inspection consistent across the three configuration types.
fn display<T: Serialize>(
    name: &str,
    definition: &T,
    documentation: Option<&str>,
) -> BakeryResult<String> {
    Ok(format!(
        "{name}\n\n{}\n{}",
        toml::to_string_pretty(definition).whatever("unable to render definition")?,
        documentation.unwrap_or("")
    ))
}
