/// Memoizer pattern.
#[derive(Clone, PartialEq)]
pub struct Memoizer<T, V>
where
    T: Fn(T) -> V + std::cmp::PartialEq + Clone,
{
    method: T,
    memoized: Vec<(T, V)>,
    capacity: usize,
}

impl<T, V> Memoizer<T, V>
where
    T: Fn(T) -> V + std::cmp::PartialEq + Clone,
    V: Clone,
{
    /// Creates a new memoizer.
    /// It will store some # of results
    /// and return matches.
    pub fn new(method: T, saved_results: usize) -> Self {
        Self {
            method,
            memoized: Vec::with_capacity(saved_results),
            capacity: saved_results,
        }
    }

    /// Calls the original method.
    pub fn call(&mut self, args: T) -> V {
        for (key, value) in self.memoized.iter() {
            if key == &args {
                return (value).clone();
            }
        }

        if self.memoized.len() >= self.capacity {
            self.memoized.remove(0);
        }

        let value = (self.method)(args.clone());
        self.memoized.push((args, value.clone()));

        value
    }
}
