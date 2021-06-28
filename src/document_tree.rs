use std::collections::VecDeque;

use crate::document_tree::DocumentTree::ContainerNode;
use crate::error;
use crate::error::ParseError;
use crate::tag::{ContainerTag, ValueTag};
use crate::tokens::Token;

pub enum DocumentTree {
    ContainerNode(ContainerTag, Vec<DocumentTree>),
    ValueNode(ValueTag, String),
    TextNode(String),
    Empty,
}

pub fn parse_doc(tokens: &mut VecDeque<Token>) -> error::Result<DocumentTree> {
    Ok(if let Some(token) = tokens.pop_front() {
        match token {
            Token::TagOpen(tag) => {
                let mut parts = Vec::new();

                while let Some(next_token) = tokens.front() {
                    if next_token == &Token::TagClose(tag) {
                        tokens.pop_front();

                        return Ok(ContainerNode(tag, parts));
                    } else if let Token::TagClose(_) = next_token {
                        return Ok(ContainerNode(tag, parts));
                    } else {
                        parts.push(parse_doc(tokens)?);
                    }
                }

                return Err(ParseError::UnexpectedEndOfInput(tag));
            }
            Token::TagClose(tag) => return Err(ParseError::UnexpectedCloseTag(tag)),
            Token::TagValue(tag, value) => DocumentTree::ValueNode(tag, value),
            Token::TextData(text) => DocumentTree::TextNode(text),
        }
    } else {
        DocumentTree::Empty
    })
}
