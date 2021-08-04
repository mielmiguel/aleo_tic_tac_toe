// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::Span;

use backtrace::Backtrace;

pub trait ErrorCode: Sized {
    fn exit_code(&self) -> u32;

    fn exit_code_mask() -> u32;

    fn error_type() -> String;

    fn new_from_backtrace<S>(message: S, help: Option<String>, exit_code: u32, backtrace: Backtrace) -> Self
    where
        S: ToString;

    fn new_from_span<S>(message: S, help: Option<String>, exit_code: u32, span: &Span) -> Self
    where
        S: ToString;
}

pub trait LeoErrorCode: ErrorCode {
    #[inline(always)]
    fn code_identifier() -> u32 {
        37
    }
}
