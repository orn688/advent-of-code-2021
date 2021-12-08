use lazy_static::lazy_static;

lazy_static! {
    // Length of each digit that's represented by a unique number of segments.
    static ref UNIQUE_DIGIT_LENGTHS: Vec<usize> = vec![2, 3, 4, 7];
}

pub fn part1(input: &str) -> Result<String, String> {
    let lines = input.trim().lines();
    let count = lines
        .map(|line| line.split(" | ").last().unwrap())
        .map(|half| {
            half.split_whitespace()
                .filter(|digit| UNIQUE_DIGIT_LENGTHS.contains(&digit.len()))
                .count()
        })
        .sum::<usize>();
    Ok(count.to_string())
}

pub fn part2(_: &str) -> Result<String, String> {
    Ok(String::new())
}

#[allow(dead_code)]
const TEST_INPUT: &str = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "26");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "");
}
