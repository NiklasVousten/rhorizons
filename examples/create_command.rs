use rhorizons::*;

fn main() {
    let t = command::CommandBuilder::from_id(10);
    let c = t.build_with_type(command::CommandType::MajorBody).unwrap();
}
