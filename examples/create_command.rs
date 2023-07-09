use rhorizons::*;

fn main() {
    let t = command::CommandBuilder::from_id(10);
    let c = t.with_type(command::CommandType::MajorBody).unwrap();
}
