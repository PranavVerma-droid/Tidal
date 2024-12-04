use rust_embed::RustEmbed;
use std::error::Error;
use termimad::*;
use colored::*;

#[derive(RustEmbed)]
#[folder = "src/embedded_docs/"]
struct DocAssets;

#[derive(Debug, Clone)]
pub struct RuntimeWikiPage {
    pub title: String,
    pub content: String,
}

pub fn fetch_docs() -> Result<Vec<RuntimeWikiPage>, Box<dyn Error>> {
    let mut pages = Vec::new();
    
    for file in DocAssets::iter() {
        let title = file.replace(".md", "").replace("_", " ");
        if let Some(content) = DocAssets::get(&file) {
            let content_str = String::from_utf8_lossy(content.data.as_ref()).to_string();
            pages.push(RuntimeWikiPage {
                title,
                content: content_str,
            });
        }
    }

    Ok(pages)
}

pub fn display_docs(pages: &[RuntimeWikiPage], page_num: usize) {
    if page_num == 0 || page_num > pages.len() {
        println!("{}", "Invalid page number!".red());
        return;
    }

    let page = &pages[page_num - 1];
    println!("\n{}", format!("# {}", page.title).bright_green());
    println!("{}", "-".repeat(40));

    let skin = MadSkin::default();
    skin.print_text(&page.content);
}

pub fn list_pages(pages: &[RuntimeWikiPage]) {
    println!("\n{}", "Available Documentation Pages:".bright_green());
    println!("{}", "-".repeat(40));
    
    for (i, page) in pages.iter().enumerate() {
        println!("{}. {}", i + 1, page.title);
    }
    println!("\nUse 'td docs --pg <number>' to view a specific page");
}
