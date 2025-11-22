pub mod auth;
pub mod jsonrpc;

use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;

//Implementing 'Request' and 'Either' from
//https://github.com/dovahcrow/deribit-rs/blob/master/src/models/mod.rs
pub trait Request {
    const METHOD: &'static str;
    const HAS_PAYLOAD: bool = true;
    type Response;

    fn no_payload(&self) -> bool {
        !Self::HAS_PAYLOAD
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn map_left<F, U>(self, f: F) -> Either<U, R>
    where
        F: FnOnce(L) -> U,
    {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    pub fn map_right<F, U>(self, f: F) -> Either<L, U>
    where
        F: FnOnce(R) -> U,
    {
        match self {
            Either::Right(r) => Either::Right(f(r)),
            Either::Left(l) => Either::Left(l),
        }
    }
    pub fn left_result(self) -> StdResult<L, R> {
        match self {
            Either::Left(l) => Ok(l),
            Either::Right(r) => Err(r),
        }
    }
    pub fn right_result(self) -> StdResult<R, L> {
        match self {
            Either::Left(l) => Err(l),
            Either::Right(r) => Ok(r),
        }
    }
}

impl<T> Either<T, T> {
    pub fn unwrap(self) -> T {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => r,
        }
    }
}

impl<L, R> Either<L, R> {
    pub fn unwrap_left(self) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => panic!("Either is right"),
        }
    }

    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub fn unwrap_right(self) -> R {
        match self {
            Either::Left(_) => panic!("Either is left"),
            Either::Right(r) => r,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }
}
