use parser::SimpleParser;
use parser::ParserError;

pub fn any_token<I, IT>() -> SimpleParser<I, IT, I>
    where
        IT: Iterator<Item=I>,
{
    SimpleParser {
        parser_func: Box::new(|it| {
            let item = it.next();
            match item {
                Some(i) => Ok(i),
                None => Err(ParserError::NoMoreTokens),
            }
        }),
    }
}

