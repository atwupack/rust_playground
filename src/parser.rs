#[derive(Debug)]
pub enum ParserError {
    EmptyIterator,
}

pub trait Parser<I, R, IT: Iterator<Item=I>> {
    fn run(&self, iter: &mut IT) -> Result<R, ParserError>;
}

pub struct SimpleParser<I, R, IT: Iterator<Item=I>> {
    run_parser: Box<Fn(&mut IT) -> Result<R, ParserError>>,
}

impl<I, R, IT: Iterator<Item=I>> Parser<I, R, IT> for SimpleParser<I, R, IT> {
    fn run(&self, iter: &mut IT) -> Result<R, ParserError> {
        (self.run_parser)(iter)
    }
}

pub struct MapParser<I, R, IT: Iterator<Item=I>, B> {
    parser: Box<Parser<I, R, IT>>,
    f: Box<Fn(R) -> B>,
}

impl<I, R, IT: Iterator<Item=I>, B> Parser<I, B, IT> for MapParser<I, R, IT, B> {
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

impl<I: 'static, R: 'static, IT: 'static + Iterator<Item=I>> SimpleParser<I, R, IT> {
    pub fn fmap<B, F: 'static + Fn(R) -> B>(self, f: F) -> MapParser<I,R,IT,B> {
        MapParser{
            parser: Box::new(self),
            f: Box::new(f),
        }
    }
}