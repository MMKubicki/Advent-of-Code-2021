use super::command::Direction;

use nom::{
    character::complete::{alpha1, multispace1, one_of},
    combinator::{map_res, recognize},
    error::ParseError,
    multi::{many0, many1},
    sequence::{pair, preceded, tuple},
    IResult,
};

fn ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, many0(alpha1)))(input)
}

fn command(input: &str) -> IResult<&str, Direction> {
    map_res(ident, str::parse)(input)
}

fn preceded_whitespace<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace1, inner)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789")))(input)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(decimal, str::parse::<usize>)(input)
}

pub fn line(input: &str) -> IResult<&str, (Direction, usize)> {
    tuple((command, preceded_whitespace(number)))(input)
}
