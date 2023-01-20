use async_openai::{types::CreateCompletionRequestArgs, Client};


//#[tokio::main]
pub async fn ai_response(question: String) -> String {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();
    //println!("I'm here!");
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(question)
        .max_tokens(120_u16)
        .build()
        .unwrap();

    let response = client.completions().create(request).await.unwrap();

    //println!("\nResponse (single):\n");
    let mut priority_response = String::new();
    for choice in response.choices {
        println!("{}", choice.text);
        priority_response = choice.text;
    }

    priority_response
}
