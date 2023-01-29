extern crate core;

use crate::dataset::ResourceTypeError;
use clap::ValueEnum;
use std::result;
use thiserror::Error;

pub mod cli_parser;
pub mod csv_record;
pub mod cvs_files;
pub mod dataset;
pub mod schemas;
pub mod validator;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    CsvExport {
        source: String,
        output: String,
        dry_run: bool,
    },
    CsvImport {
        file: String,
        mode: Mode,
        output: String,
        dry_run: bool,
    },
    Seed,
    Validate(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    Json,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CliError {
    #[error("path not found ({0})")]
    PathNotFound(String),
    #[error("invalid resource type {0}")]
    InvalidResourceType(#[from] ResourceTypeError),
    #[error("filename is not a valid UTF-8")]
    InvalidFileName,
}

impl CliError {
    pub fn display(self) {
        eprintln!("**ERROR** {self}");
    }
}

pub type Result<T> = result::Result<T, CliError>;
