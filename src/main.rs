use std::fs;
use comrak::{markdown_to_html, ComrakOptions};

static REPO_ROOT: &'static str = "/Users/arya/Documents/blog";
static BASE_URL: &'static str = "https://arya-k.netlify.com";

fn main() {
    // First, remove the compiled assets
    println!("Removing compiled assets...");
    let dir = fs::read_dir(format!("{}/compiled", REPO_ROOT)).unwrap();
    for entry in dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if ! path.is_dir() && path.extension().unwrap() == "html" {
                fs::remove_file(path).expect("Failed to remove a file");
            }
        }
    }

    // Add support for all github extensions on Markdown
    let comrak_options = ComrakOptions {
        unsafe_: true,
        default_info_string: Some("bash".into()),
        ext_strikethrough: true,
        ext_table: true,
        ext_autolink: true,
        ext_tasklist: true,
        ..ComrakOptions::default()
    };

    // Gather the head and tail strings:
    let header = fs::read_to_string(format!("{}/compiled/assets/head.html", REPO_ROOT))
                    .expect("Unable to read header")
                    .replace("{{ BASE_URL }}", BASE_URL);
    let footer = "</body>\n</html>";

    // Start parsing through all the posts:
    let posts = fs::read_dir(format!("{}/posts", REPO_ROOT)).unwrap();
    for post in posts {
        let safe_post = post.unwrap();
        println!("Compiling {:?}", safe_post.file_name());

        let md = fs::read_to_string(safe_post.path()).expect("Unable to read file");

        if ! md.starts_with("# ") {
            println!("Skipping {:?} - Missing title", safe_post.file_name());
            continue;
        }

        if ! md.contains("\n") {
            println!("Skipping {:?} - Too short", safe_post.file_name());
            continue;
        }

        let mut lines = md.lines();
        let title = &(lines.next().unwrap())[2..];
        let rawdate = lines.next().unwrap();

        if ! rawdate.starts_with("<time>") {
            println!("Skipping {:?} - Missing date", safe_post.file_name());
            continue;
        }

        let html = markdown_to_html(&md, &comrak_options);
        let fullhtml = format!("{}{}{}", header.replace("{{ TITLE }}", title), html, footer);

        let finalpost = fullhtml.replace("<p><time>", "<time>").replace("</time></p>", "</time>");

        let goal_path = safe_post.path().with_extension("html");
        fs::write(format!("{}/compiled/{}", REPO_ROOT, goal_path.file_name().unwrap().to_str().unwrap()), finalpost).expect("Unable to write file");
    }
}
