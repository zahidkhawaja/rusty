use langchain_rust::chain::chain_trait::Chain;
use langchain_rust::chain::llm_chain::LLMChainBuilder;
use langchain_rust::llm::openai::{OpenAI, OpenAIModel};
use langchain_rust::prompt::{HumanMessagePromptTemplate, MessageOrTemplate};
use langchain_rust::{message_formatter, prompt_args, template_fstring};
use std::env;
use std::error::Error;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Check for environment variable OPENAI_API_KEY

    let prompt = HumanMessagePromptTemplate::new(template_fstring!(
        "Given text, return 1 bash command.
You should only answer with the Command.

Text:{input}
    ",
        "input"
    ));

    let formatter = message_formatter![MessageOrTemplate::Template(prompt.into()),];
    let llm = OpenAI::default().with_model(OpenAIModel::Gpt35);
    let chain = LLMChainBuilder::new()
        .prompt(formatter)
        .llm(llm)
        .build()
        .expect("Failed to build LLMChain");

    let user_input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    // If no arguments were provided, ask for user input
    let user_input = if user_input.is_empty() {
        print!("Enter prompt: ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read prompt.");
        input
    } else {
        user_input
    };

    let input_variables = prompt_args! {
        "input" => &user_input,

    };

    let res = chain.invoke(input_variables).await?;

    println!("{}", res);

    Ok(())
}
