use std::process::ExitStatus;

use anyhow::{anyhow, bail, Result};
use colored::Colorize;

#[derive(Debug)]
pub enum Command {
    ForbidUnsafe,
    ForbidCollections,
    ForbidStd,
    CargoFmt,
    CargoClippy,
    CargoTest,
    CargoTestDebug,
    CargoMiriTest,
    CargoCompileTestMiniFrunk,
    CargoCompileTestOrm,
    CargoCompileTestSnapshot,
    PythonTest,
}

impl Command {
    pub fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "forbid-unsafe" => Self::ForbidUnsafe,
            "forbid-collections" => Self::ForbidCollections,
            "forbid-std" => Self::ForbidStd,
            "cargo-fmt" => Self::CargoFmt,
            "cargo-clippy" => Self::CargoClippy,
            "cargo-test" => Self::CargoTest,
            "cargo-test-debug" => Self::CargoTestDebug,
            "cargo-miri-test" => Self::CargoMiriTest,
            "cargo-compile-test-mini-frunk" => Self::CargoCompileTestMiniFrunk,
            "cargo-compile-test-orm" => Self::CargoCompileTestOrm,
            "cargo-compile-test-snapshot" => Self::CargoCompileTestSnapshot,
            "python-test" => Self::PythonTest,
            name => bail!("command \"{name}\" is not supported"),
        })
    }

    pub fn get_shell_line(&self) -> Result<String> {
        Ok(match self {
            Self::ForbidUnsafe => bail!("no shell line for ForbidUnsafe"),
            Self::ForbidCollections => bail!("no shell line for ForbidCollections"),
            Self::ForbidStd => bail!("no shell line for ForbidStd"),
            Self::CargoFmt => "cargo fmt --check".to_string(),
            Self::CargoClippy => "cargo clippy --release -- -D warnings".to_string(),
            Self::CargoTest => "cargo test --release".to_string(),
            Self::CargoTestDebug => "cargo test".to_string(),
            Self::CargoMiriTest => "cargo miri test --release".to_string(),
            Self::CargoCompileTestMiniFrunk => bail!("no shell line for CargoCompileTestMiniFrunk"),
            Self::CargoCompileTestOrm => bail!("no shell line for CargoCompileTestOrm"),
            Self::CargoCompileTestSnapshot => bail!("no shell line for CargoCompileTestSnapshot"),
            Self::PythonTest => "python3 test.py".to_string(),
        })
    }

    pub fn get_failure_error(&self, _status: ExitStatus) -> anyhow::Error {
        match self {
            Self::CargoFmt => anyhow!(
                "Format your code as suggested above, \
                 or run `{}` to do it automatically",
                "cargo fmt".bold(),
            ),
            _ => anyhow!(
                "Command {} failed, see message above",
                format!("{self:?}").bold()
            ),
        }
    }
}
