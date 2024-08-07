use std::{collections::HashSet, fmt::Display};

use changelog::ser::{ChangeLogSerOption, ChangeLogSerOptionRelease};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub map: MapMessageToSection,
}

impl Default for Config {
    fn default() -> Self {
        let map = include_str!("../res/map_commit_type_to_section.json");
        serde_json::de::from_str(map).unwrap()
    }
}

impl Config {
    #[inline]
    pub fn into_changelog_ser_options(self) -> changelog::ser::ChangeLogSerOption {
        self.map.into_changelog_ser_options()
    }
}

#[derive(clap::ValueEnum, Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommitMessageParsing {
    #[default]
    Smart,
    Strict,
}

impl Display for CommitMessageParsing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitMessageParsing::Smart => write!(f, "smart"),
            CommitMessageParsing::Strict => write!(f, "strict"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMessageToSection(pub IndexMap<String, HashSet<String>>);

impl MapMessageToSection {
    pub fn into_changelog_ser_options(self) -> ChangeLogSerOption {
        ChangeLogSerOption {
            release_option: ChangeLogSerOptionRelease {
                section_order: self.0.into_iter().map(|(section, _)| section).collect(),
                ..Default::default()
            },
        }
    }

    pub fn map_section(&self, section: &str) -> Option<String> {
        let section_normalized = section.to_lowercase();

        for (section, needles) in &self.0 {
            for needle in needles {
                let needle_normalized = needle.to_lowercase();

                if section_normalized == needle_normalized {
                    return Some(section.to_owned());
                }
            }
        }

        None
    }

    /// Best effort recognition
    pub fn try_find_section(&self, (message, desc): (&str, &str)) -> Option<String> {
        let message_normalized = message.to_lowercase();
        let desc_normalized = desc.to_lowercase();

        for (section, needles) in &self.0 {
            for needle in needles {
                let needle_normalized = needle.to_lowercase();

                if message_normalized.contains(&needle_normalized) {
                    return Some(section.to_owned());
                }
                if desc_normalized.contains(&needle_normalized) {
                    return Some(section.to_owned());
                }
            }
        }

        None
    }
}
