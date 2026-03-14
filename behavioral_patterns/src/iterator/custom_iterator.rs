mod users {
    pub struct UserCollection {
        users: [&'static str; 3],
    }

    /// A custom collection contains an arbitrary user array under the hood.
    impl UserCollection {
        pub fn new() -> Self {
            Self { users: ["Alice", "Bob", "Carl"] }
        }

        /// Returns an iterator over a user collection.
        ///
        /// The method name may be different, however, `iter` is used as a de facto
        /// standard in a Rust naming convention.
        pub fn iter(&self) -> UserIterator {
            UserIterator {
                index: 0,
                user_collection: self
            }
        }
    }

    /// UserIterator allows sequential traversal through a complex user collection
    /// without exposing its internal details.
    pub struct UserIterator<'a> {
        index: usize,
        user_collection: &'a UserCollection,
    }

    /// `Iterator` is a standard interface for dealing with iterators
    /// from the Rust standard library.
    impl Iterator for UserIterator<'_> {
        type Item = &'static str;
        
        /// A `next` method is the only `Iterator` trait method which is mandatory to be
        /// implemented. It makes accessible a huge range of standard methods,
        /// e.g. `fold`, `map`, `for_each`.
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.user_collection.users.len() {
                let user = Some(self.user_collection.users[self.index]);
                self.index +=1;
                user
            } else {
                None
            }    
        }
    }
}

pub fn ct_iterator_main() {
    print!("Iterators are widely used in the standard libaray: ");

    let array = &[1, 2, 3];
    let iterator = array.iter();

    iterator.for_each(|e| print!("{}, ", e));

    println!("\n\nLet's test our own iterator.\n");

    let users = self::users::UserCollection::new();
    let mut iterator = users.iter();

    println!("1st element: {:?}", iterator.next());
    println!("2nd element: {:?}", iterator.next());
    println!("3rd element: {:?}", iterator.next());
    println!("4th element: {:?}", iterator.next());

    print!("\nAll elements in user collection: ");
    users.iter().for_each(|e| print!("{}, ", e));

    println!();
}
