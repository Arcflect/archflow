use std::collections::BTreeMap;

/// Template rendering capability boundary.
///
/// Intent-focused contract: render named templates from key/value context.
#[allow(dead_code)]
pub trait TemplateRendererPort {
    type Error;

    fn render(
        &self,
        template_name: &str,
        variables: &BTreeMap<String, String>,
    ) -> Result<String, Self::Error>;
}
