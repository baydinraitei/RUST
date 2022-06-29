fn isbig(input: String, taille: usize) -> bool {
    input.len() > taille
}

fn isbig2(input: &String, taille: usize) -> bool {
    input.len() > taille
}

fn inverse(v: f32) -> f32 {
    1.0 / v
}

fn safe_inverse(v: f32) -> Option<f32> {
    if 0.0 == v {
        None
    } else {
        Some(1.0 / v)
    }
}

fn somme(values: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    for i in values {
        result += i
    }
    result
}

fn maximum(values: &Vec<u32>) -> Option<&u32> {
    values.iter().max()
}

fn main() {
    let str1 = "Hello, world!".to_string();
    let str2 = String::from("Hello, world!");

    println!("{}", str1);
    println!("{}", str2);

    let name = "Tristan".to_string();
    println!("{}", isbig(name.clone(), 10));
    println!("{}", isbig(name.clone(), 5));

    let name2 = "Benoit".to_string();
    println!("{}", isbig2(&name2, 10));
    println!("{}", isbig2(&name2, 5));

    println!("{}", inverse(12.0));
    println!("{}", inverse(-5.0));
    println!("{}", inverse(0.0));

    match safe_inverse(12.0) {
        None => println!("Vide"),
        Some(x) => println!("Pas vide: {}", x),
    }

    match safe_inverse(-5.0) {
        None => println!("Vide"),
        Some(x) => println!("Pas vide: {}", x),
    }

    match safe_inverse(0.0) {
        None => println!("Vide"),
        Some(x) => println!("Pas vide: {}", x),
    }

    let v = vec![1, 2, 3, 4];

    println!("{:?}", v); //Print [1, 2, 3, 4]
    for i in v {
        print!("{} ", i)
    }

    let mut v2 = vec![3, 756, 2];
    v2.push(69);

    for i in &v2 {
        println!("{} ", i)
    }

    println!("{}", somme(&v2));
    v2.push(420);
    println!("{}", somme(&v2));

    match maximum(&v2) {
        None => println!("Vecteur Vide"),
        Some(x) => println!("Max : {}", x),
    }

    let mut v3: Vec<u32> = vec![];

    match maximum(&v3) {
        None => println!("Vecteur Vide"),
        Some(x) => println!("Max : {}", x),
    }
}
