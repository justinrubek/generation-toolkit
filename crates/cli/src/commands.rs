#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Gpt(Gpt),
    Util(Utility),
}

/// Generate text using OpenAI's GPT API.
#[derive(clap::Args, Debug)]
pub(crate) struct Gpt {
    #[clap(subcommand)]
    pub command: GptCommands,
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
