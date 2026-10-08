/// Builds the chat prompt body from already-resolved user, datetime, history, and input values.
pub fn create_prompt(
    username: &str,
    datetime: &str,
    user_lang: &str,
    history: &str,
    user_input: &str,
    command_output: Option<&str>,
    stdin_attachment: Option<&str>,
) -> String {
    let command_section = match command_output {
        Some(output) => format!(
            ":: COMMAND OUTPUT (SYSTEM) ::\n{}\n:: END COMMAND OUTPUT (SYSTEM) ::",
            output
        ),
        None => String::new(),
    };

    let stdin_section = match stdin_attachment {
        Some(content) => format!(
            ":: STDIN ATTACHMENT (SYSTEM) ::\n{}\n:: END STDIN ATTACHMENT (SYSTEM) ::",
            content
        ),
        None => String::new(),
    };

    // NOTE: user_lang should reflect the OS locale (e.g., LANG/LC_ALL).
    let extra_sections = [command_section, stdin_section]
        .into_iter()
        .filter(|section| !section.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let extra_block = if extra_sections.is_empty() {
        String::new()
    } else {
        format!("{extra_sections}\n")
    };

    format!(
        "LLM ROL: Conversational terminal assistant\nUSERNAME: {}\nDATETIME: {}\nUSER LANG: {}\n\
:: INSTRUCTION (SYSTEM) ::\n\
- Keep responses concise: 5-20 lines maximum.\n\
- Do not use emojis or decorations.\n\
- Always prioritize the latest user message over the HISTORICAL CHAT.\n\
- The latest message may be completely unrelated to previous messages.\n\
- Do not assume continuity or context from the history unless the user explicitly refers to it.\n\
:: END INSTRUCTION (SYSTEM) ::\n\
:: HISTORIAL CHAT (SYSTEM) ::\n\
{}\n\
:: END HISTORIAL CHAT (SYSTEM) ::\n\
{}\
:: USER MESSAGE ::\n\
{}\n\
:: END USER MESSAGE ::",
        username, datetime, user_lang, history, extra_block, user_input
    )
}
