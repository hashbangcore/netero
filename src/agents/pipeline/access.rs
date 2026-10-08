use crate::core;
use crate::utilities;
use crate::utilities::{attach, render};

pub async fn dispatch(
    service: &core::Service,
    args: &core::Cli,
    request: &str,
    stdin: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_lang = utilities::get_user_lang();
    let user_lang = utilities::normalize_lang_tag(&user_lang);
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
        render::print_labeled("USER", &prompt);
        render::print_labeled("LLM", response.trim());
    } else {
        println!(
            "{}",
            render::render_markdown_with(&response, args.output.as_ref())
        );
    }

    Ok(())
}
