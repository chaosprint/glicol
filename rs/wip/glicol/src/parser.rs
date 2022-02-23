nom::bits::complete::tag;
nom::IResult;


pub fn node_name(input: &str) -> IResult<&str, &str> {
    alt((tag("sin"), tag("mul", tag("add"), tag("const_sig"))))
}

pub fn parse(input: &str) -> IResult<&str, &str> {
    
}