use std::env::Args;

use regex::Regex;

/*
    # Part 1

    The computer appears to be trying to run a program, but its memory (the puzzle input)
    is corrupted. All of the instructions have been jumbled up!

    The goal of the program is to multiply some numbers, doing so with instructions like
    `mul(x,y)`, where x, y are 1 to 3 digit numbers. For example, `mul(44,46)` would give
    `2024`.

    However, because the programs memory has been corrupted, there are also many invalid
    characters that should be ignored, even if they look like part of a `mul` instruction.
    Sequences like `mul(4*`, `mul(6,9!`, `?(12,34)`, or `mul ( 2 , 4 )` do nothing.

    For example, consider the following section of corrupted memory:

    `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
      ^------^                    ^------^                ^-------^-------^

    Only the four highlighted sections are real `mul` instructions. Adding up the result
    of each instruction produces `161`.

    Scan the corrupted memory for uncorrupted `mul` instructions, and return the sum.

    # Part 2

    There are two new instructions you'll need to handle:

    - The `do()` instruction enables future `mul` instructions
    - The `don't()` instruction disables future `mul` instructions

    Only the most recent `do`/`don't` instruction applies. At the beginning of the program,
    `mul` instructions are enabled.
*/

static CORRUPTED_MEMORY: &str = include_str!("../../res/day_3.txt");

pub fn solution(_args: Args) -> Result<i32, &'static str> {

    // Create a regular expression (that only ever creates one capture group) that will captures EITHER
    //  - The operands for a mul instruction, OR
    //  - do/don't
    let Ok(re) = Regex::new(r"(?:mul\(([0-9]{1,3},[0-9]{1,3})\)|(do)\(\)|(don't)\(\))") else {
        return Err("Unable to create Regex object for re");
    };

    // Create another regular expression to make parsing each operand of a mul instruction
    // easier, since the regex crate doesn't allow capturing each individually in this situation
    let Ok(nums_re) = Regex::new(r"([0-9]{1,3}),([0-9]{1,3})") else {
        return Err("Unable to create Regex object for nums_re");
    };

    let mut sum = 0i32;
    let mut enabled = true;

    for (_, [instruction]) in re.captures_iter(CORRUPTED_MEMORY)
                                        .map(|c| c.extract()) {
        match instruction {
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => {
                if enabled {
                    // Parse the numbers out of the mul instruction
                    let (_, [operand_a, operand_b]) = nums_re.captures(instruction).unwrap().extract();
                    sum += operand_a.parse::<i32>().unwrap() * operand_b.parse::<i32>().unwrap();
                }
            }
        }
    }

    Ok(sum)
}
