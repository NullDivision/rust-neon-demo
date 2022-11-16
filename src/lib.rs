use neon::prelude::*;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader, BufWriter, Error, Write };

fn build_word_counter(buf: BufReader<File>) -> Result<HashMap<String, u32>, Error> {
    println!("Reading file");

    let mut word_count: HashMap<String, u32> = HashMap::new();
    let re = regex::Regex::new(r"\w+").expect("Failed to compile regex");

    for line in buf.lines() {
        let lowercase_line = line?.to_lowercase();
        let words = lowercase_line.split_whitespace();

        for word in words {
            if let Some(word_match) = re.find(&word) {
                word_count.entry(word_match.as_str().to_string()).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }

    Ok(word_count)
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

    write!(&mut writer, "Word{:word_length$} | Count{:count_length$}\n", "", "")?;
    writer.write_all("-".repeat(word_length + count_length + 3).as_bytes())?;
    writer.write_all("\n".as_bytes())?;

    for (word, count) in &word_count_vec {
        write!(&mut writer, "{word:word_length$} | {count:count_length$}\n")?;
    }

    writer.flush()?;

    Ok(())
}

fn count_words_impl(filepath: &str) -> Result<(), Error> {
    let word_counter = build_word_counter(BufReader::new(File::open(filepath)?))?;
    let word_count_vec = to_sorted_vec(&word_counter);

    println!("Writing file");

    write_to_file(word_count_vec)?;

    Ok(())
}

fn count_words(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let filepath = cx.argument::<JsString>(0)?.value(&mut cx);

    println!("Counting words in {}", filepath);

    match count_words_impl(&filepath) {
        Ok(_) => Ok(cx.undefined()),
        Err(e) => cx.throw_error(e.to_string())
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("countWords", count_words)?;
    Ok(())
}
