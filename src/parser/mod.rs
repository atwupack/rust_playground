pub mod prim;

use std::boxed::FnBox;

#[derive(Debug)]
pub enum ParserError {
    NoMoreTokens,
}

pub trait Parser
    where
        Self: Sized,
{
    type In;
    type Iter : Iterator<Item=Self::In>;
    type Out;

    fn run(self, iter: &mut Self::Iter) -> Result<Self::Out, ParserError>;

    fn map<B, F>(self, f: F) -> MapParser<Self, F>
        where
            Self: Sized + 'static,
            F: Fn(Self::Out) -> B + Sized + 'static ,
    {
        MapParser{
            parser: self,
            func: f,
        }
    }

    fn bind<P2, F>(self, f: F) -> BindParser<Self, P2>
        where
            Self: Sized + 'static,
            F: FnOnce(Self::Out) -> P2 + Sized + 'static ,
            P2: Parser<In=Self::In, Iter=Self::Iter>,
    {
        BindParser{
            parser: self,
            func: Box::new(f),
        }
    }

    fn seq<P2>(self, p: P2) -> BindParser<Self, P2>
        where
            Self: Sized + 'static,
            P2: Parser<In=Self::In, Iter=Self::Iter> + 'static,
    {
        self.bind(|_r| {
            p
        })
    }
}

/// Simple parser consisting of a parsing function.
pub struct SimpleParser<I, IT, R>
    where
        IT: Iterator<Item=I>,
{
    parser_func: Box<Fn(&mut IT) -> Result<R, ParserError>>,
}

impl<I, R, IT> Parser for SimpleParser<I, IT, R>
    where
        IT: Iterator<Item=I>,
{
    type In=I;
    type Iter=IT;
    type Out=R;

    fn run(self, iter: &mut IT) -> Result<R, ParserError> {
        (self.parser_func)(iter)
    }
}

pub struct BindParser<P1, P2>
    where
        P1: Parser,
        P2: Parser,
{
    parser: P1,
    func: Box<FnBox(P1::Out) -> P2>,
}

impl<P1, P2> Parser for BindParser<P1, P2>
    where
        P1: Parser,
        P2: Parser<In=P1::In, Iter=P1::Iter> + 'static,
{
    type In=P1::In;
    type Iter=P1::Iter;
    type Out=P2::Out;

    fn run(self, iter: &mut Self::Iter) -> Result<P2::Out, ParserError> {
        let result = (self.parser).run(iter);
        match result {
            Ok(o) => {
                let p2 = (self.func)(o);
                p2.run(iter)
            }
            Err(e) => Err(e),
        }
    }
}


/// parser mapping another parser over a function.
pub struct MapParser<P, F>
{
    parser: P,
    func: F,
}

impl<P, F, B> Parser for MapParser<P, F>
    where
        P: Parser,
        F: Fn(P::Out) -> B,
{
    type In=P::In;
    type Iter=P::Iter;
    type Out=B;

    fn run(self, iter: &mut P::Iter) -> Result<B, ParserError> {
        let result = (self.parser).run(iter);
        match result {
            Ok(o) => Ok((self.func)(o)),
            Err(e) => Err(e),
        }
    }
}




