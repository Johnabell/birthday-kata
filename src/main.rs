use chrono::NaiveDate;
use mockall::automock;

fn main() {
    println!("Hello, world!");
}

fn send_birthday_emails(friends: Vec<Friend>, sender: impl Sender) -> Result<(), ()>{
    todo!()
}

fn is_birthday_today() -> bool {
    todo!()
}

struct Friend {
    first_name: String,
    last_name: String,
    date_of_birth: NaiveDate,
    email: String,
}

#[automock]
trait Sender {
    fn send(email: &str, content: &str, subject: &str) -> Result<(), ()>;
}

struct SenderImpl;

impl Sender for SenderImpl {
    fn send(email: &str, content: &str, subject: &str) -> Result<(), ()> {
        todo!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn is_single_friend_birthday() {
        let friends = vec![
            Friend {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1982, 12, 20).unwrap(),
                email: "john.doe@foobar.com".to_string()
            }
        ];
        let mut mock_sender = MockSender::new();

        let result = send_birthday_emails(friends, mock_sender);

        assert!(result.is_ok())

    }

    #[test]
    fn is_birthday_today() {
        
    }

    
}
