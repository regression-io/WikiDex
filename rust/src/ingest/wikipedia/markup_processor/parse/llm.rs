use crate::{
    ingest::wikipedia::helper::wiki::{DescribedTable, UnlabledDocument},
    llm::{LlmInput, LlmMessage, LlmRole, LlmService, LlmServiceError, OpenAiService},
};

pub(crate) async fn process_table_to_llm(
    table: &str,
    client: &OpenAiService,
) -> Result<UnlabledDocument, LlmServiceError> {
    let message = LlmInput {
        system: String::from("Interpret and summarize the following HTML table in a concise, plain English description."),
        conversation: vec![LlmMessage{message: table.to_string(), role: LlmRole::User}],
    };
    let output = client.get_llm_answer(message).await?;

    let response = output
        .conversation
        .into_iter()
        .last()
        .and_then(|m| Some(m.message))
        .ok_or(LlmServiceError::EmptyResponse)?;

    Ok(UnlabledDocument::from_str_and_vec(
        String::new(),
        vec![DescribedTable {
            description: response,
            table: table.to_string(),
        }],
    ))
}
