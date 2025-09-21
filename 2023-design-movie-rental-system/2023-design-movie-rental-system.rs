use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

#[derive(Eq, PartialEq, Clone)]
struct Entry {
    price: i32,
    shop: i32,
    movie: i32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.price.cmp(&other.price) {
            Ordering::Equal => match self.shop.cmp(&other.shop) {
                Ordering::Equal => self.movie.cmp(&other.movie),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MovieRentingSystem {
    unrented: HashMap<i32, BTreeSet<Entry>>,
    price: HashMap<(i32, i32), i32>,
    rented: BTreeSet<Entry>,
}

impl MovieRentingSystem {
    fn new(_: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut unrented = HashMap::new();
        let mut prices = HashMap::new();

        for entry in entries {
            let shop = entry[0];
            let movie = entry[1];
            let price = entry[2];
            let entry = Entry { price, shop, movie };

            unrented
                .entry(movie)
                .or_insert_with(BTreeSet::new)
                .insert(entry.clone());
            prices.insert((shop, movie), price);
        }

        Self {
            unrented,
            price: prices,
            rented: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.unrented
            .get(&movie)
            .map(|set| set.iter().take(5).map(|e| e.shop).collect())
            .unwrap_or_else(Vec::new)
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        if let Some(&p) = self.price.get(&(shop, movie)) {
            let e = Entry {
                price: p,
                shop,
                movie,
            };
            if let Some(set) = self.unrented.get_mut(&movie) {
                set.remove(&e);
            }
            self.rented.insert(e);
        }
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        if let Some(&p) = self.price.get(&(shop, movie)) {
            let e = Entry {
                price: p,
                shop,
                movie,
            };
            self.rented.remove(&e);
            self.unrented
                .entry(movie)
                .or_insert_with(BTreeSet::new)
                .insert(e);
        }
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented
            .iter()
            .take(5)
            .map(|e| vec![e.shop, e.movie])
            .collect()
    }
}