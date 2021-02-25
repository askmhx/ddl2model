

pub fn gen_model(lang: &str, in_file: os::File, out_file: os::File) {
    let buffered: BufReader<File> = BufReader::new(in_file);

    for line in buffered.lines().map(|x| x.unwrap()) {
        println!("{}", line);

        if CONST_REGEX_START.is_match(line.as_str()) {
            println!("match title:{}", line);
            let title = &(CONST_REGEX_START.captures(&line).unwrap())["title"];
            out_file.write_all(title.as_bytes());
            out_file.write_all(b"\n");
        }

        if CONST_REGEX_ROW.is_match(line.as_str()) {
            println!("match colum:{}", line);
            //out_file.write_all( titleRegex.captures_iter(&line)[0].at(0).unwrap_or(""))
        }

        if CONST_REGEX_END.is_match(line.as_str()) {
            println!("match end:{}", line);
            let end = CONST_REGEX_END.captures(&line).unwrap().get(0).unwrap().as_str();
            out_file.write_all(end.as_bytes());
            out_file.write_all(b"\n");
        }
    }
}