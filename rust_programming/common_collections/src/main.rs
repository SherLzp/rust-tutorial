fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    v2.pop();
    println!("{:?}", v1);
    println!("{:?}", v2);

    let v3 = vec![1, 2, 3, 4, 5];
//    let third = &v3[2];
    let third = v3.get(2);
    println!("{:?}", third.unwrap());

    let s1 = String::from("Hello,");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{:?}", s3);

    let s1 = String::from("tic");
    let mut s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", &s1, &s2, &s3);
    let s4 = s1 + &s2 + &s3;
    s2.push_str("ok");
    println!("{:?}", s);
//    println!("{:?}", s1);
    println!("{:?}", s4);

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let mut scores = HashMap::new();
//    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
