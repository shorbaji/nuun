//! # Dal
//!
//! An interpreter for the Dal languagea
//!
//! ## Overview
//!
//! Dal is a simple Lisp-like language. This module defines an interpreter for the Dal language.
//! The main components of the interpreter are:
//! - Machine: the interpreter itself
//! - Object: the data types of the language
//!

mod error;
mod lexer;
mod object;
mod parser;
mod machine;

