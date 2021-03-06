/**
 * http://doc.rust-lang.org/std/collections/hashmap/struct.HashMap.html
 * https://github.com/rust-lang/rust/blob/master/src/test/run-pass/hashmap-memory.rs#L39
 * https://github.com/rust-lang/rust/blob/master/src/libstd/collections/hashmap.rs
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
extern crate debug;
use std::collections::hashmap::{HashMap, Occupied, Vacant};

fn main() {
	println!("Using borrowed pointers as keys.");
	let mut h: HashMap<&str, int>;
	h = HashMap::new();
	h.insert("foo", 42);
	println!("Is there a key foo?  => {}", h.contains_key(&("foo"))); // => true
	println!("Is there a key baz?  => {}", h.contains_key(&("baz"))); // => false
	println!("The value for foo is => {:?}", h.find(&("foo"))); // => Some(&42)
	let key = "baz";
	h.insert(key, 1);
	println!("Is there a key baz?  => {}", h.contains_key(&("baz"))); // => false

	// Doing a find, inserting with a `proc()`, using the key to construct the value
	let mut map = HashMap::<String, String>::new();
	match map.entry("foo".to_string()) {
		Vacant(entry) => { entry.set("bar".to_string()); },
		Occupied(mut entry) => { entry.get_mut().push_str("bar"); },
	}
	println!("The value for foo is => {:?}", map.find(&("foo".to_string()))); // => Some(&~"foobar")
	// running this for the first time, will add "foo" with the value 1
	// running the same for the second time, will add +1 to "foo"
	match h.entry("foo") {
		Vacant(entry) => { entry.set(1); },
		Occupied(mut entry) => { *entry.get_mut() += 1; },
	}
	println!("foo={}", h.get(&("foo")));
	assert_eq!(h["foo"], 43);

	// You don't actually need the HashMap to own the keys (but
	// unless all keys are static, this will be likely to lead
	// to problems, so I don't suggest you do it in reality)

	println!("Using owned pointers as keys.");
	let mut h = HashMap::<String, int>::new();
	h.insert("foo".to_string(), 42);
	println!("Is there a key foo?  => {}", h.contains_key(&"foo".to_string())); // => true
	println!("Is there a key baz?  => {}", h.contains_key(&"baz".to_string())); // => false
	println!("The value for foo is => {:?}", h.find(&"foo".to_string())); // => Some(&42)
	h.insert(key.to_string(), 1);
	println!("Is there a key baz?  => {}", h.contains_key(&"baz".to_string())); // => true

	// List keys of the HashMap
	let mut keys: Vec<String> = Vec::new();
	for (k, _) in h.iter() {
		keys.push(k.to_string());
	}
	println!("These are the keys: {}.", keys);
	let keys = h.keys().map(|v| v.clone()).collect::<Vec<String>>();
	println!("These are the keys: {}.", keys);

	// List values of the HashMap
	let values = h.values().map(|v| v.clone()).collect::<Vec<int>>();
	println!("These are the values: {}.", values);

	// type inference lets us omit an explicit type signature (which
	// would be `HashMap<&str, &str>` in this example).
	let mut book_reviews = HashMap::new();

	// review some books.
	book_reviews.insert("Adventures of Huckleberry Finn",    "My favorite book.");
	book_reviews.insert("Grimms' Fairy Tales",               "Masterpiece.");
	book_reviews.insert("Pride and Prejudice",               "Very enjoyable.");
	book_reviews.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");

	// check for a specific one.
	if !book_reviews.contains_key(&("Les Misérables")) {
		println!("We've got {} reviews, but Les Misérables ain't one.",
			 book_reviews.len());
	}

	// oops, this review has a lot of spelling mistakes, let's delete it.
	book_reviews.remove(&("The Adventures of Sherlock Holmes"));

	// look up the values associated with some keys.
	let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
	for book in to_find.iter() {
		match book_reviews.find(book) {
			Some(review) => println!("{}: {}", *book, *review),
			None => println!("{} is unreviewed.", *book)
		}
	}

	// iterate over everything.
	for (book, review) in book_reviews.iter() {
		println!("{}: \"{}\"", *book, *review);
	}
}
