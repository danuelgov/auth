use new_type::EmailAddress;

#[derive(Deserialize)]
pub struct Data {
    pub email_address: EmailAddress,
}
