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
            let chatgpt_config = ModelConfigurationBuilder::default()
                .engine(gpt.engine)
                .build()?;

            let client = ChatGPT::new_with_config(openai_key, chatgpt_config)?;

            match gpt.command {
                GptCommands::Oneshot(Oneshot { prompt, input, .. }) => {
                    println!("prompt: {:?}", prompt);
                    let response = if let Some(prompt) = prompt {
                        let mut conversation = client.new_conversation_directed(prompt);

                        conversation.send_message(input).await?
                    } else {
                        client.send_message(input).await?
                    };

                    print!("{}", response.message().content);
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

        Commands::StableDiff(stable) => {
            stable.run().await?;
        }
    }

    Ok(())
}
