use crate::{
    commands::{Commands, GptCommands, Oneshot},
    error::Result,
};
use chatgpt::prelude::*;
use clap::Parser;

mod commands;
mod error;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let openai_key = std::env::var("OPENAI_KEY").expect("OPENAI_KEY not set");

    let args = commands::Args::parse();
    match args.command {
        Commands::Gpt(gpt) => {
            let client = ChatGPT::new(openai_key)?;

            match gpt.command {
                GptCommands::Oneshot(Oneshot { prompt, input, .. }) => {
                    println!("prompt: {:?}", prompt);
                    let response = client.send_message(input).await?;

                    println!("response: {:?}", response);
                }
                GptCommands::Conversation => {
                    todo!()
                }
            }
        }

        Commands::Util(util) => match util.command {
            commands::UtilityCommands::GeneratePrompt { repo } => {
                let prompt = util::generate_repo_prompt(repo)?;

                print!("{}", prompt);
            }
        },
    }

    Ok(())
}
