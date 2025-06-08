use crate::models::entry::Entry;
use crate::models::key_value::KeyValueList;
use crate::models::preamble::Preamble;
use crate::models::value::Value;
use crate::models::{Comment, document::Document, Element, StringDef};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use dusty_errors::{DustyResult, DustyError};
use crate::errors::ErrorStack;

#[derive(Parser)]
#[grammar = "bibtex.pest"]
pub struct BibtexParser;

pub fn parse(input: &str) -> DustyResult<Document> {
    let pairs = match BibtexParser::parse(Rule::document, input) {
        Ok(pairs) => pairs,
        Err(e) => return Err(DustyError::BibtexParsingError(e.to_string())),
    };

    let res = match handle_document(pairs) {
        Ok(res) => res,
        Err(e) => {
            return Err(DustyError::BuildError(e.to_string()))
        },
    };
    
    res.valid()?;

    Ok(res)
}

fn handle_document(pairs: Pairs<Rule>) -> Result<Document, ErrorStack> {
    let mut elements: Vec<Element> = vec![];
    for pair in pairs {
        // pair := document
        for inner in pair.into_inner() {
            // inner := preamble_def | string_def | comment_def | entry_def
            let e_inner = ErrorStack::new(&inner);
            elements.push(match inner.as_rule() {
                Rule::preamble_def => {
                    match handle_preamble(inner) {
                        Ok(preamble) => Element::Preamble(preamble),
                        Err(mut e) => {
                            e.push_from_error(&e_inner);
                            return Err(e);
                        }
                    }
                }
                Rule::string_def => {
                    match handle_string(inner) {
                        Ok(string_def) => Element::StringDef(string_def),
                        Err(mut e) => {
                            e.push_from_error(&e_inner);
                            return Err(e);
                        }
                    }
                }
                Rule::comment_def => {
                    match handle_comment(inner) {
                        Ok(comment) => Element::Comment(comment),
                        Err(mut e) => {
                            e.push_from_error(&e_inner);
                            return Err(e);
                        }
                    }
                }
                Rule::entry_def => {
                    match handle_entry(inner) {
                        Ok(entry) => Element::Entry(entry),
                        Err(mut e) => {
                            e.push_from_error(&e_inner);
                            return Err(e);
                        }
                    }
                }
                _ => {
                    continue;
                }
            })
        }
    }
    Ok(Document(elements))
}

fn handle_preamble(pair: Pair<Rule>) -> Result<Preamble, ErrorStack> {
    // pair := preamble_def
    let e_pair = ErrorStack::new(&pair);
    for inner in pair.into_inner() {
        // inner := preamble_keyword | decorated_value
        let e_inner = ErrorStack::new(&inner);
        match inner.as_rule() {
            Rule::decorated_value => {
                return match handle_decorated_value(inner) {
                    Ok(content) => Ok(Preamble::new(content)),
                    Err(mut e) => {
                        e.push_from_error(&e_inner);
                        Err(e)
                    },
                };
            }
            _ => {}
        }
    }
    Err(e_pair)
}

fn handle_string(pair: Pair<Rule>) -> Result<StringDef, ErrorStack> {
    // pair := string_def
    let e_pair = ErrorStack::new(&pair);
    for inner in pair.into_inner() {
        // inner := string_keyword | key_value_list
        let e_inner = ErrorStack::new(&inner);
        match inner.as_rule() {
            Rule::key_value_list => {
                let list = match handle_key_value_list(inner) {
                    Ok(list) => list,
                    Err(mut e) => {
                        e.push_from_error(&e_inner);
                        return Err(e)
                    },
                };
                return Ok(StringDef::new(list));
            }
            _ => {}
        }
    }
    Err(e_pair)
}

fn handle_comment(pair: Pair<Rule>) -> Result<Comment, ErrorStack> {
    // pair := comment_def
    let e_pair= ErrorStack::new(&pair);
    for inner in pair.into_inner() {
        // inner := comment_keyword | decorated_value
        let e_inner = ErrorStack::new(&inner);
        match inner.as_rule() {
            Rule::decorated_value => {
                let value = match handle_decorated_value(inner) {
                    Ok(value) => value,
                    Err(mut e) => {
                        e.push_from_error(&e_inner);
                        return Err(e)
                    },
                };
                return Ok(Comment::new(value));
            }
            _ => {}
        }
    }
    Err(e_pair)
}

fn handle_entry(pair: Pair<Rule>) -> Result<Entry, ErrorStack> {
    // pair := entry_def
    let e_pair = ErrorStack::new(&pair);
    let mut inner = pair.into_inner();
    let model = match inner.next() {
        // inner := entry_keyword
        Some(k) => {
            let e_inner = ErrorStack::new(&k);
            match k.as_rule() {
                Rule::entry_keyword => match handle_entry_keyword(k) {
                    Ok(entry) => entry,
                    Err(mut e) => {
                        e.push_from_error(&e_inner);
                        return Err(e)
                    }
                },
                _ => return Err(e_inner),
            }
        }
        _ => return Err(e_pair),
    };

    let id = match inner.next() {
        Some(i) => {
            let e_inner = ErrorStack::new(&i);
            match i.as_rule() {
                Rule::entry_id => match handle_entry_id(i) {
                    Ok(id) => id,
                    Err(mut e) => {
                        e.push_from_error(&e_inner);
                        return Err(e);
                    }
                },
                _ => return Err(e_inner),
            }
        }
        _ => return Err(e_pair),
    };

    let list = match inner.next() {
        Some(l) => {
            let e_inner = ErrorStack::new(&l);
            match l.as_rule() {
                Rule::key_value_list => match handle_key_value_list(l) {
                    Ok(list) => list,
                    Err(mut e) => {
                        e.push_from_error(&e_inner);
                        return Err(e)
                    }
                },
                _ => return Err(e_inner),
            }
        }
        _ => return Err(e_pair),
    };

    Ok(Entry::new(model, id, list))
}

fn handle_entry_keyword(pair: Pair<Rule>) -> Result<String, ErrorStack> {
    // pair := entry_keyword
    let e_pair = ErrorStack::new(&pair);
    match pair.into_inner().next() {
        // inner := entry_keyword_inner
        Some(entry_keyword_inner) => Ok(entry_keyword_inner.as_str().to_string()),
        _ => Err(e_pair),
    }
}

fn handle_entry_id(pair: Pair<Rule>) -> Result<String, ErrorStack> {
    // pair := entry_id
    let e_pair = ErrorStack::new(&pair);
    let mut inner = pair.into_inner();
    match inner.next() {
        // inner := key
        Some(txt) => Ok(txt.as_str().to_string()),
        _ => Err(e_pair),
    }
}

fn handle_key_value_list(pair: Pair<Rule>) -> Result<KeyValueList, ErrorStack> {
    // pair := key_value_list
    let e_pair = ErrorStack::new(&pair);
    let mut key_value = KeyValueList::new();
    for inner in pair.into_inner() {
        // inner := key_value_pair
        let e_inner = ErrorStack::new(&inner);
        match inner.as_rule() {
            Rule::key_value_pair => {
                match handle_key_value_pair(inner) {
                    Ok((key, value)) => {
                        key_value.insert(key, value);
                    }
                    Err(mut e) => {
                        e.push_from_error(&e_inner);
                        return Err(e);
                    }
                };
            }
            _ => return Err(e_pair),
        }
    }

    Ok(key_value)
}

fn handle_key_value_pair(pair: Pair<Rule>) -> Result<(String, Value), ErrorStack> {
    // pair := key_value_pair
    let e_pair = ErrorStack::new(&pair);
    let mut inner = pair.into_inner();
    let key = match inner.next() {
        // key := key
        Some(key) => key.as_str().to_string(),
        None => return Err(e_pair),
    };

    let value = match inner.next() {
        // value := decorated_value
        Some(value) => {
            let e_value = ErrorStack::new(&value);
            match handle_decorated_value(value) {
                Ok(val) => val,
                Err(mut e) => {
                    e.push_from_error(&e_value);
                    return Err(e)
                },
            }
        }
        None => return Err(e_pair),
    };

    Ok((key, value))
}

fn handle_decorated_value(pair: Pair<Rule>) -> Result<Value, ErrorStack> {
    // pair := decorated_value
    let e_pair = ErrorStack::new(&pair);
    let inner = match pair.into_inner().next() {
        // inner := wrapped_value_braked | concatenated_value
        Some(inner) => inner,
        None => return Err(e_pair),
    };
    let e_inner = ErrorStack::new(&inner);
    match inner.as_rule() {
        Rule::wrapped_value_braked => {
            let txt = match inner.into_inner().next() {
                // inner := wrapped_value_braked_inner
                Some(inner) => inner.as_str().to_string(),
                _ => return Err(e_inner),
            };
            Ok(Value::String(txt))
        }
        Rule::concatenated_value => match handle_concatenated_value(inner) {
            Ok(val) => Ok(val),
            Err(mut e) => {
                e.push_from_error(&e_inner);
                Err(e)
            },
        },
        _ => Err(e_inner),
    }
}

fn handle_concatenated_value(pair: Pair<Rule>) -> Result<Value, ErrorStack> {
    // pair := concatenated_value
    let e_pair = ErrorStack::new(&pair);
    let mut content = vec![];
    for inner in pair.into_inner() {
        let next = match inner.as_rule() {
            Rule::wrapped_value_quotes => {
                // inner := wrapped_value_quotes
                let txt = inner.into_inner()
                    // -> wrapped_value_quotes_inner
                    .next().unwrap()
                    .as_str().to_string();
                Value::String(txt)
            }
            Rule::key => Value::Key(inner.as_str().to_string()),
            _ => {
                return Err(ErrorStack::new(&inner));
            }
        };
        content.push(next);
    }

    if content.is_empty() {
        return Err(e_pair);
    }

    if content.len() == 1 {
        return match &content[0] {
            Value::String(s) => Ok(Value::String(s.to_string())),
            Value::Key(k) => Ok(Value::Key(k.clone())),
            _ => Err(e_pair),
        };
    }
    Ok(Value::Mixed(content))
}
