/***
In this short exercise, you will implement a generic min function that determines the minimum of two values, using the Ord trait.
 */

use std::cmp::Ordering;

fn min<T>(x: T, y: T) -> T
where
    T: Ord,
{
    if x < y { x } else { y }
}

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
