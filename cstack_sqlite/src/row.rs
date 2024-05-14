const ID_SIZE: usize = std::mem::size_of::<u32>();
const ID_OFFSET: usize = 0;
const USERNAME_SIZE: usize = 32;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_SIZE: usize = 255;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = ID_SIZE + EMAIL_SIZE + USERNAME_SIZE;

#[derive(Debug)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: u32, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }

    pub fn serialize(&self, dest: &mut [u8]) {
        dest[ID_OFFSET..USERNAME_OFFSET].copy_from_slice(&self.id.to_be_bytes());

        let username_bytes = self.username.as_bytes();
        let username_len = usize::min(username_bytes.len(), USERNAME_SIZE);
        dest[USERNAME_OFFSET..(USERNAME_OFFSET + username_len)]
            .copy_from_slice(&username_bytes[..username_len]);

        let email_bytes = self.email.as_bytes();
        let email_len = usize::min(email_bytes.len(), EMAIL_SIZE);
        dest[EMAIL_OFFSET..(EMAIL_OFFSET + email_len)].copy_from_slice(&email_bytes[..email_len]);
    }

    pub fn deserialize(src: &[u8]) -> Self {
        let id: [u8; 4] = src[ID_OFFSET..ID_SIZE]
            .try_into()
            .expect("could not deserialize id");
        let id = u32::from_be_bytes(id);

        let username = String::from_utf8(src[USERNAME_OFFSET..USERNAME_SIZE].to_vec())
            .expect("could not deserialize username")
            .trim_matches(char::from(0))
            .to_string();
        let email = String::from_utf8(src[EMAIL_OFFSET..EMAIL_SIZE].to_vec())
            .expect("could not deserialize email")
            .trim_matches(char::from(0))
            .to_string();

        Self::new(id, username, email)
    }
}
