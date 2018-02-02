#[derive(Debug)]
pub enum ParserError {
    EmptyIterator,
}

pub trait Parser<I, R, IT: Iterator<Item=I>> {
    fn run(&self, iter: &mut IT) -> Result<R, ParserError>;

    fn map<B, F>(self, f: F) -> MapParser<Self, F> 
        where
            Self: Sized,
            F: Fn(R) -> B ,
    {
        MapParser{
            parser: self,
            func: f,
        }
    }
}

pub struct SimpleParser<I, R, IT: Iterator<Item=I>> {
    run_parser: Box<Fn(&mut IT) -> Result<R, ParserError>>,
}

impl<I, R, IT: Iterator<Item=I>> Parser<I, R, IT> for SimpleParser<I, R, IT> {
    fn run(&self, iter: &mut IT) -> Result<R, ParserError> {
        (self.run_parser)(iter)
    }
}

pub struct MapParser<P,F> {
    parser: P,
    func: F,
}

impl<R, P, F, I, IT: Iterator<Item=I>, B> Parser<I, B, IT> for MapParser<P, F>
    where
        P: Parser<I, R, IT>,
{
    fn run(&self, iter: &mut IT) -> Result<B, ParserError> {
        let result = (self.parser).run(iter);
        match result {
            Ok(o) => Ok((self.f)(o)),
            Err(e) => Err(e),
        }
    }
}


// pub fn run_parser<I, R, IT: Iterator<Item=I>, P: Parser<I, R, IT>>(parser: P, input: &mut IT) -> Result<R, ParserError> {
//     parser.run(input)
// }

pub fn any_token<I, IT: Iterator<Item=I>>() -> SimpleParser<I,I,IT> {
    SimpleParser {
        run_parser: Box::new(|it| {
            let item = it.next();
            match item {
                Some(i) => Ok(i),
                None => Err(ParserError::EmptyIterator),
            }
        }),
    }
}


