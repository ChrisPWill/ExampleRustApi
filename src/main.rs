#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/speak/<value>")]
fn speak(value: u32) -> Option<String> {
    robot_speak(value)
}

fn main() {
    rocket::ignite().mount("/v0/", routes![speak]).launch();
}

/// Produces commentary on the bleakness of robotic existence, all wrapped
/// nicely as an `Option<String>`.
///
/// # Examples
///
/// ```
/// let purpose = robot_speak(42);
/// println!("{}", purpose);
/// ```
pub fn robot_speak(x: u32) -> Option<String> {
    match x {
        42 => Some("What is my purpose?".to_string()),
        100...200 => Some("Beep boop".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use robot_speak;

    #[test]
    fn test_purpose() {
        assert_eq!(robot_speak(42), Some("What is my purpose?".to_string()));
    }

    #[test]
    fn test_boop() {
        assert_eq!(robot_speak(150), Some("Beep boop".to_string()));
    }

    #[test]
    fn test_other() {
        assert_eq!(robot_speak(343), None);
    }
}
