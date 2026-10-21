pub const BASE_PATTERN_COUNT: usize = 21;
pub const BASE_PATTERNS: [&str; BASE_PATTERN_COUNT] = [
    "____XX____",
    "O___XX____",
    "O___XX___O",
    "O__XX____",
    "O__XX___O",
    "O__XX__O",
    "O_XX____",
    "O_XX___O",
    "OXX____",
    "___XXX___",
    "O__XXX___",
    "O__XXX__O",
    "O_XXX___",
    "O_XXX__O",
    "OXXX___",
    "__XXXX__",
    "O_XXXX__",
    "O_XXXX_O",
    "OXXXX__",
    "_XXXXX_",
    "OXXXXX_",
];

pub fn pattern_permutations(line: &str) -> [String; 4] {
    let base: String = String::from(line);
    let swapped: String = line.chars().map(|c| match c {
        'X' => 'O',
        'O' => 'X',
        other => other,
    }).collect();
    let reversed = base.chars().rev().collect();
    let swapped_reversed = swapped.chars().rev().collect();
    
    [base, swapped, reversed, swapped_reversed]
}