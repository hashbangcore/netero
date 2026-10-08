use crate::core;
use crate::util::{self, attach, terminal};

pub async fn dispatch(
    service: &core::Service,
    args: &core::Cli,
    request: &str,
    stdin: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_lang = util::get_user_lang();
    let user_lang = util::normalize_lang_tag(&user_lang);
    let attachments = attach::extract_attachments_from_input(request);
    let stdin_content = stdin;
    let attached_files = attach::format_attached_files(
        if stdin_content.trim().is_empty() {
            None
        } else {
            Some(stdin_content.as_str())
        },
        &attachments,
    );

    let mut prompt = format!(
        "USER LANG: {} !important\n:: USER HINT ::\n{}\n:: END USER HINT ::",
        user_lang,
        request.trim()
    );
    if let Some(block) = attached_files {
        prompt.push_str("\n\n");
        prompt.push_str(&block);
    }

    let response = service.complete(&prompt).await?;

    if args.verbose {
        terminal::print_labeled("USER", &prompt);
        terminal::print_labeled("LLM", response.trim());
    } else {
        let output = args
            .output
            .as_deref()
            .and_then(terminal::OutputFormat::from_name);
        println!("{}", terminal::render_markdown_with(&response, output));
    }

    Ok(())
}
