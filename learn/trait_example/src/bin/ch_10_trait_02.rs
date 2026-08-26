use std::fmt;

struct Message {
    text: String,
}

trait MyMessageTrait {
    fn show(&self);
}

// 为自己的类实现自己的trait
impl MyMessageTrait for Message {
    fn show(&self) {
        println!("{}", self.text);
    }
}

// 为库类实现自己的trait
impl MyMessageTrait for String {
    fn show(&self) {
        println!("{}", self);
    }
}

// 为自己的类实现库里的trait
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

fn main() {
    let my_message_trait = Message { text: "    Message for MyMessageTrait".into() };
    my_message_trait.show();

    let string_message_trait = String::from("std::String for MyMessageTrait");
    string_message_trait.show();

    let msg = Message { text: "    Message for std::fmt::Display".into()};
    println!("{}", msg);
}
