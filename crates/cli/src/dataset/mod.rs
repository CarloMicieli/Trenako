use crate::{CliError, Result};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::{fmt, result};
use std::{fs, str};
use thiserror::Error;
use walkdir::WalkDir;

/// It represent a dataset
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Dataset {
    /// the brands resources (path contains `brands`)
    pub brands: Vec<Resource>,
    /// the catalog items resources (path contains `catalog_items`)
    pub catalog_items: Vec<Resource>,
    /// the railways resources (path contains `railways`)
    pub railways: Vec<Resource>,
    /// the scale resources (path contains `scales`)
    pub scales: Vec<Resource>,
}

impl Dataset {
    /// Creates a new Dataset from a path
    pub fn from_path(root: &str) -> Result<Dataset> {
        let resources = read_files(root)?;

        let mut dataset = Dataset::default();
        for resource in resources {
            match resource.resource_type {
                ResourceType::Brands => dataset.brands.push(resource),
                ResourceType::CatalogItems => dataset.catalog_items.push(resource),
                ResourceType::Railways => dataset.railways.push(resource),
                ResourceType::Scales => dataset.scales.push(resource),
            }
        }

        Ok(dataset)
    }
}

impl Display for Dataset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} brand(s)\n{} catalog item(s)\n{} railway(s)\n{} scale(s)",
            self.brands.len(),
            self.catalog_items.len(),
            self.railways.len(),
            self.scales.len()
        )
    }
}

fn read_files(root: &str) -> Result<Vec<Resource>> {
    let root_path = Path::new(root);

    if !root_path.exists() {
        return Err(CliError::PathNotFound(String::from(root)));
    }

    let mut results: Vec<Resource> = vec![];
    for entry in WalkDir::new(root_path).min_depth(0).max_depth(4) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_name = entry.path().file_name().and_then(|x| x.to_str()).map(|x| x.to_string());
            let content = fs::read_to_string(entry.path()).unwrap();
            let resource_type = resource_type_from_path(entry.path())?;

            results.push(Resource {
                file_name,
                content,
                resource_type,
            });
        }
    }
    Ok(results)
}

fn resource_type_from_path(path: &Path) -> result::Result<ResourceType, ResourceTypeError> {
    for component in path.components() {
        if let Some(resource_type) = ResourceType::from_os_str(component.as_os_str()) {
            return Ok(resource_type);
        }
    }

    Err(ResourceTypeError::ResourceTypeNotFound(String::from("")))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Resource {
    pub file_name: Option<String>,
    pub resource_type: ResourceType,
    pub content: String,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.resource_type, self.file_name)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResourceType {
    Brands,
    CatalogItems,
    Railways,
    Scales,
}

impl ResourceType {
    fn from_os_str(input: &OsStr) -> Option<ResourceType> {
        if let Some(s) = input.to_str() {
            match s {
                "brands" => Some(ResourceType::Brands),
                "catalog_items" => Some(ResourceType::CatalogItems),
                "railways" => Some(ResourceType::Railways),
                "scales" => Some(ResourceType::Scales),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ResourceTypeError {
    #[error("unable to determine the resource type (path: {0})")]
    ResourceTypeNotFound(String),
}
