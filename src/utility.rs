extern crate uuid;
use self::uuid::Uuid;

pub fn get_new_uuid() -> String {
	return format!("{}", Uuid::new_v4());
}