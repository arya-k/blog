use chrono::prelude::*;
use comrak::nodes::NodeValue;
use comrak::{format_html, markdown_to_html, parse_document, Arena, ComrakOptions};
use std::ffi::OsString;
use std::fs;
use std::vec::Vec;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static BASE_URL: &'static str = "https://blog.arya-k.dev";

#[derive(Debug)]
struct Post {
    name: String,
    url: String,
    datestring: String,
    datecreated: i64,
}

fn remove_compiled_assets() {
    println!("Removing compiled assets...");
    let dir = fs::read_dir("./compiled").unwrap();
    for entry in dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if !path.is_dir() && path.extension().unwrap_or(&OsString::new()) == "html" {
                fs::remove_file(path).expect("Failed to remove a file");
            }
        }
    }
}

fn validate_post(md: &str, file_name: &std::ffi::OsString, goal_name: &str) -> Option<Post> {
    if !md.starts_with("# ") {
        // if it doesn't have a title, ignore it.
        println!("Skipping {:?} - Missing title", file_name);
        return None;
    } else if !md.contains('\n') {
        // if it can't have a rawdate, ignore it.
        println!("Skipping {:?} - Too short", file_name);
        return None;
    }

    let mut lines = md.lines();
    let title = &(lines.next().unwrap())[2..];
    let rawdate = lines.next().unwrap();

    if !rawdate.starts_with("<time>") {
        // if it doesn't have a date, ignore it.
        println!("Skipping {:?} - Missing date", file_name);
        return None;
    }

    let parsed_date = Utc.datetime_from_str(
        &format!("{} 00:00:00", rawdate),
        "<time>%b %e, %Y</time> %T",
    );

    if parsed_date.is_err() {
        // if date is malformed, ignore it
        println!("Skipping {:?} - Malformed date", file_name);
        return None;
    }

    // Gather info into a struct:
    Some(Post {
        name: title.to_string(),
        url: format!("{}/{}", BASE_URL, goal_name),
        datestring: rawdate.to_string(),
        datecreated: parsed_date.unwrap().timestamp(),
    })
}

fn md_to_html(md: &str, comrak_options: &ComrakOptions) -> std::string::String {
    let arena = Arena::new();
    let root = parse_document(&arena, &md, &comrak_options);

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // parse code snippets outside of the parser to avoid quotes being html escaped.
    let mut code_snippets: Vec<std::string::String> = Vec::new();

    // basic BFS
    let mut nodes_to_parse = vec![root];
    let mut node;
    while nodes_to_parse.len() > 0 {
        node = nodes_to_parse.pop().unwrap();
        match &mut node.data.borrow_mut().value {
            &mut NodeValue::CodeBlock(ref mut codenode) => {
                let lang = std::str::from_utf8(&codenode.info).unwrap();
                let syntax = ps.find_syntax_by_token(lang).unwrap();
                let orig_body = std::mem::replace(
                    // replace code snippets with {{ CODE:# }}
                    &mut codenode.literal,
                    format!("{{ CODE:{} }}", code_snippets.len())
                        .as_bytes()
                        .to_vec(),
                );

                code_snippets.push(
                    // Save the converted code snippet so we can add it back in
                    highlighted_html_for_string(
                        std::str::from_utf8(&orig_body).unwrap(),
                        &ps,
                        &syntax,
                        &ts.themes["base16-ocean.dark"],
                    ),
                );
            }
            _ => (),
        }
        nodes_to_parse.extend(node.children());
    }

    let mut html = vec![];
    format_html(root, comrak_options, &mut html).unwrap(); // generate main html

    let mut html_string = std::str::from_utf8(&html)
        .unwrap()
        .to_string()
        .replace("<p><time>", "<time>")
        .replace("</time></p>", "</time>"); // avoid time getting extra escaped

    // Add the code snippets back in:
    for (i, cs) in code_snippets.iter().enumerate() {
        html_string = html_string.replace(
            &format!("{{ CODE:{} }}", i),
            &cs.lines()
                .skip(1)
                .map(|l| l.to_owned() + "\n")
                .collect::<String>() // reconnect the strings
                .replace("</span></pre>", "</span>"),
        );
    }

    html_string // return final html string
}

fn write_index(post_structs: &mut Vec<Post>, header: &str, comrak_options: &ComrakOptions) {
    // sort the posts: most recent first.
    post_structs.sort_by_key(|x| -x.datecreated);

    println!("Generating index"); // this is done inline since it's pretty simple
    let mut index = header.replace("{{ TITLE }}", "arya-k").to_string(); // reuse header
    index.push_str("<h1 style=\"font-size:3em\">Posts</h1>\n"); // say "Posts" at top of page

    for post in post_structs {
        // generate posts in order
        index.push_str(&format!(
            "<h1><a class=\"mainpage\" href=\"{}\">{}</a></h1>\n{}\n",
            post.url,
            markdown_to_html(&post.name, &comrak_options) // respect markdown in titles
                .replace("<p>", "")
                .replace("</p>", ""),
            post.datestring
        ));
    }

    index.push_str(
        &fs::read_to_string("./compiled/assets/index_footer.html").expect("Unable to read footer"),
    ); // this part is a custom footer

    fs::write("./compiled/index.html", index).expect("Unable to write index file");
}

fn main() {
    // First, remove the compiled assets
    remove_compiled_assets();

    // Gather the head and tail strings:
    let header = fs::read_to_string("./compiled/assets/head.html")
        .expect("Unable to read header")
        .replace("{{ BASE_URL }}", BASE_URL);
    let footer = "</body>\n</html>";

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

    // Start parsing through all the posts:
    let mut post_structs: Vec<Post> = Vec::new();

    let posts = fs::read_dir("./posts").unwrap();
    for post in posts {
        // gather info about file
        let safe_post = post.unwrap();
        let goal_path = safe_post.path().with_extension("html");
        let goal_name = goal_path.file_name().unwrap().to_str().unwrap();
        println!("Compiling {:?}", safe_post.file_name());

        // read markdown
        let md = fs::read_to_string(safe_post.path()).expect("Unable to read file");

        if let Some(p) = validate_post(&md, &safe_post.file_name(), &goal_name) {
            // convert to markdown + generate file:
            let html = format!(
                "{}{}{}",
                header.replace("{{ TITLE }}", &p.name),
                md_to_html(&md, &comrak_options),
                footer
            );

            // write to file
            fs::write(format!("./compiled/{}", goal_name), html).expect("Unable to write file");

            // add to all posts
            post_structs.push(p);
        }
    }

    // create the index file using the posts
    write_index(&mut post_structs, &header, &comrak_options);

    println!("Done!");
}
