use std::fs;
use std::vec::Vec;
use std::ffi::OsString;
use chrono::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};

static REPO_ROOT: &'static str = "/Users/arya/Documents/blog";
static BASE_URL: &'static str = "https://arya-k.netlify.com";

#[derive(Debug)]
struct Post {
    name: String,
    url: String,
    datestring: String,
    datecreated: i64
}

fn main() {
    // First, remove the compiled assets
    println!("Removing compiled assets...");
    let dir = fs::read_dir(format!("{}/compiled", REPO_ROOT)).unwrap();
    for entry in dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if ! path.is_dir() && path.extension().unwrap_or(&OsString::new()) == "html" {
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
    let mut post_structs: Vec<Post> = Vec::new();

    let posts = fs::read_dir(format!("{}/posts", REPO_ROOT)).unwrap();
    for post in posts {
        let safe_post = post.unwrap();
        println!("Compiling {:?}", safe_post.file_name());

        let md = fs::read_to_string(safe_post.path()).expect("Unable to read file");

        if ! md.starts_with("# ") { // if it doesn't have a title, ignore it.
            println!("Skipping {:?} - Missing title", safe_post.file_name());
            continue;
        }

        if ! md.contains("\n") { // if it can't have a rawdate, ignore it.
            println!("Skipping {:?} - Too short", safe_post.file_name());
            continue;
        }

        let mut lines = md.lines();
        let title = &(lines.next().unwrap())[2..];
        let rawdate = lines.next().unwrap();

        if ! rawdate.starts_with("<time>") { // if it doesn't have a date, ignore it.
            println!("Skipping {:?} - Missing date", safe_post.file_name());
            continue;
        }

        if Utc.datetime_from_str(&format!("{} 00:00:00", rawdate), "<time>%b %e, %Y</time> %T").is_err() {
            println!("Skipping {:?} - Malformed date", safe_post.file_name());
            continue;
        }

        // convert to markdown + generate file:
        let html = format!(
            "{}{}{}",
            header.replace("{{ TITLE }}", title),
            markdown_to_html(&md, &comrak_options)
                .replace("<p><time>", "<time>")
                .replace("</time></p>", "</time>"),
            footer
        );

        let goal_path = safe_post.path().with_extension("html");
        let goal_name = goal_path.file_name().unwrap().to_str().unwrap();
        fs::write(format!("{}/compiled/{}", REPO_ROOT, goal_name), html).expect("Unable to write file");

        // Gather info into a struct:
        post_structs.push(Post {
            name: title.to_string(),
            url: format!("{}/{}", BASE_URL, goal_name),
            datestring: rawdate.to_string(),
            datecreated: Utc.datetime_from_str(&format!("{} 00:00:00", rawdate), "<time>%b %e, %Y</time> %T").unwrap().timestamp()
        });
    }

    // sort the posts: most recent first.
    post_structs.sort_by_key(|x| -x.datecreated);

    println!("Generating index"); // this is done inline since it's pretty simple
    let mut index = header.replace("{{ TITLE }}", "arya-k").to_string(); // reuse header
    index.push_str("<h1 style=\"font-size:3em\">Posts</h1>\n"); // Posts at top of page

    for post in post_structs { // generate posts in order
        index.push_str(&format!(
            "<h1><a class=\"mainpage\" href=\"{}\">{}</a></h1>\n{}\n",
            post.url,
            markdown_to_html(&post.name, &comrak_options) // respect markdown in titles
                .replace("<p>", "").replace("</p>", ""),
            post.datestring
        ));
    }

    index.push_str(&fs::read_to_string(
        format!("{}/compiled/assets/index_footer.html", REPO_ROOT))
        .expect("Unable to read footer")); // this part is a custom footer

    fs::write(format!("{}/compiled/index.html", REPO_ROOT), index).expect("Unable to write index file");

    println!("Done!");
}
