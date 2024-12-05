use shared::read_lines;
// use std::{collections::HashMap, ops};

#[derive(Debug, PartialEq)]
struct PageOrder {
    page: u32,
    before_page: u32,
}

#[derive(Debug, PartialEq)]
struct PageUpdate {
    pages: Vec<u32>,
}

fn main() {
    let filename = "./input";
    let (page_orders, pages_updates) = read_pages(filename);

    let sum = part1(&page_orders, &pages_updates);
    println!("part 1: {}", sum);

    // let found = part2(&matrix);
    // println!("part 2: {}", found);
}

fn part1(page_orders: &[PageOrder], pages_updates: &[PageUpdate]) -> u32 {
    get_correct_order_middle_numbers_iterator(page_orders, pages_updates).sum()
}

fn read_pages(filename: &str) -> (Vec<PageOrder>, Vec<PageUpdate>) {
    let lines = read_lines(filename);

    let (page_orders, pages_updates) = lines.fold(
        (Vec::<PageOrder>::new(), Vec::<PageUpdate>::new()),
        |(mut page_orders, mut pages_updates), line| {
            if line.contains('|') {
                let numbers = line
                    .clone()
                    .split('|')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                page_orders.push(PageOrder {
                    before_page: numbers[0],
                    page: numbers[1],
                });
            }
            if line.contains(',') {
                let numbers = line
                    .clone()
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                pages_updates.push(PageUpdate { pages: numbers });
            }

            (page_orders, pages_updates)
        },
    );

    (page_orders, pages_updates)
}

fn is_correct_order(page_orders: &[PageOrder], pages_updates: &PageUpdate) -> bool {
    let pages = &pages_updates.pages;
    pages.iter().enumerate().all(|(index, page)| {
        let other_pages = pages[index + 1..].to_vec();
        let is_in_correct_order = !page_orders
            .iter()
            .filter(|p| p.page == *page)
            .map(|p| p.before_page)
            .any(|before_page| other_pages.iter().any(|p| *p == before_page));

        is_in_correct_order
    })
}

fn get_correct_order_iter<'a>(
    page_orders: &'a [PageOrder],
    pages_updates: &'a [PageUpdate],
) -> impl Iterator<Item = &'a PageUpdate> {
    pages_updates
        .iter()
        .filter(|pu| is_correct_order(page_orders, pu))
}

fn get_correct_order_middle_numbers_iterator<'a>(
    page_orders: &'a [PageOrder],
    pages_updates: &'a [PageUpdate],
) -> impl Iterator<Item = &'a u32> {
    get_correct_order_iter(page_orders, pages_updates).map(|pu| &pu.pages[pu.pages.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let (page_orders, pages_updates) = read_pages("./input_example");
        assert_eq!(page_orders.len(), 21);
        assert_eq!(
            page_orders[0],
            PageOrder {
                before_page: 47,
                page: 53,
            }
        );
        assert_eq!(pages_updates.len(), 6);
        assert_eq!(pages_updates[0].pages, vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_correct_order() {
        let (page_orders, pages_updates) = read_pages("./input_example");
        assert_correct_order(&page_orders, &pages_updates, 0, true);
        assert_correct_order(&page_orders, &pages_updates, 1, true);
        assert_correct_order(&page_orders, &pages_updates, 2, true);
        assert_correct_order(&page_orders, &pages_updates, 3, false);
        assert_correct_order(&page_orders, &pages_updates, 4, false);
        assert_correct_order(&page_orders, &pages_updates, 5, false);
    }

    fn assert_correct_order(
        page_orders: &Vec<PageOrder>,
        pages_updates: &Vec<PageUpdate>,
        index: usize,
        expected: bool,
    ) {
        assert_eq!(
            is_correct_order(page_orders, &pages_updates[index]),
            expected
        );
    }

    #[test]
    fn test_correct_order_iterator() {
        let (page_orders, pages_updates) = read_pages("./input_example");
        let iter = get_correct_order_iter(&page_orders, &pages_updates);

        assert_eq!(iter.count(), 3);
    }

    #[test]
    fn test_correct_order_middle_numbers_iterator() {
        let (page_orders, pages_updates) = read_pages("./input_example");
        let numbers = get_correct_order_middle_numbers_iterator(&page_orders, &pages_updates)
            .collect::<Vec<_>>();

        assert_eq!(numbers.len(), 3);
        assert_eq!(numbers[0], &61);
        assert_eq!(numbers[1], &53);
        assert_eq!(numbers[2], &29);
    }

    #[test]
    fn test_part_1() {
        let (page_orders, pages_updates) = read_pages("./input_example");
        let sum = part1(&page_orders, &pages_updates);
        assert_eq!(sum, 143);
    }

    // #[test]
    // fn test_part_2() {
    //     let matrix = read_matrix("./input_example");
    //     let sum = part2(&matrix);
    //     assert_eq!(sum, 9);
    // }
}
