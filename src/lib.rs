// No lambda, aws or any other specifics, just common functionality here

#![feature(async_await)]
// DotEnv
#[macro_use]
extern crate dotenv_codegen;

pub mod aws;
pub mod query;