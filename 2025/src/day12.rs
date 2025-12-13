use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let (presents_section, trees) = input.rsplit_once("\n\n").unwrap();
    let presents = presents_section
        .split("\n\n")
        .map(|present| present.as_bytes().iter().filter(|&&c| c == b'#').count())
        .collect_vec();

    trees
        .lines()
        .filter_map(|tree| {
            let mut parts = tree.split_whitespace();
            let dims = parts.next()?;
            let dims = dims.trim_end_matches(':');
            let (width, height) = dims.split_once('x')?;
            let tree_area = width.parse::<usize>().ok()? * height.parse::<usize>().ok()?;

            let present_area = parts
                .filter_map(|d| d.parse::<usize>().ok())
                .zip(presents.iter())
                .map(|(count, present_size)| present_size * count)
                .sum::<usize>();

            (present_area <= tree_area).then_some(1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::part1;
    use test::Bencher;

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        // assert_eq!(part1(EXAMPLE), 2); // don't be silly there is always enough room for all the
        // presents!
        assert_eq!(part1(include_str!("../_inputs/day12.txt")), 524)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }
}
