use std::*;

use crate::test_set::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Query {
    Type1(usize),
    Type2(usize),
}

pub struct Template {}

impl Jury for Template {
    type Input = TemplateInput;
    type Output = TemplateOutput;
    const TL: Duration = Duration::from_millis(TEMPLATE);
    const PROBLEM: Oj = Template("Template");
    fn parse_input(input: String) -> Self::Input {
        let mut input = input.lines();
    }
    fn parse_output(_: &Self::Input, output: String) -> Self::Output {
        let mut output = output.lines();
    }
}
