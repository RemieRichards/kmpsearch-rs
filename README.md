# kmpsearch
Allows searching for bytes or strings within byte slices or strings, using the Knuth Morris Pratt algorithm.
KMP runs in O(n+h) time broken down as O(n) preparation, O(h) matching, where n is the length of the needle and h the length of the haystack.

Special thanks to PJB3005 for the help with AsRef<T> which makes this library nicer to use.
This is my first rust crate, so any feedback is welcome.
Pull requests are accepted on github.

## Usage:
Matching strings in strings
```rust
if "Hello World!".contains_needle("World") {
	println!("Matches!");
} else {
	println!("Doesn't match!");
}
```

Matching bytes in bytes
```rust
if b"DEADBEEF".contains_needle(b"BEEF") {
	println!("Matches!");
} else {
	println!("Doesn't match!");
}
```
