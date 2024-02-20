use clap::Parser;
use colored::Colorize;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::{GPT3_5_TURBO, GPT4};
use termimad::crossterm::style::Color::*;
use termimad::*;

#[derive(Parser)]
struct Cli {
    gpt_version: String,
    open_ai_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let client = Client::new(args.open_ai_key);
    let mut gpt_version: String = GPT3_5_TURBO.to_string();
    let mut skin = MadSkin::default();
    match args.gpt_version.as_str() {
        "3.5" => {
            gpt_version = GPT3_5_TURBO.to_string();
        }
        "4" => {
            gpt_version = GPT4.to_string();
        }
        _ => {
            println!("Invalid GPT version. Using GPT-3.5 turbo by default.");
        }
    }

    let mut messages: Vec<chat_completion::ChatCompletionMessage> =
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text("Hello, how are you?".to_string()),
            name: None,
        }];

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim() {
            "exit" => break,
            "cls" => {
                messages.clear();
                print!("{esc}c", esc = 27 as char);
                println!("{}", "Chat history cleared.".green());
            }
            "clh" => {
                messages.clear();
                println!("{}", "Chat history cleared.".green());
            }
            _ => {
                messages.push(chat_completion::ChatCompletionMessage {
                    role: chat_completion::MessageRole::user,
                    content: chat_completion::Content::Text(input.trim().to_string()),
                    name: None,
                });
                let req = ChatCompletionRequest::new(gpt_version.clone(), messages.clone());
                let result = client.chat_completion(req)?;
                match &result.choices[0].message.content {
                    Some(content) => {
                        messages.push(chat_completion::ChatCompletionMessage {
                            role: chat_completion::MessageRole::assistant,
                            content: chat_completion::Content::Text(content.to_string()),
                            name: None,
                        });
                        let mut markdown = String::new();
                        skin.set_headers_fg(Yellow);
                        skin.bold.set_fg(Yellow);
                        skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
                        skin.paragraph.set_fg(Yellow);

                        markdown.push_str(&content.yellow());

                        println!("{}\n", skin.term_text(&markdown.yellow()));
                    }
                    None => {
                        println!("{}", "No response from GPT".red());
                    }
                }
            }
        }
    }
    Ok(())
}
