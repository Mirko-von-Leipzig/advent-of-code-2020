fn main() {
    // Create an instance of my ExpenseReport type which is responsible for solving the problems
    //
    // include_str! - macro which embeds my input at compile time
    // .parse::<T>  - fallable parsing from string into my ExpenseReport type
    //                this is only possible because we implement the Trait FromStr for ExpenseReport
    // .expect      - adds a bit of context if the above fails
    let expense_report = include_str!("../../inputs/01.txt")
        .parse::<ExpenseReport>()
        .expect("parsing expense report");

    // Solve parts 1 and 2.
    // Maybe a good place to elaborate on 'expect':
    //
    // If I brute `unwrap()` an Option or Result type, and
    // there is actually no value then the program will !!!panic!!!.
    // `expect` is the same as `unwrap` but adds a bit of context to
    // the panic message.
    //
    // The general expectation is that library code *never* uses `unwrap`
    // or `expect`, but will instead bubble that up to the user.
    // The exception to this is you are **certain** it can never panic.
    let part_1 = expense_report.part_1().expect("part 1");
    let part_2 = expense_report.part_2().expect("part 2");
    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

// A basic type which basically just wraps a vector of numbers.
//
// I made my own so that I can extend it with my own functions,
// which will also let me unit test it.
//
// These could all be replaced by free-standing functions, but I
// feel it reads more nicely by calling it an `ExpenseReport`
struct ExpenseReport {
    entries: Vec<u32>,
}

// Functions of the ExpenseReport
impl ExpenseReport {
    // Constructor -- note that this could literally be called anything.
    // It is a static function (due to the lack of `self` parameter).
    //
    // new() is the industry standard naming for most constructors.
    // Other good names are `default` (which usually takes 0 parameters,
    // because otherwise its no default?).
    //
    // Many can be defined, but no overloads so each needs a unique name.
    // This makes naming important, but makes it a lot more readable imo.
    //
    // For example, std::vec<T> has a variety of construction methods,
    // ::new() -> new empty vector
    // ::with_capacity(N) -> empty vector with space for N elements
    fn new(mut entries: Vec<u32>) -> Self {
        // Self is syntactic sugar for the type name. So this function
        // takes a mutable vector of u32, and returns a Self==ExpenseReport

        // I sort the entries to make finding elements faster.
        // This forces me to take entries as mut.
        //
        // Note also that its not &mut (which is a borrow), but rather
        // that my function states I am taking (and keeping) entries.
        // The caller won't be able to use entries after this
        // (he doesn't get it back).
        entries.sort();

        // In general you can construct a struct T by
        // T {
        //  field_1: value_1,
        //  field_2: value_2
        // }
        // but if you have matching parameters names you can shorten it.
        Self { entries } // short for Self { entries: entries }
    }

    // Helper function, which searches through a slice (i.e. sub-vector) of u32s,
    // to find a pair which sums to the target value. This is used to simplify part_2
    // (and is pretty much the entire part_1).
    //
    // It returns an optional tuple of two u32s (aka a pair).
    fn find_pair(entries: &[u32], target: u32) -> Option<(u32, u32)> {
        for entry in entries {
            if *entry <= target {
                if entries.contains(&(target - entry)) {
                    // Note the double brackets, inner brackets for the tuple pair,
                    // outer brackets to construct the Option
                    //
                    // We use an explicit return here. It is idiomatic to not use return
                    // when possible, but early returns require it.
                    return Some((*entry, target - entry));
                }
            }
        }

        // no return
        None
    }

    // Lower letter self (not Self == type)
    // self is syntactic sugar for `self: Self` and is equivalent to `this`
    // in other languages.
    //
    // Note that we take it by reference &self (borrowing it),
    // otherwise we would not be able to call part_2() after as self would
    // be consumed.
    fn part_1(&self) -> Option<u32> {
        let target = 2020;

        // If I find a pair (a + b) == 2020, map that to a*b, otherwise
        // it will simply return None (because find_pair does).
        //
        // `map` converts an Option<T> into an Option<U>.
        // In this case T = (u32, u32), U = u32.
        // The `None` is propagated.
        //
        // There is also a map function defined for Result which functions the same.
        //
        // Notice how we are unpacking the tuple (a,b), to make it easier to read. We
        // could also do `map(|x| x.0 * x.1)` (as x would be the entire tuple).
        Self::find_pair(&self.entries, target).map(|(a, b)| a * b)
    }

    // Same deal as part_1, we borrow an instance of ExpenseReport.
    // Algorithm is just slightly more involved.
    //
    // There is an unrolled version of this function `part_2_loopy`,
    // if this gets too confusing.
    fn part_2(&self) -> Option<u32> {
        let target = 2020;

        self.entries
            // iterate over entries
            .iter()
            // enumerate adds an index &v -> (i, &v)
            .enumerate()
            // Skip obviously wrong values.
            // Triple can't start at second last or last index.
            .filter(|(i, &v)| v <= target && i + 2 < self.entries.len())
            // Returns the first non-None value, or None if None :)
            .find_map(|(i, &v)| {
                // find pair (a,b) if any, then return that as a*b*v
                Self::find_pair(&self.entries[i + 1..], target - v).map(|(a, b)| a * b * v)
            })
    }

    // this disables the unused code warning (since I'm not using this function anywhere)
    #[allow(dead_code)]
    fn part_2_loopy(&self) -> Option<u32> {
        let target = 2020;

        // a more-loopy version
        for (i, v) in self.entries.iter().enumerate() {
            // this is the `filter`
            if *v <= target && i + 1 < self.entries.len() {
                // `find_map` ~ sort of
                if let Some((a, b)) = Self::find_pair(&self.entries[i + 1..], target - *v) {
                    return Some(a * b * *v);
                }
            }
        }

        None
    }
}

// Implementation of std library trait used by string.parse<T>().
//
// Basically, implementating this allows us to:
// let expense_report = "123\n456\n789".parse::<ExpenseReport>().unwrap();
impl std::str::FromStr for ExpenseReport {
    // The trait requires that we define the error type
    // (string parsing is obviously fallible).
    // In our case we are parsing ints, and such an error type
    // already exists, so I just re-use it.
    type Err = std::num::ParseIntError;

    // Actual parsing
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // read integers line-by-line
        let entries: Vec<u32> = s
            // line iterator over the string
            .lines()
            // map each line to an integer
            .map(|line| line.parse::<u32>())
            // collect into a vector (Result because the above line parse is fallible)
            // `?` will short-circuit bubble any error back out
            .collect::<Result<Vec<u32>, Self::Err>>()?;

        Ok(Self::new(entries))
    }
}

// My white-box tests. I usually simply use the examples in the question as unit tests.
// This `#[cfg(test)]` enables the following section only when the feature flag for `test`
// is set. The `test` feature flag is built-in but one can easily create one's own for
// conditional compilation (such as OS specific code).
//
// The test flag is automatically set when you run any tests, so you don't have to worry
// it should all just work.
//
// IDE tip: Typing `tmod` should bring up the assist to create all this boiler-plate for you.
//          Typing `tf` does the same for a single test function (to use inside the section).
#[cfg(test)]
mod tests {
    // we are inside an inner namespace called tests, so we need to "import" our parent module,
    // or we can access parent structs and functions using `super::xx`.
    use super::*;

    #[test]
    fn pair() {
        let uut = ExpenseReport::new(vec![1721, 979, 366, 299, 675, 1456]);

        assert_eq!(uut.part_1(), Some(514579));
    }

    #[test]
    fn triple() {
        let uut = ExpenseReport::new(vec![1721, 979, 366, 299, 675, 1456]);

        assert_eq!(uut.part_2(), Some(241861950));
    }

    #[test]
    fn triple_loopy() {
        let uut = ExpenseReport::new(vec![1721, 979, 366, 299, 675, 1456]);

        assert_eq!(uut.part_2_loopy(), Some(241861950));
    }

    // I added this test after having the correct results, in order to not regress
    // accidentily.
    #[test]
    fn solution() {
        let uut = include_str!("../../inputs/01.txt")
            .parse::<ExpenseReport>()
            .expect("parsing expense report");

        assert_eq!(uut.part_1(), Some(744475));
        assert_eq!(uut.part_2(), Some(70276940));
    }
}
