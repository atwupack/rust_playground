#[derive(Debug)]
pub enum ParserError {
    EmptyIterator,
}

pub trait Parser {
    fn run<I, R, IT: Iterator<Item=I>>(&self, iter: &mut IT) -> Result<R, ParserError>;

    fn map<B, F, R>(self, f: F) -> MapParser<Self, F> 
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

/// Simple parser consisting of a parsing function.
pub struct SimpleParser<F> {
    parser_func: F,
}

impl<F> Parser for SimpleParser<F> {
    fn run<I, R, IT>(&self, iter: &mut IT) -> Result<R, ParserError> 
        where   
            IT: Iterator<Item=I>,
            F: Fn(IT) -> Result<R, ParserError>,
    {
        (self.parser_func)(iter)
    }
}

/// parser mapping another parser over a function.
pub struct MapParser<P,F> {
    parser: P,
    func: F,
}

impl<P, F> Parser for MapParser<P, F>
    where
        P: Parser,
{
    fn run(&self, iter: &mut IT) -> Result<B, ParserError> {
        let result = (self.parser).run(iter);
        match result {
            Ok(o) => Ok((self.func)(o)),
            Err(e) => Err(e),
        }
    }
}


// pub fn run_parser<I, R, IT: Iterator<Item=I>, P: Parser<I, R, IT>>(parser: P, input: &mut IT) -> Result<R, ParserError> {
//     parser.run(input)
// }

pub fn any_token<IT, I, F>() -> SimpleParser<F> 
    where
        IT: Iterator<Item=I>,
        F: Fn(&mut IT) -> Result<I, ParserError>,
{
    SimpleParser {
        parser_func: |it| {
            let item: Option<I> = it.next();
            match item {
                Some(i) => Ok(i),
                None => Err(ParserError::EmptyIterator),
            }
        },
    }
}


