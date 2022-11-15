use neon::prelude::*;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader, BufWriter, Error, Write };

fn build_word_counter(buf: BufReader<File>) -> HashMap<String, u32> {
    println!("Reading file");

    let mut word_count: HashMap<String, u32> = HashMap::new();
    let re = regex::Regex::new(r"\w+").unwrap();

    for line in buf.lines() {
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
            Err(_) => continue
        }
    }

    word_count
}

fn to_sorted_vec<'a>(word_count: &'a HashMap<String, u32>) -> Vec<(&'a String, &'a u32)> {
    let mut word_count_vec: Vec<(&String, &u32)> = word_count.iter().collect();
    word_count_vec.sort_by(|a, b| a.1.cmp(&b.1));

    word_count_vec
}

fn write_to_file(word_count_vec: Vec<(&String, &u32)>) -> Result<(), Error> {
    let mut writer = BufWriter::new(File::create("./output.txt").expect("Unable to create file"));
    let mut word_length = 0;
    let mut count_length = 0;

    for (word, count) in &word_count_vec {
        word_length = max(word_length, word.len());
        count_length = max(count_length, count.to_string().len());
    }

    write!(
        &mut writer,
        "{:word_length$} | {:count_length$}\n",
        "Word",
        "Count",
        word_length = word_length,
        count_length = count_length
    ).expect("Unable to write to file");
    writer.write_all("-".repeat(word_length + word_length + 3).as_bytes()).unwrap();
    writer.write_all("\n".as_bytes()).unwrap();

    for (word, count) in &word_count_vec {
        write!(
            &mut writer,
            "{:word_length$} | {:count_length$}\n",
            word,
            count,
            word_length = word_length,
            count_length = count_length
        ).expect("Unable to write to file");
    }

    writer.flush().unwrap();

    Ok(())
}

fn count_words(mut cx: FunctionContext) -> JsResult<JsString> {
    let filepath = cx.argument::<JsString>(0)?.value(&mut cx);

    println!("Counting words in {}", filepath);

    match File::open(filepath) {
        Ok(file) => {
            let word_count = build_word_counter(BufReader::new(file));
            let word_count_vec = to_sorted_vec(&word_count);

            println!("Writing file");

            match write_to_file(word_count_vec) {
                Ok(_) => Ok(cx.string("Done")),
                Err(_) => cx.throw_error("Error writing file")
            }
        },
        Err(e) => cx.throw_error(e.to_string()),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("countWords", count_words)?;
    Ok(())
}
