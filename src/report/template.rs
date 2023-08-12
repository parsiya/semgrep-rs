use crate::error::Result;

/// Execute a handlebars compatible template and return the result.
pub fn execute_template(template: &str, data: &Vec<u8>) -> Result<String> {
    // 0. verify the template.
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_template_string("template", template)?;

    // 1. apply the template to the data.
    let result = handlebars.render("template", data)?;
    Ok(result)
}
