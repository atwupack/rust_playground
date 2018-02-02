use std::option::Option;
use functor::Functor;

pub trait Monad<A, B>: Functor<A, B> {
    fn bind<F: Fn(&A) -> Self::Output>(&self, F) -> Self::Output;
    fn mpure(B) -> Self::Output;
}

impl<A : Clone, B> Monad<A, B> for Option<A> {
    fn bind<F: Fn(&A) -> Option<B>>(&self, f: F) -> Option<B> {
        match *self {
            Some(ref x) => f(x),
            None => None,
        }
    }
    fn mpure(input: B) -> Option<B> {
        Some(input)
    }
}
