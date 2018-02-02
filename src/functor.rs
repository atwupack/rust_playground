use std::option::Option;
use std::result::Result;
use std::vec::Vec;

pub trait Functor<A, B> {
    type Output;
    fn fmap<F: Fn(&A) -> B>(&self, F) -> Self::Output;
}

impl<A,B> Functor<A, B> for Option<A> {
    type Output = Option<B>;
    fn fmap<F: Fn(&A) -> B>(&self, f: F) -> Option<B> {
        match *self {
            Some(ref x) => Some(f(x)),
            None => None,
        }
    }
}

impl<A, B, E: Clone> Functor<A, B> for Result<A, E> {
    type Output = Result<B, E>;
    fn fmap<F: Fn(&A) -> B>(&self, f: F) -> Result<B, E> {
        match *self {
            Ok(ref x) => Ok(f(x)),
            Err(ref e) => Err(e.clone()),
        }
    }
}

impl<A, B> Functor<A, B> for Vec<A> {
    type Output = Vec<B>;
    fn fmap<F: Fn(&A) -> B>(&self, f: F) -> Vec<B> {
        let mut result = Vec::new();
        for elem in self {
            result.push(f(elem));
        }
        result
    }
}
