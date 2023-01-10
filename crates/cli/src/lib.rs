extern crate core;

use crate::dataset::ResourceTypeError;
use std::result;
use thiserror::Error;

pub mod cli_parser;
pub mod dataset;
pub mod schemas;
pub mod validator;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Validate(String),
    Seed,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CliError {
    #[error("path not found ({0})")]
    PathNotFound(String),
    #[error("invalid resource type {0}")]
    InvalidResourceType(#[from] ResourceTypeError),
}

impl CliError {
    pub fn display(self) {
        eprintln!("**ERROR** {}", self);
    }
}

pub type Result<T> = result::Result<T, CliError>;
