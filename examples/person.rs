use prost::Message;
use tut_proto::pb::{Person, PhoneNumber, PhoneType};

fn main() {
    let phones = vec![PhoneNumber::new("111-222-3333", PhoneType::Mobile)];
    let person = Person::new("Leon".to_owned(), 1, "yanglyu.leon.7@gmail.com", phones);
    let v1 = person.encode_to_vec();
    let person1 = Person::decode(v1.as_ref()).unwrap();
    assert_eq!(person, person1);
    println!("{person:?}, {v1:?}(len: {})", v1.len());
    let json = serde_json::to_string_pretty(&person1).unwrap();
    println!("{}", json);
}
