#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::error::Error;
use std::fs;

use serde::Deserialize;

// ─── Structure du Trie ───────────────────────────────────────────

#[derive(Default)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end: bool,
    pub name: Option<String>,
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, number: &str, name: &str) {
        let mut node = &mut self.root;
        for ch in number.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        node.is_end = true;
        node.name = Some(name.to_string());
    }
}

// ─── Désérialisation JSON ────────────────────────────────────────

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub phone: String,
}

pub fn load_users(filepath: &str) -> Result<Vec<User>, Box<dyn Error>> {
    let content = fs::read_to_string(filepath)?;
    let users: Vec<User> = serde_json::from_str(&content)?;
    Ok(users)
}

// ─── Génération PlantUML MindMap ─────────────────────────────────

pub fn generate_plantuml(trie: &Trie) -> String {
    let mut output = String::from("@startmindmap\n* root\n");
    traverse_node(&trie.root, 1, &mut output);
    output.push_str("@endmindmap\n");
    output
}

fn traverse_node(node: &TrieNode, depth: usize, output: &mut String) {
    let mut keys: Vec<char> = node.children.keys().cloned().collect();
    keys.sort();
    for key in keys {
        let stars = "*".repeat(depth + 1);
        let child = &node.children[&key];
        let label = if child.is_end {
            if let Some(name) = &child.name {
                format!("{} {} ({})", stars, key, name)
            } else {
                format!("{} {} (end)", stars, key)
            }
        } else {
            format!("{} {}", stars, key)
        };
        output.push_str(&label);
        output.push('\n');
        traverse_node(child, depth + 1, output);
    }
}

// ─── Config ──────────────────────────────────────────────────────

pub struct Config {
    pub input_path: String,
    pub output_path: String,
}

impl Config {
    pub fn new(_args: &[String]) -> Result<Config, &'static str> {
        Ok(Config {
            input_path: String::from("data/04_common_parts.json"),
            output_path: String::from("graph/04_common_parts.puml"),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let users = load_users(&config.input_path)?;

    let mut trie = Trie::new();
    for user in &users {
        trie.insert(&user.phone, &user.name);
    }

    let puml = generate_plantuml(&trie);
    fs::write(&config.output_path, puml)?;

    println!("Fichier PlantUML généré : {}", config.output_path);
    Ok(())
}