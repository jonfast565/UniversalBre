extern crate uuid;
use self::uuid::Uuid;

#[derive(Debug)]
pub struct Pair(pub String, pub String);

pub fn get_new_uuid() -> String {
	return format!("{}", Uuid::new_v4());
}