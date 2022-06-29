use chrono;
use chrono::Datelike; // 0.4.19

fn mad(a: i32, b: i32, c: i32) -> i32 {
    a * b + c
}

fn sum_from_to_while(a: i32, b: i32) -> i32 {
    let mut result = 0;
    let mut i = a;
    while i <= b {
        result += i;
        i += 1;
    }
    result
}

fn sum_from_to_for(a: i32, b: i32) -> i32 {
    let mut result = 0;

    for i in a..b + 1 {
        result += i
    }
    result
}

fn sum_from_to_recur(a: i32, b: i32) -> i32 {
    if a == b {
        b
    } else {
        a + sum_from_to_recur(a + 1, b)
    }
}

#[derive(Debug, Clone)]
struct Livre {
    titre: String,
    annee_publication: i32,
    genre: Genre,
}
#[derive(Debug, Clone)]
enum Genre {
    Fiction,
    Histoire,
    Fantasy,
    Informatique,
}

fn age_livre(livre: Livre) -> i32 {
    chrono::Utc::now().year() - livre.annee_publication
}

fn note_livre(livre: Livre) -> i32 {
    let note = livre.titre.len() as i32 + livre.annee_publication;
    match livre.genre {
        Genre::Fiction => note * 12,
        Genre::Histoire => note * 2,
        Genre::Fantasy => note * 36,
        Genre::Informatique => note * 41,
    }
}
#[derive(Debug)]
enum ResultatDivision {
    DivisionParZero,
    DivisionCorrecte(f32),
}

fn division1(a: f32, b: f32) -> ResultatDivision {
    if b == 0.0 {
        ResultatDivision::DivisionParZero
    } else {
        ResultatDivision::DivisionCorrecte(a / b)
    }
}

fn division2(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    println!("mad(2, 5, 7)) : {}", mad(2, 5, 7));
    println!("sum_from_to_while(2, 5)) : {}", sum_from_to_while(2, 5));
    println!("sum_from_to_for(2, 5)) : {}", sum_from_to_for(2, 5));
    println!("sum_from_to_recur(2, 5)) : {}", sum_from_to_recur(2, 5));
    let book = Livre {
        titre: "Ellana - Le Pacte des Marchombres".to_string(),
        annee_publication: 2006,
        genre: Genre::Fantasy,
    };
    println!("Le livre à {} ans", age_livre(book.clone()));
    println!("Note du livre : {}", note_livre(book.clone()));

    match division2(15.0, 3.0) {
        Some(x) => println!("Division avec Option: {:?}", x),
        None => println!("Impossible de diviser par 0"),
    }
    match division1(15.0, 3.0) {
        ResultatDivision::DivisionCorrecte(x) => println!("Division avec Enum: {:?}", x),
        ResultatDivision::DivisionParZero => println!("Impossible de diviser par 0"),
    }
    match division2(15.0, 0.0) {
        Some(x) => println!("Division avec Option: {:?}", x),
        None => println!("Impossible de diviser par 0 (Option)"),
    }
    match division1(15.0, 0.0) {
        ResultatDivision::DivisionCorrecte(x) => println!("Division avec Enum: {:?}", x),
        ResultatDivision::DivisionParZero => println!("Impossible de diviser par 0 (enum)"),
    }
}
