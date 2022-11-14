use neon::prelude::*;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader, BufWriter, Write };

fn count_words(mut cx: FunctionContext) -> JsResult<JsString> {
    let filepath = cx.argument::<JsString>(0)?.value(&mut cx);

    println!("Counting words in {}", filepath);

    match File::open(filepath) {
        Ok(file) => {
            let mut word_count: HashMap<String, u32> = HashMap::new();
            let mut writer = BufWriter::new(File::create("./output.txt").expect("Unable to create file"));
            let re = regex::Regex::new(r"\w+").unwrap();

            println!("Reading file");

            for line in BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        let lowercase_line = line.to_lowercase();
                        let words = lowercase_line.split_whitespace();

                        for word in words {
                            let capture = re.find(&word);

                            match capture {
                                Some(capture) => {
                                    let word = capture.as_str();

                                    word_count.entry(word.to_string()).and_modify(|c| *c += 1).or_insert(1);
                                },
                                None => continue
                            }
                        }
                    }
                    Err(e) => {
                        return cx.throw_error(format!("Error reading file: {}", e));
                    }
                }
            }

            println!("Writing file");

            let mut word_length = 0;
            let mut count_length = 0;

            word_count.iter().for_each(|(k, v)| {
                word_length = max(word_length, k.len());
                count_length = max(count_length, v.to_string().len());
            });

            println!("Word length: {}", word_length);

            writer.write_all(format!(
                "{:word_length$} | {:count_length$}\n",
                "Word",
                "Count",
                word_length = word_length,
                count_length = count_length
            ).as_bytes()).unwrap();
            writer.write_all("-".repeat(word_length + word_length + 3).as_bytes()).unwrap();
            writer.write_all("\n".as_bytes()).unwrap();

            word_count.iter().for_each(|(k, v)| {
                writer.write_all(format!(
                    "{:word_length$} | {:count_length$}\n",
                    k,
                    v,
                    word_length = word_length,
                    count_length = count_length
                ).as_bytes()).unwrap();
            });

            writer.flush().unwrap();

            Ok(cx.string(""))
        },
        Err(e) => cx.throw_error(e.to_string()),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("countWords", count_words)?;
    Ok(())
}
