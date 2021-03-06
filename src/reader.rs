use anyhow::{anyhow, bail, Context, Result};
use lazy_static::lazy_static;
use regex::{Regex};
use crate::types::{MalList, MalType};
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::MalType::List;

type Reader = Peekable<IntoIter<Token>>;
type Token = String;
lazy_static! {
        static ref RE: Regex = Regex::new(
        r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        ).unwrap();}

fn add_parens(result: & mut Vec<Token>){
    result.push(")".to_string());
    result.insert(0, "(".to_string());
}
fn tokenize(text: &str) -> Vec<Token>{
    let mut result: Vec<Token> = Vec::new();
    for cap in RE.captures_iter(text){
        result.push(cap[1].to_string())
    }
    add_parens(& mut result);
    result
}

pub fn read_str(text: &str) -> Result<MalType>{
    let mut r = tokenize(text).into_iter().peekable();
    let result = read_list(& mut r)?;
    if r.peek().is_some(){
        bail!("to many closing parenthesis ")
    }
    Ok(result)


}
fn read_list(r: & mut Reader) -> Result<MalType> {
    let mut v: MalList = vec![];
    r.next();
    loop {
        let x = r.peek().ok_or(anyhow!("no closing paren"))?;
        match x.as_str() {
            ")" => {r.next();
                break}
            _ => v.push(read_form(r)?)
        }
    }
    Ok(List(v))
}
fn read_form(r: & mut Reader) -> Result<MalType>{
    match r.peek().unwrap().as_str() {
        "(" => read_list(r),
        _ => read_atom(r),
    }
}
fn read_atom(r: & mut Reader) -> Result<MalType> {
    let x = r.next().expect("should always be valid");
    if x.starts_with('"'){
        if x.len() == 1{
            bail!("invalid string {}", x)
        } else if !x.ends_with('"'){
            bail!("no closing quote {}", x)
        } else {
            Ok(MalType::Str(x))
        }
    } else if x.chars().nth(0).unwrap().is_numeric() {
        Ok(MalType::Num(x.parse().map_err(|_|anyhow!("failed to parse {} as num", x))?))
    } else if x == "true" {
        Ok(MalType::Bool(true))
    } else if x == "false" {
        Ok(MalType::Bool(false))
    } else {
        Ok(MalType::Symbol(x))
    }

}
