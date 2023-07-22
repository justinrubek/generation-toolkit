use crate::Result;
use diffusers::{pipelines::stable_diffusion, transformers::clip};
use tch::{nn::Module, Device, Kind, Tensor};

const GUIDANCE_SCALE: f64 = 7.5;

/// Generate images using Stable Diffusion.
#[derive(clap::Args, Debug)]
pub(crate) struct StableDiffusion {
    #[clap(subcommand)]
    pub command: StableDiffusionCommands,

    #[clap(long, short, default_value = "models")]
    pub model_dir: std::path::PathBuf,
}

impl StableDiffusion {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            StableDiffusionCommands::Generate(generate) => {
                todo!()
            }
        }

        Ok(())
    }
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum StableDiffusionCommands {
    Generate(Generate),
}

#[derive(clap::Args, Debug)]
pub(crate) struct Generate {
    /// The path to a git repisory to package.
    #[clap()]
    pub repo: std::path::PathBuf,
}
