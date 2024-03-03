#[macro_use]
extern crate lazy_static;
use std::collections::{HashMap, HashSet};
lazy_static! {
static ref PARENS: HashMap<char, char> =
[('(', ')'), ('{', '}'), ('[', ']')].into_iter().collect();
static ref ALL_PARENS: HashSet<char> = PARENS.keys().chain(PARENS.values()).cloned().collect();
}
fn is_parenthesis(ch: char) -> bool {
ALL_PARENS.contains(&ch)
}
fn is_matching(opening: char, closing: char) -> bool {
if let Some(actual_closing) = PARENS.get(&opening) {
*actual_closing == closing
} else {
false
}
}
fn is_balanced(input: &str) -> bool {
let mut stack = vec![];
for ch in input.chars() {
if is_parenthesis(ch) {
match stack.last() {
Some(&opening) if is_matching(opening, ch) => {
                    stack.pop();
}
                _ => stack.push(ch),
}
}
}
    stack.is_empty()
}
fn main() {
dbg!(is_balanced("([]{()})"));
dbg!(is_balanced("([yo, foo]{(14)*89})"));
dbg!(is_balanced("([]{)})"));
dbg!(is_balanced("([yo, foo]{)*89})"));
dbg!(is_balanced("([]{()})]"));
dbg!(is_balanced("([]{(})"));
}
