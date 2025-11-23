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

pub fn render(template: String, environment: Option<&HashMap<String, String>>) -> Result<String, String> {
    let captures = get_captures(&template);

    match captures {
        None => return Ok(template),
        Some(captures) => {
            let source = evaluate(&template, captures, environment)?;
            return Ok(render(source, environment)?)
        }
    }
}

fn get_captures(source: &String) -> std::option::Option<regex::Captures<'_>> {
    let source_regex = Regex::new(r"(?<start_token><!--%)|(?<end_token>%-->)").unwrap();
    source_regex.captures(&source)
}

fn evaluate(source: &String, captures: Captures<'_>, environment: Option<&HashMap<String, String>>) -> Result<String, String> {
    let mut text = source;

    let lua = Lua::new();

    if let Some(e) = environment {
        for (key, value) in e.into_iter() {
            lua.globals().set(key.to_string(), value.to_string()).unwrap();
        }
    }

    lua.globals().set("data", block.data).unwrap();
    lua.load(block.expression).exec().unwrap();

    let data: String = lua.globals().get("data").unwrap();

    let mut left = text[0..block.start].to_string().to_owned();
    let right = &text[block.end..text.len()];

    left.push_str(&data);
    left.push_str(right);

    text = left;

    Ok(text)
}
