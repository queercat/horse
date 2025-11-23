use std::{collections::HashMap, env};

use mlua::Lua;
use regex::{Captures, Regex};

pub trait Render {
    fn render(&mut self, environment: Option<&HashMap<String, String>>) -> Result<String, String>;
}

impl Render for String {
    fn render(&mut self, environment: Option<&HashMap<String, String>>) -> Result<String, String> {
        render(self.to_owned(), environment)
    }
}

struct Block {
    /// The lua expression inside the source match.
    expression: String,
    /// Stuff between the two matches.
    data: String,
    /// This is the first character index of the block.
    start: usize,
    /// This is the last character index of the block.
    end: usize,
}

pub fn render(html: String, environment: Option<&HashMap<String, String>>) -> Result<String, String> {
    let source_regex = Regex::new(r"<!--%(.+)-->").unwrap();
    let captures: Vec<Captures> = source_regex.captures_iter(&html).collect();

    Ok(process(html.clone(), captures, environment)?)
}

fn process(source: String, captures: Vec<Captures>, enviornment: Option<&HashMap<String, String>>) -> Result<String, String> {
    let mut text = source;

    let mut blocks = Vec::<Block>::new();
    let lua = Lua::new();

    for _ in 0..captures.len() / 2 {
        let (_, [expression]) = captures[0].extract();
        let start = captures[0].get_match().start();
        let end = captures[1].get_match().end();

        let data_beginning = captures[0].get_match().end();
        let data_end = captures[1].get_match().start();

        blocks.push(Block {
            expression: expression.to_string(),
            start,
            end,
            data: text[data_beginning..data_end].to_string(),
        });
    }

    if let Some(ref e) = enviornment {
        for (key, value) in e.into_iter() {
            lua.globals().set(key.to_string(), value.to_string()).unwrap();
        }
    }

    for block in blocks {
        lua.globals().set("data", block.data).unwrap();
        lua.load(block.expression).exec().unwrap();

        let data: String = lua.globals().get("data").unwrap();

        let mut left = text[0..block.start].to_string().to_owned();
        let right = &text[block.end..text.len()];

        left.push_str(&data);
        left.push_str(right);

        text = left;
    }

    Ok(text)
}
