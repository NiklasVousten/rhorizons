use rhorizons::*;

fn main() {
    let t = command_old::CommandBuilder::from_id(10);
    let c: Box<dyn command_old::QueryCommand + Send + Sync> = t
        .build_with_type(command_old::CommandType::MajorBody)
        .unwrap();
}
