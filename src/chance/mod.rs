use itertools::{Either, Itertools};
use rand::Rng;

/// It's often helpful to have weighted probabilities.
/// This struct serves as a sort of weighted bag; you can give it entries
/// with various weights, and then randomly sample them.
///
/// This is the way Minecraft loot tables work, if this sounds familiar.
///
/// The algorithm used is [Vose's Alias Method](https://www.keithschwarz.com/darts-dice-coins/)
/// (scroll to the bottom), which to be honest I absolutely do not understand.
/// But it has O(n) creation and O(1) selection, so sounds good to me.
///
/// You can't edit the probabilities after you've created it due to the algorithm.
#[derive(Debug, Clone)]
pub struct WeightedPicker<T> {
    prob: Vec<f64>,
    alias: Vec<usize>,
    items: Vec<T>,
}

impl<T> WeightedPicker<T> {
    /**
    Initialize a WeightedPicker from the given
    items and weights.

    Panics if you pass it an empty Vec.

    ```
    # use cogs_gamedev::chance::WeightedPicker;

    let picker = WeightedPicker::new(vec![
        ("common", 10.0),
        ("uncommon", 5.0),
        ("rare", 2.0),
        ("legendary", 1.0),
        ("mythic", 0.1),
    ]);

    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        println!("- {}", picker.get(&mut rng));
    }

    /*
        A sample output:
        - legendary
        - rare
        - uncommon
        - common
        - common
        - rare
        - uncommon
        - common
        - common
        - uncommon
    */
    ```

    */
    pub fn new(entries: Vec<(T, f64)>) -> Self {
        assert_ne!(entries.len(), 0, "Cannot use an empty vec!");

        let total_weight: f64 = entries.iter().map(|(_, weight)| *weight).sum();
        let len = entries.len();
        let average = (len as f64).recip();

        let (items, weights): (Vec<_>, Vec<_>) = entries.into_iter().unzip();

        let (mut small, mut large): (Vec<_>, Vec<_>) = weights
            .iter()
            .enumerate()
            .map(|(idx, weight)| {
                let prob = weight / total_weight * len as f64;
                (idx, prob)
            })
            .partition_map(|(idx, prob)| {
                // true goes to small, false to large
                if prob < average {
                    Either::Left(idx)
                } else {
                    Either::Right(idx)
                }
            });

        let mut alias = vec![0; len];
        let mut prob = vec![0.0; len];

        while !small.is_empty() && !large.is_empty() {
            // what do you mean this is great rust code
            let less = small.pop().unwrap();
            let more = large.pop().unwrap();

            prob[less] *= len as f64;
            alias[less] = more;

            let prev_more = prob[more];
            let prev_less = prob[less];
            prob[more] = prev_more + prev_less - average;

            if prob[more] >= average {
                large.push(more)
            } else {
                small.push(more);
            }
        }
        while let Some(last) = small.pop() {
            prob[last] = 1.0;
        }
        while let Some(last) = large.pop() {
            prob[last] = 1.0;
        }

        debug_assert_eq!(prob.len(), len);
        debug_assert_eq!(alias.len(), len);
        debug_assert_eq!(items.len(), len);

        Self { alias, items, prob }
    }

    /// Get an item from the list.
    pub fn get<R: Rng + ?Sized>(&self, rand: &mut R) -> &T {
        let column = rand.gen_range(0..self.prob.len());
        let coin_toss = rand.gen::<f64>() < self.prob[column];
        let idx = if coin_toss {
            column
        } else {
            self.alias[column]
        };
        &self.items[idx]
    }
}

// doctests don't println so let's replicate that test
#[test]
fn pick() {
    let picker = WeightedPicker::new(vec![
        ("common", 10.0),
        ("uncommon", 5.0),
        ("rare", 2.0),
        ("legendary", 1.0),
        ("mythic", 0.1),
    ]);

    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        println!("- {}", picker.get(&mut rng));
    }
}
