use std::io::BufReader;
use std::{fs::File, io::Write};

use regex::Regex;
use xml::reader::{EventReader, XmlEvent};

pub fn run() {
    let max_thread = 128;
    let handlers = (0..max_thread)
        .map(|id| std::thread::spawn(move || run_analysis(max_thread, id).unwrap()))
        .collect::<Vec<_>>();
    handlers.into_iter().for_each(|h| {
        h.join().unwrap();
    });
}

fn run_analysis(max_thread: usize, id: usize) -> std::io::Result<()> {
    let html_re = Regex::new(r"<[^>]*>").unwrap();
    let template_re = Regex::new(r"\{\{[^}]*\}\}").unwrap();
    let link_re = Regex::new(r"\[\[([^|\]]+)(?:\|[^\]]*)?\]\]").unwrap();
    let quote_re = Regex::new(r"'''[^']*'''").unwrap();
    let skip_re = Regex::new(r"^[\d\W[:alpha:]]").unwrap();
    let url_re = Regex::new(r"https?://[^\s]+|www\.[^\s]+").unwrap();

    let remove_re = Regex::new(r"[\[\]{}()\/\.\:、。「」『』（）0-9_\-&a-zA-Z|=\W]").unwrap();

    let mut num_bytes = 0;
    let mut num_mega_bytes = 0;

    let mut out = File::create(format!("./data/text/raw_{id}.txt"))?;

    for i in 0..10000000 {
        let filename = format!("./data/source/page_{}.xml", i);
        println!("{id}: processing {filename}", id = id, filename = filename);
        if i % max_thread != id {
            continue;
        }
        if let Ok(file) = File::open(filename) {
            let file = BufReader::new(file);

            let parser = EventReader::new(file);
            let mut inside_text_tag = false;
            let mut text = String::new();

            for event in parser {
                match event {
                    Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "text" => {
                        inside_text_tag = true;
                    }
                    Ok(XmlEvent::EndElement { name }) if name.local_name == "text" => {
                        inside_text_tag = false;
                    }
                    Ok(XmlEvent::Characters(data)) if inside_text_tag => {
                        text.push_str(&data);
                    }
                    _ => {}
                }
            }
            if text.is_empty() {
                continue;
            }

            let text = html_re.replace_all(&text, "");
            let text = template_re.replace_all(&text, "");
            let text = link_re.replace_all(&text, "$1");
            let text = quote_re.replace_all(&text, "");
            let text = url_re.replace_all(&text, "");

            for line in text
                .lines()
                .map(str::trim)
                .filter(|line| line.chars().count() > 500 && !skip_re.is_match(line))
            {
                let line = remove_re.replace_all(line, "");
                writeln!(out, "{line}")?;
                num_bytes += line.len();
                if num_bytes / (1024 * 1024 * 1024) > num_mega_bytes {
                    num_mega_bytes = num_bytes / (1024 * 1024 * 1024);
                    println!("{id}: {} GB processed", num_mega_bytes);
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}
