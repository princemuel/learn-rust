pub struct MeanCollection {
    list: Vec<i32>,
    average: f64,
}

impl MeanCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

use gui_lib::{Button, Draw, Screen};

struct SelectBox {
    width: u8,
    height: u8,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {}
}
fn main() {
    let screen = Screen {
        components: vec![
            // Box::new(String::from("yuck")),
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("ok"),
            }),
        ],
    };

    screen.run();
}
