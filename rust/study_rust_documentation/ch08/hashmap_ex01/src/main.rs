use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("yellow"), 40);    

    // let teams = vec![String::from("blue"), String::from("yellow")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();




    // let field_name = String::from("Favorite color");
    // let field_value = String::from("blue");

    // println!("{} {}", field_name, field_value);

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // // println!("{} {}", field_name, field_value); 



    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
     
    // let team_name = String::from("blue");
    // let score = scores.get(&team_name);

    // println!("{:?}", score);

    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

}
