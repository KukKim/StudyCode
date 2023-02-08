use oop_ex02_trait::Draw;
use oop_ex02_trait::{Screen, Button, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width:75,
                height:10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("I don't know"),
                ],
            }),
            Box::new(Button {
                width:50,
                height:10,
                label: String::from("check"),
            }),
            // cannot compile if not implement 'Draw'
            // Box::new(String::from("is it work?")),
        ]
    };
    screen.run();
}
