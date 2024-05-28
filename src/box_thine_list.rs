//use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
enum List {
    Int(i64),
    Float(f64),
    Text(String),
}

#[derive(Debug)]
struct BoxThineList {
    chest: Vec<List>
}

impl BoxThineList {
    fn new() -> BoxThineList {
        BoxThineList {
            chest: Vec::new()
        }
    }

    fn append (&mut self, item: List ) {
        match item {
            List::Float(f) => self.chest.push(List::Float(f)),
            List::Int(i) => self.chest.push(List::Int(i)),
            List::Text(t) => self.chest.push(List::Text(t)),

        }
    }
}

trait IntoThineBox {
    fn into_thine_box(self) -> List;
}

impl IntoThineBox for String {
    fn into_thine_box(self) -> List {
        List::Text(self)
    }
}

impl IntoThineBox for i64 {
    fn into_thine_box(self) -> List {
        List::Int(self)
    }
}

impl IntoThineBox for f64 {
    fn into_thine_box(self) -> List {
        List::Float(self)
    }
}

impl List {
    fn show(&self) -> String {
        match self {
            List::Float(f) => f.to_string(),
            List::Int(i) => i.to_string(),
            List::Text(t) => t.to_string(),
        }
    }    
}

fn main(){
    let mut container = BoxThineList::new();
    container.append("Alan".to_owned().into_thine_box());
    container.append(95.into_thine_box());
    container.append(3.143.into_thine_box());
    container.chest.into_iter().for_each(|item| {
        println!("Showing Item: {:#?}", item.show());
        match item {
            List::Int(x) => println!("Revealing actual value: {:#?}", x),
            List::Float(x) => println!("Revealing actual value: {:#?}", x),
            List::Text(x) => println!("Revealing actual value: {:#?}", x),
        }
    })
}