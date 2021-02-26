use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        (u64::from(self.weight) * u64::from(other.value))
            .cmp(&(u64::from(self.value) * u64::from(other.weight)))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        u64::from(self.weight) * u64::from(other.value)
            == u64::from(self.value) * u64::from(other.weight)
    }
}

pub fn get_max_knapsack_value(capacity: u32, items: &Vec<Item>) -> f64 {
    let mut items = items.clone();
    items.sort();
    
    let mut value: f64 = 0.0;
    let mut capacity = capacity;

    for i in 0 .. items.len() {
        if capacity > items[i].weight {
            capacity -= items[i].weight;
            value += f64::from(items[i].value);
        } else {
            value += f64::from(items[i].value * capacity) / f64::from(items[i].weight);
            break;
        }
    }

    value
}

#[cfg(test)]
mod tests_get_max_knapsack_value {
    use super::{
        get_max_knapsack_value,
        Item
    };

    #[test]
    fn test0() {
        let items: Vec<Item> = vec![
            Item { weight: 20, value: 40 },
            Item { weight: 60, value: 30 },
        ];
        let value = get_max_knapsack_value(40, &items);

        assert_eq!(value, 50.0);
    }
}
