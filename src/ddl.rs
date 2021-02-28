use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

pub fn gen_model(lang: &str, in_file: File, mut out_file: File) {

    let lang_go: &str = "GO";
    let lang_java: &str = "JAVA";
    let lang_rust: &str = "RUST";

    let mut template_start: HashMap<&str, &str> = HashMap::new();
    let mut template_end: HashMap<&str, &str> = HashMap::new();
    let mut template_row: HashMap<&str, &str> = HashMap::new();

    let regex_start: Regex = Regex::new(r"CREATE\s+TABLE\s+\S+.(?P<title>\S+$)").unwrap();
    let regex_end: Regex = Regex::new(r"\)\s+CHARSET\s+=\s+UTF8MB4(\s+)?;(\s+)?").unwrap();
    let regex_row: Regex = Regex::new(r"^\S{4}(\S+)\s+(\S+),?$").unwrap();


    template_start.insert(lang_go, "struct %s{");
    template_start.insert(lang_java, "public class %s{");
    template_start.insert(lang_rust, "pub struct %s {");

    template_row.insert(lang_go, "%s %s");
    template_row.insert(lang_java, "%s %s;");
    template_row.insert(lang_rust, "%s:%s,");


    template_end.insert(lang_go, "}");
    template_end.insert(lang_java, "}");
    template_end.insert(lang_rust, "}");


    let buffered: BufReader<File> = BufReader::new(in_file);

    for line in buffered.lines().map(|x| x.unwrap()) {
        // println!("{}", line);

        if regex_start.is_match(line.as_str()) {
            println!("match title:{}", line);
            let title = &(regex_start.captures(&line).unwrap())["title"];

            // template_start[lang]

            out_file.write_all(title.as_bytes());
            out_file.write_all(b"\n");
        }

        if regex_row.is_match(line.as_str()) {
            println!("match colum:{}", line);
            //out_file.write_all( titleRegex.captures_iter(&line)[0].at(0).unwrap_or(""))
        }

        if regex_end.is_match(line.as_str()) {
            println!("match end:{}", line);
            let end = regex_end.captures(&line).unwrap().get(0).unwrap().as_str();
            out_file.write_all(end.as_bytes());
            out_file.write_all(b"\n");
        }
    }
}