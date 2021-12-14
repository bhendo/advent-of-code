pub fn part1(input: &str) -> usize {
    let (mut dots, folds) = parse_input(input);
    match folds.first() {
        Some(&(axis, line)) => fold(&mut dots, axis, line),
        _ => (),
    }
    dots.sort_unstable();
    dots.dedup();
    dots.len()
}

pub fn part2(input: &str) -> String {
    let (mut dots, folds) = parse_input(input);
    let (max_x, max_y) = folds
        .iter()
        .fold((100, 100), |(mut max_x, mut max_y), &(axis, line)| {
            match axis {
                "y" if line < max_y => max_y = line,
                "x" if line < max_x => max_x = line,
                _ => (),
            }
            fold(&mut dots, axis, line);
            (max_x, max_y)
        });
    dots.sort_unstable();
    dots.dedup();
    plot_points(&mut dots, max_x, max_y)
}

fn fold(dots: &mut Vec<(usize, usize)>, axis: &str, line: usize) {
    dots.iter_mut().for_each(|(x, y)| match axis {
        "y" => {
            if *y > line {
                *y = line - (*y - line)
            }
        }
        "x" => {
            if *x > line {
                *x = line - (*x - line)
            }
        }
        _ => (),
    });
}

fn plot_points(dots: &mut Vec<(usize, usize)>, max_x: usize, max_y: usize) -> String {
    let mut plot = vec![vec![" "; max_x]; max_y];
    dots.iter().for_each(|&(x, y)| plot[y][x] = "#");

    plot.iter()
        .map(|l| l.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<(&str, usize)>) {
    let (paper, instructions) = input.split_once("\n\n").unwrap();
    let dots = paper
        .lines()
        .map(|l| {
            l.split_once(",")
                .map(|(x, y)| {
                    (
                        usize::from_str_radix(x, 10).unwrap(),
                        usize::from_str_radix(y, 10).unwrap(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<(usize, usize)>>();
    let folds = instructions
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .last()
                .map(|s| {
                    s.split_once("=")
                        .and_then(|(a, l)| Some((a, usize::from_str_radix(l, 10).unwrap())))
                        .unwrap()
                })
                .unwrap()
        })
        .collect::<Vec<(&str, usize)>>();
    (dots, folds)
}

#[cfg(test)]
mod tests {
    use crate::day13::{part1, part2};

    const EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 17);
        assert_eq!(part1(include_str!("../../_inputs/day13.txt")), 765);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(EXAMPLE),
            "#####
#   #
#   #
#   #
#####
     
     "
        );
        assert_eq!(
            part2(include_str!("../../_inputs/day13.txt")),
            "###  #### #  # #### #    ###   ##  #  # 
#  #    # # #     # #    #  # #  # #  # 
#  #   #  ##     #  #    #  # #    #### 
###   #   # #   #   #    ###  # ## #  # 
# #  #    # #  #    #    #    #  # #  # 
#  # #### #  # #### #### #     ### #  # "
        )
    }
}
