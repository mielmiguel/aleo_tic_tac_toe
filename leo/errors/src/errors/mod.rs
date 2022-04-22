// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

/// Contains the ASG error definitions.
use crate::LeoMessageCode;

/// Contains the AST error definitions.
pub mod ast;
pub use self::ast::*;

/// Contains the AST error definitions.
pub mod cli;
pub use self::cli::*;

/// Contains the AST error definitions.
pub mod compiler;
pub use self::compiler::*;

/// Contains the Input error definitions.
pub mod input;
pub use self::input::*;

/// Contains the Package error definitions.
pub mod package;
pub use self::package::*;

/// Contains the Parser error definitions.
pub mod parser;
pub use self::parser::*;

/// The LeoError type that contains all sub error types.
/// This allows a unified error type throughout the Leo crates.
#[derive(Debug, Error)]
pub enum LeoError {
    /// Represents an AST Error in a Leo Error.
    #[error(transparent)]
    AstError(#[from] AstError),
    /// Represents an CLI Error in a Leo Error.
    #[error(transparent)]
    CliError(#[from] CliError),
    /// Represents an Compiler Error in a Leo Error.
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
    /// Represents an Input Error in a Leo Error.
    #[error(transparent)]
    InputError(#[from] InputError),
    /// Represents an Package Error in a Leo Error.
    #[error(transparent)]
    PackageError(#[from] PackageError),
    /// Represents an Parser Error in a Leo Error.
    #[error(transparent)]
    ParserError(#[from] ParserError),
}

impl LeoError {
    /// Implement error code for each type of Error.
    pub fn error_code(&self) -> String {
        use LeoError::*;

        match self {
            AstError(error) => error.error_code(),
            CompilerError(error) => error.error_code(),
            CliError(error) => error.error_code(),
            InputError(error) => error.error_code(),
            ParserError(error) => error.error_code(),
            PackageError(error) => error.error_code(),
        }
    }

    /// Implement exit code for each type of Error.
    pub fn exit_code(&self) -> i32 {
        use LeoError::*;

        match self {
            AstError(error) => error.exit_code(),
            CompilerError(error) => error.exit_code(),
            CliError(error) => error.exit_code(),
            InputError(error) => error.exit_code(),
            ParserError(error) => error.exit_code(),
            PackageError(error) => error.exit_code(),
        }
    }
}

/// A global result type for all Leo crates, that defaults the errors to be a LeoError.
pub type Result<T, E = LeoError> = core::result::Result<T, E>;