use anyhow::{Context, Result};
use clap::Args;

use crate::{app, distribution, process};

/// Directly invoke the installed Python
#[derive(Args, Debug)]
#[command(hide = env!("PYAPP_EXPOSE_PYTHON") == "0", disable_help_flag = true)]
pub struct Cli {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        let installation_directory = app::installation_directory();
        distribution::ensure_ready(&installation_directory)?;

        let mut command = distribution::python_command(&app::python_path(&installation_directory));
        command.args(self.args);

        process::exec(command)
            .with_context(|| "Python execution failed, consider restoring project from scratch")
    }
}
