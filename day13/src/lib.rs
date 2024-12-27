use std::cmp::Ordering;

use itertools::Itertools;
use itertools::EitherOrBoth;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(untagged)]
pub enum Node {
    Number(u8),
    List(Vec<Node>),
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (l, r) => l.with_slice(|l| r.with_slice(|r| l.partial_cmp(r))),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

#[derive(Debug, PartialEq)]
enum OrderStatus {
    Ordered,
    NotOrdered,
    NA,
}

struct SimpleIntPair<'a> {
    left: &'a u8,
    right: &'a u8,
}

impl SimpleIntPair<'_> {
    fn get_order_status(&self) -> OrderStatus {
        if self.left < self.right {
            return OrderStatus::Ordered;
        } else if self.left > self.right {
            return OrderStatus::NotOrdered;
        } else {
            return OrderStatus::NA;
        };
    }
}

struct SimpleListPair<'a> {
    left: &'a Vec<u8>,
    right: &'a Vec<u8>,
}

impl SimpleListPair<'_> {
    fn get_order_status(&self) -> OrderStatus {
        let x = self.left.iter().zip_longest(self.right);
        for zipped in x {
            match zipped {
                EitherOrBoth::Both(l, r) => {
                    let a = SimpleIntPair{left: l, right: r};
                    match a.get_order_status() {
                        OrderStatus::Ordered => return OrderStatus::Ordered,
                        OrderStatus::NotOrdered => return OrderStatus::NotOrdered,
                        OrderStatus::NA => continue,
                    }
                },
                EitherOrBoth::Left(l) => {
                    return OrderStatus::NotOrdered;
                },
                EitherOrBoth::Right(r) => {
                    return OrderStatus::Ordered;
                }
            }
        }
        return OrderStatus::NA;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = SimpleListPair{ left: &vec![1, 2, 3], right: &vec![1, 2, 4]};
        println!("{:?}", a.get_order_status());
        assert_eq!(a.get_order_status(), OrderStatus::Ordered)
    }

    #[test]
    fn test_2() {
        let a = SimpleListPair{ left: &vec![1, 2, 3], right: &vec![1, 2, 3]};
        println!("{:?}", a.get_order_status());
        assert_eq!(a.get_order_status(), OrderStatus::NA)
    }

    #[test]
    fn test_3() {
        let a = SimpleListPair{ left: &vec![1, 2, 3], right: &vec![1, 2, 2]};
        println!("{:?}", a.get_order_status());
        assert_eq!(a.get_order_status(), OrderStatus::NotOrdered)
    }

    #[test]
    fn test_4() {
        let a = SimpleListPair{ left: &vec![1, 2, 3], right: &vec![1, 2]};
        println!("{:?}", a.get_order_status());
        assert_eq!(a.get_order_status(), OrderStatus::NotOrdered)
    }
}