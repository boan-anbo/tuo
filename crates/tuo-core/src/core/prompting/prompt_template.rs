use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::core::prompting::prompt::Prompt;

#[async_trait]
pub trait PromptTemplateTrait {
    async fn get_prompt(&self) -> Prompt;
}

pub struct PromptTemplate {
    pub template: String,
    pub var_map: HashMap<String, String>,
    /// The left delimiter for the variable
    ///
    /// Default is `[[`
    pub var_left_delimiter: String,

    /// The right delimiter for the variable
    ///
    /// Default is `]]`
    pub var_right_delimiter: String,
}

#[async_trait]
impl PromptTemplateTrait for PromptTemplate {
    async fn get_prompt(&self) -> Prompt {
        // Create the Regex inside the method instead of as a static variable
        let left_delimiter = regex::escape(&self.var_left_delimiter);
        let right_delimiter = regex::escape(&self.var_right_delimiter);
        let re = Regex::new(&format!(r"{}\S*{}", left_delimiter, right_delimiter)).unwrap();
        let mut prompt_template = self.template.clone();
        for cap in re.captures_iter(prompt_template.clone().as_str()) {
            let var = cap.get(0).unwrap().as_str();
            let var_name = var.trim_start_matches(&self.var_left_delimiter).trim_end_matches(&self.var_right_delimiter);
            let var_value = self.var_map.get(var_name.to_lowercase().as_str()).unwrap();
            prompt_template = prompt_template.replace(var, var_value);
        }

        Prompt::new_system_prompt_by_user(prompt_template)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::core::prompting::prompt::Prompt;
    use crate::core::prompting::prompt_template::{PromptTemplate, PromptTemplateTrait};

    #[tokio::test]
    async fn test_prompt_template() {
        let mut var_map = HashMap::new();
        var_map.insert("name".to_string(), "John".to_string());
        var_map.insert("age".to_string(), "25".to_string());
        let prompt_template = PromptTemplate {
            template: "Hello, [[name]]. Your name is [[Name]]. You are [[age]] years old.".to_string(),
            var_map,
            var_left_delimiter: "[[".to_string(),
            var_right_delimiter: "]]".to_string(),
        };
        let prompt = prompt_template.get_prompt().await;
        assert_eq!(prompt.text, Prompt::new_system_prompt_by_user("Hello, John. Your name is John. You are 25 years old.".to_string()).text);
    }
}



