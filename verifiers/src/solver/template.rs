use std::*;

use crate::jury;
use crate::test_set::solver;

use jury::template::*;

use my_modules::*;

pub struct Template {}

impl Solver for Template {
    type Jury = jury::Template;
    fn solve(input: Self::Jury::Input) -> Self::Jury::Output {}
}

pub struct NekoTemplate {}

impl NekoTemplate {}
