/* Description: Query n-gram indices to seed a regex search.

Copyright (C) 2024 Danny McClanahan <dmcC2@hypnicjerk.ai>
SPDX-License-Identifier: AGPL-3.0-or-later

This file is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation; either version 3 of the
License, or (at your option) any later version.

This file is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

#![warn(rustdoc::missing_crate_level_docs)]
/* #![warn(missing_docs)] */
#![doc(test(attr(deny(warnings))))]

//! Query n-gram indices to seed a regex search.

use std::fmt;

use displaydoc::Display;
use regex_syntax::{self, hir::Hir, ParserBuilder};
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum QueryError {
  /// regex pattern parse error
  Pattern(#[from] regex_syntax::Error),
}

#[derive(Debug, Clone, Default)]
pub struct QueryConfig {
  parser_builder: ParserBuilder,
}

impl QueryConfig {
  pub fn build(self) -> QueryParser {
    let Self { parser_builder } = self;
    let parser = parser_builder.build();
    QueryParser { parser }
  }
}

#[derive(Debug, Clone)]
pub struct QueryParser {
  parser: regex_syntax::Parser,
}

impl Default for QueryParser {
  fn default() -> Self { QueryConfig::default().build() }
}

impl QueryParser {
  pub fn parse(&mut self, pattern: &str) -> Result<Query, QueryError> {
    let hir = self.parser.parse(pattern)?;
    Ok(Query { hir })
  }
}

#[derive(Debug, Clone)]
pub struct Query {
  hir: Hir,
}

impl fmt::Display for Query {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Query(/{}/)", &self.hir) }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn default_parse_display() {
    let query = QueryParser::default().parse("asdf").unwrap();
    assert_eq!(format!("{}", query), "Query(/(?:asdf)/)");
  }

  #[test]
  fn not_just_literal_display() {
    let query = QueryParser::default().parse("(as|df)").unwrap();
    assert_eq!(format!("{}", query), "Query(/((?:(?:as)|(?:df)))/)");
  }

  #[test]
  fn failed_parse() {
    match QueryParser::default().parse("aa(") {
      Err(QueryError::Pattern(regex_syntax::Error::Parse(e))) => {
        assert!(matches!(
          e.kind(),
          regex_syntax::ast::ErrorKind::GroupUnclosed
        ));
      },
      _ => unreachable!(),
    }
  }
}
