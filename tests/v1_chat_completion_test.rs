use jrest::expect;
use mistralai_client::v1::{
    chat_completion::{
        ChatCompletionMessage, ChatCompletionMessageRole, ChatCompletionRequest,
        ChatCompletionRequestOptions,
    },
    client::Client,
    constants::OPEN_MISTRAL_7B,
};

#[test]
fn test_chat_completion() {
    extern crate dotenv;

    use dotenv::dotenv;
    dotenv().ok();

    let client = Client::new(None, None, None, None);

    let model = OPEN_MISTRAL_7B.to_string();
    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::user,
        content: "Just guess the next word: \"Eiffel ...\"?".to_string(),
    }];
    let options = ChatCompletionRequestOptions {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let chat_completion_request = ChatCompletionRequest::new(model, messages, Some(options));
    let result = client.chat(chat_completion_request);

    match result {
        Ok(res) => {
            expect!(res.model).to_be("open-mistral-7b".to_string());
            expect!(res.object).to_be("chat.completion".to_string());
            expect!(res.choices.len()).to_be(1);
            expect!(res.choices[0].index).to_be(0);
            expect!(res.choices[0].message.role.clone())
                .to_be(ChatCompletionMessageRole::assistant);
            expect!(res.choices[0].message.content.clone()).to_be(
                "Tower. The Eiffel Tower is a famous landmark in Paris, France.".to_string(),
            );
            expect!(res.usage.prompt_tokens).to_be_greater_than(0);
            expect!(res.usage.completion_tokens).to_be_greater_than(0);
            expect!(res.usage.total_tokens).to_be_greater_than(21);
        }
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}