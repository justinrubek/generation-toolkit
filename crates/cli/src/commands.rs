use crate::commands::stable::StableDiffusion;
use chatgpt::config::ChatGPTEngine;

pub(crate) mod stable;

#[derive(Clone, Debug)]
pub(crate) enum ChatGptEngine {
    Gpt35Turbo,
    Gpt4,
    Gpt4_32k,
}

impl From<String> for ChatGptEngine {
    fn from(s: String) -> Self {
        match s.as_str() {
            "gpt3.5-turbo" => Self::Gpt35Turbo,
            "gpt4" => Self::Gpt4,
            "gpt4-32k" => Self::Gpt4_32k,
            _ => panic!("invalid engine"),
        }
    }
}

impl From<ChatGptEngine> for ChatGPTEngine {
    fn from(engine: ChatGptEngine) -> Self {
        match engine {
            ChatGptEngine::Gpt35Turbo => Self::Gpt35Turbo,
            ChatGptEngine::Gpt4 => Self::Gpt4,
            ChatGptEngine::Gpt4_32k => Self::Gpt4_32k,
        }
    }
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Gpt(Gpt),
    StableDiff(StableDiffusion),
    Util(Utility),
}

/// Generate text using OpenAI's GPT API.
#[derive(clap::Args, Debug)]
pub(crate) struct Gpt {
    #[clap(subcommand)]
    pub command: GptCommands,

    /// Which model to use.
    #[clap(long, short, default_value = "gpt3.5-turbo")]
    pub engine: ChatGptEngine,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum GptCommands {
    Oneshot(Oneshot),
    Conversation,
}

#[derive(clap::Args, Debug)]
pub(crate) struct Oneshot {
    /// Optional system prompt to start the conversation.
    #[clap(long, short)]
    pub prompt: Option<String>,
    /// User input to provide to the model.
    #[clap()]
    pub input: String,
    /// optional token limit to use when generating the response.
    #[clap(long, short)]
    pub token_limit: Option<usize>,
}

/// Convenience utilities for working with the text generation models.
#[derive(clap::Args, Debug)]
pub(crate) struct Utility {
    #[clap(subcommand)]
    pub command: UtilityCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum UtilityCommands {
    /// Package a git repository into text that can be used with a prompt.
    GeneratePrompt {
        /// The path to a git repisory to package.
        #[clap()]
        repo: std::path::PathBuf,
    },
}
