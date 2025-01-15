use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Category {
    pub name: String,
    pub extensions: Vec<String>,
    pub color: [f32; 3],
}

impl Category {
    pub fn new(name: String, extensions: Vec<String>, color: [f32; 3]) -> Self {
        Self {
            name,
            extensions,
            color,
        }
    }

    pub fn matches_extension(&self, extension: &str) -> bool {
        self.extensions.iter().any(|e| e == extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_matches_extension() {
        let category = Category::new(
            "Documents".to_string(),
            vec!["pdf".to_string(), "doc".to_string()],
            [0.2, 0.6, 1.0],
        );

        assert!(category.matches_extension("pdf"));
        assert!(category.matches_extension("doc"));
        assert!(!category.matches_extension("jpg"));
    }
}