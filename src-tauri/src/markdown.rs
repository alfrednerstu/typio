use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedMarkdown {
    pub frontmatter: HashMap<String, serde_json::Value>,
    pub content: String,
    pub wikilinks: Vec<String>,
}

/// Parse a markdown file, extracting frontmatter and wikilinks.
pub fn parse_markdown(raw: &str) -> ParsedMarkdown {
    let (frontmatter, content) = extract_frontmatter(raw);
    let wikilinks = extract_wikilinks(&content);

    ParsedMarkdown {
        frontmatter,
        content,
        wikilinks,
    }
}

/// Extract YAML frontmatter from markdown content.
fn extract_frontmatter(raw: &str) -> (HashMap<String, serde_json::Value>, String) {
    let trimmed = raw.trim_start();
    if !trimmed.starts_with("---") {
        return (HashMap::new(), raw.to_string());
    }

    // Find the closing ---
    if let Some(end) = trimmed[3..].find("\n---") {
        let yaml_str = &trimmed[3..end + 3].trim();
        let rest = &trimmed[end + 3 + 4..]; // skip past closing ---

        let frontmatter: HashMap<String, serde_json::Value> =
            match serde_yaml::from_str(yaml_str) {
                Ok(map) => map,
                Err(_) => HashMap::new(),
            };

        (frontmatter, rest.to_string())
    } else {
        (HashMap::new(), raw.to_string())
    }
}

/// Extract all [[wikilinks]] from markdown content.
pub fn extract_wikilinks(content: &str) -> Vec<String> {
    let re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    re.captures_iter(content)
        .map(|cap| {
            let link = cap[1].to_string();
            // Handle [[link|display text]] format
            if let Some(idx) = link.find('|') {
                link[..idx].trim().to_string()
            } else {
                link.trim().to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_wikilinks() {
        let content = "See [[Page One]] and [[Page Two|display]] for more.";
        let links = extract_wikilinks(content);
        assert_eq!(links, vec!["Page One", "Page Two"]);
    }

    #[test]
    fn test_extract_frontmatter() {
        let raw = "---\ntitle: Hello\ntags:\n  - rust\n  - tauri\n---\n# Content here";
        let (fm, content) = extract_frontmatter(raw);
        assert_eq!(
            fm.get("title").unwrap(),
            &serde_json::Value::String("Hello".to_string())
        );
        assert!(content.contains("# Content here"));
    }
}
