pub fn part1(input: &str) -> usize {
    input
        .trim_end()
        .split(",")
        .map(|product_range| {
            println!("{product_range}");
            let (start, end) = product_range.split_once("-").unwrap();
            (start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
                .map(|product| {
                    let product_string = product.to_string();
                    let product_bytes = product_string.as_bytes();
                    if product_bytes.len() % 2 == 0 {
                        let (left, right) = product_bytes.split_at(product_string.len() / 2);
                        if left == right {
                            return product;
                        }
                    }
                    0
                })
                .sum::<usize>()
        })
        .sum()
}
pub fn part2(input: &str) -> usize {
    input
        .trim_end()
        .split(",")
        .map(|product_range| {
            let (start, end) = product_range.split_once("-").unwrap();
            (start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
                .map(|product| {
                    let product_string = product.to_string();
                    let product_bytes = product_string.as_bytes();
                    let product_len = product_bytes.len();
                    if product_len > 1 {
                        for block_size in 1..=product_len / 2 {
                            if product_len % block_size == 0 {
                                let (first_block, remaining_blocks) =
                                    product_bytes.split_at(block_size);
                                if remaining_blocks
                                    .chunks_exact(block_size)
                                    .all(|block| block == first_block)
                                {
                                    return product;
                                }
                            }
                        }
                    }
                    0
                })
                .sum::<usize>()
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
        assert_eq!(part1(include_str!("../_inputs/day2.txt")), 38158151648)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
        assert_eq!(part2(include_str!("../_inputs/day2.txt")), 45283684555)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
