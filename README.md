
# json_io [![Build Status](https://travis-ci.org/mitchmindtree/json_io.svg?branch=master)](https://travis-ci.org/mitchmindtree/json_io)

A tool for simplifying saving and loading serializable types to and from json files.

It looks like this:

```Rust

extern crate find_folder;
extern crate json_io;

fn main() {
    let test_string = "This is a json_io test!".to_owned();
    let target = find_folder::Search::Parents(1).for_folder("target").unwrap();
    let path = target.join("test");
    json_io::save(&path, &test_string).unwrap();
    let the_string: String = json_io::load(&path).unwrap();
    assert_eq!(&test_string, &the_string);
    println!("{:?}", the_string);
}

```

You can add it to your project by adding this to your Cargo.toml:

```toml
[dependencies]
json_io = "*"
```


