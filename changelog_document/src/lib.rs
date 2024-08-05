use std::collections::HashMap;

use indexmap::IndexMap;

mod parser;
pub use parser::parse_changelog;

mod serializer;
pub use serializer::serialize_changelog;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseTitle {
    pub version: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseSection {
    pub title: String,
    pub notes: Vec<ReleaseSectionNote>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseSectionNote {
    pub component: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub title: ReleaseTitle,
    pub header: Option<String>,
    pub note_sections: HashMap<String, ReleaseSection>,
    pub footer: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FooterLink {
    pub text: String,
    pub link: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FooterLinks {
    pub links: Vec<FooterLink>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeLog {
    pub header: Option<String>,
    pub releases: IndexMap<String, Release>,
    pub footer_links: FooterLinks,
}

impl Default for ChangeLog {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangeLog {
    pub fn new() -> Self {
        let mut releases = IndexMap::new();

        let version = String::from("Unreleased");
        releases.insert(
            version.clone(),
            Release {
                title: ReleaseTitle {
                    version,
                    title: None,
                },
                header: None,
                note_sections: HashMap::new(),
                footer: None,
            },
        );

        ChangeLog {
            header: None,
            releases,
            footer_links: FooterLinks { links: Vec::new() },
        }
    }
}
