use std::collections::BTreeMap;

pub struct SimpleTemplateRendererAdapter;

impl crate::ports::TemplateRendererPort for SimpleTemplateRendererAdapter {
    type Error = String;

    fn render(
        &self,
        template_name: &str,
        variables: &BTreeMap<String, String>,
    ) -> Result<String, Self::Error> {
        // Minimal adapter: treat template_name as template body for now.
        // Real file/template-engine integration can replace this adapter later.
        let mut rendered = template_name.to_string();
        for (key, value) in variables {
            let marker = format!("{{{{{}}}}}", key);
            rendered = rendered.replace(&marker, value);
        }
        Ok(rendered)
    }
}
