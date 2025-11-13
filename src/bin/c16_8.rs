/***
在本练习中，您将学习一个非常简单的数据结构，并将其变成泛型的。该结构使用 std::collections::HashMap 来跟踪已经出现过的值以及每个值出现的次数。

Counter 的初始版本经过硬编码，仅适用于 u32 值。使结构体及其方法可用于所跟踪的值类型，以便 Counter 能够跟踪任何类型的值。

如果提前完成操作，请尝试使用 entry 方法将哈希查找次数减半，从而实现 count 方法。
 */

use std::collections::HashMap;
use std::hash::Hash;

/// Counter counts the number of times each value of type T has been seen.
struct Counter<T> {
    values: HashMap<T, u64>,
}

impl <T> Counter<T> where T : Eq + Hash {
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}