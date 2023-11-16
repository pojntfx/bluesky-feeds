// Code generated by scale-signature 0.4.5, DO NOT EDIT.
// output: local_classifier_latest_guest

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::io::Cursor;
use polyglot_rs::{DecodingError, Encoder, Decoder, Kind};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::collections::HashMap;
use regex::Regex;
pub trait Encode {
    fn encode<'a>(
        a: Option<&Self>,
        b: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
trait EncodeSelf {
    fn encode_self<'a, 'b>(
        &'b self,
        b: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>>;
}
pub trait Decode {
    fn decode(
        b: &mut Cursor<&mut Vec<u8>>,
    ) -> Result<Option<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    pub post: Option<Post>,
    pub weight: i64,
}
impl Context {
    pub fn new() -> Self {
        Self {
            post: Some(Post::new()),
            weight: 0,
        }
    }
}
impl Encode for Context {
    fn encode<'a>(
        a: Option<&Context>,
        e: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        a.encode_self(e)
    }
}
impl EncodeSelf for Context {
    fn encode_self<'a, 'b>(
        &'b self,
        e: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        self.post.encode_self(e)?;
        e.encode_i64(self.weight)?;
        Ok(e)
    }
}
impl EncodeSelf for Option<&Context> {
    fn encode_self<'a, 'b>(
        &'b self,
        e: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        if let Some(x) = self {
            x.encode_self(e)?;
        } else {
            e.encode_none()?;
        }
        Ok(e)
    }
}
impl EncodeSelf for Option<Context> {
    fn encode_self<'a, 'b>(
        &'b self,
        e: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        if let Some(x) = self {
            x.encode_self(e)?;
        } else {
            e.encode_none()?;
        }
        Ok(e)
    }
}
impl Decode for Context {
    fn decode(
        d: &mut Cursor<&mut Vec<u8>>,
    ) -> Result<Option<Context>, Box<dyn std::error::Error>> {
        if d.decode_none() {
            return Ok(None);
        }
        if let Ok(error) = d.decode_error() {
            return Err(error);
        }
        let mut x = Context::new();
        x.post = Post::decode(d)?;
        x.weight = d.decode_i64()?;
        Ok(Some(x))
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub did: String,
    pub rkey: String,
    pub text: String,
    pub langs: Vec<String>,
    pub created_at: i64,
    pub likes: i64,
    pub reply: bool,
}
impl Post {
    pub fn new() -> Self {
        Self {
            did: "".to_string(),
            rkey: "".to_string(),
            text: "".to_string(),
            langs: Vec::with_capacity(0),
            created_at: 0,
            likes: 0,
            reply: false,
        }
    }
}
impl Encode for Post {
    fn encode<'a>(
        a: Option<&Post>,
        e: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        a.encode_self(e)
    }
}
impl EncodeSelf for Post {
    fn encode_self<'a, 'b>(
        &'b self,
        e: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        e.encode_string(&self.did)?;
        e.encode_string(&self.rkey)?;
        e.encode_string(&self.text)?;
        e.encode_array(self.langs.len(), Kind::String)?;
        for a in &self.langs {
            e.encode_string(&a)?;
        }
        e.encode_i64(self.created_at)?;
        e.encode_i64(self.likes)?;
        e.encode_bool(self.reply)?;
        Ok(e)
    }
}
impl EncodeSelf for Option<&Post> {
    fn encode_self<'a, 'b>(
        &'b self,
        e: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        if let Some(x) = self {
            x.encode_self(e)?;
        } else {
            e.encode_none()?;
        }
        Ok(e)
    }
}
impl EncodeSelf for Option<Post> {
    fn encode_self<'a, 'b>(
        &'b self,
        e: &'a mut Cursor<Vec<u8>>,
    ) -> Result<&'a mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        if let Some(x) = self {
            x.encode_self(e)?;
        } else {
            e.encode_none()?;
        }
        Ok(e)
    }
}
impl Decode for Post {
    fn decode(
        d: &mut Cursor<&mut Vec<u8>>,
    ) -> Result<Option<Post>, Box<dyn std::error::Error>> {
        if d.decode_none() {
            return Ok(None);
        }
        if let Ok(error) = d.decode_error() {
            return Err(error);
        }
        let mut x = Post::new();
        x.did = d.decode_string()?;
        x.rkey = d.decode_string()?;
        x.text = d.decode_string()?;
        let size_langs = d.decode_array(Kind::String)?;
        for _ in 0..size_langs {
            x.langs.push(d.decode_string()?);
        }
        x.created_at = d.decode_i64()?;
        x.likes = d.decode_i64()?;
        x.reply = d.decode_bool()?;
        Ok(Some(x))
    }
}
