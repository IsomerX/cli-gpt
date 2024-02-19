use clap::Parser;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT3_5_TURBO;
use std::env;

#[derive(Parser)]
struct Cli {
    questions: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let args = Cli::parse();
    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(args.questions)),
            name: None,
        }],
    );
    let result = client.chat_completion(req)?;
    println!("{:?}", result.choices[0].message.content);
    Ok(())
}
