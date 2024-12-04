use std::fs;
use std::path::Path;
use reqwest::blocking::Client;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    let docs_dir = Path::new("src").join("embedded_docs");
    fs::create_dir_all(&docs_dir).expect("Failed to create embedded_docs directory");

    let wiki_pages = [
        // vec of all docs on wiki
        ("Home", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Home.md"),
        ("Brain Rot Mode", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Brain-Rot-Mode-☠%EF%B8%8F.md"),
        ("Control Flow", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Control-Flow.md"),
        ("Data Types", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Data-Types.md"),
        ("File Extension", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/File-Extension.md"),
        ("Libraries", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Libraries.md"),
        ("Loops", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Loops.md"),
        ("Operators", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Operators.md"),
        ("Syntax", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Syntax.md"),
        ("Variables", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Variables.md"),
        ("Functions", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Functions.md"),
        ("Verbose Mode", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/Verbose-Mode.md"),
        ("For Developers", "https://raw.githubusercontent.com/wiki/Tidal-Lang/Tidal/For-Developers.md"),
    ];

    let client = Client::new();
    for (name, url) in wiki_pages.iter() {
        println!("Fetching documentation: {}", name);
        let content = client.get(*url)
            .send()
            .expect(&format!("Failed to fetch {}", name))
            .text()
            .expect(&format!("Failed to read content for {}", name));
        
        let file_path = docs_dir.join(format!("{}.md", name.replace(" ", "_")));
        fs::write(&file_path, content)
            .expect(&format!("Failed to write {}", file_path.display()));
    }

    println!("Successfully fetched all documentation files");
}