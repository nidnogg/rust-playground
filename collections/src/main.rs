use std::collections::HashMap;
fn main() {
    vecs();
    hash_maps();
}

#[derive(Debug)]
struct Person<'a> {
    age: u32,
    gender: &'a str,
}

fn hash_maps() {
    let mut map1 = HashMap::new();

    map1.insert("Jake", Person { age: 18, gender: "Male" });
    map1.insert("Karen", Person { age: 22, gender: "Female" });

    println!("Jake's attributes are {:?}", map1.get("Jake").unwrap());


    // Sample from book that shows that accessing borrowed elements like String doesn't quite work for hashmaps
    // (i32 is not borrowed, but String is)
    let mut map2 = HashMap::new();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map2.insert(field_name, field_value);

    // Can't use field_name because it was moved into the hashMap (moved borrow value)
    // println!("{}", map2.get(field_name).unwrap());


}

fn vecs() {
    let mut v = vec![1,2,3,4,5];

    // This panics if the element does not exist
    // let first = &v[0];

    // Gets option of second element. .get() syntax does not panic if None
    // Great for sniffing out end of vectors
    // let first = v.get(0).unwrap();

    let mut first = &v[0].clone();

    v.push(6);

    println!("{}", first);

    println!("Editing out first. Vector before edit:");
    println!("{:?}", v);
    first = &999;

    println!("Vector after edit:");
    println!("{}", first);
    println!("{:?}", v);
}
