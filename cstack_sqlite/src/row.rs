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

pub enum RowSerializationError {
    StringTooLong { field: String },
}

impl Row {
    pub fn new(id: u32, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }

    fn serialize_string_column(
        column_name: &str,
        column_value: &str,
        column_size: usize,
        offset: usize,
        dest: &mut [u8],
    ) -> Result<(), RowSerializationError> {
        let mut column_buf = vec![0; column_size];
        let column_value_len = column_value.as_bytes().len();
        if column_value_len > column_size {
            return Err(RowSerializationError::StringTooLong {
                field: column_name.to_string(),
            });
        }
        let (msbytes, lsbytes) = column_buf.split_at_mut(column_value_len);
        msbytes.copy_from_slice(column_value.as_bytes());

        let column_buf = [msbytes, lsbytes].concat();
        dest[offset..(offset + column_size)].copy_from_slice(&column_buf[..]);
        Ok(())
    }

    fn serialize_integer_column(
        column_value: u32,
        column_size: usize,
        offset: usize,
        dest: &mut [u8],
    ) {
        dest[offset..(offset + column_size)].copy_from_slice(&column_value.to_be_bytes());
    }

    pub fn serialize(&mut self, dest: &mut [u8]) -> Result<(), RowSerializationError> {
        Self::serialize_integer_column(self.id, ID_SIZE, ID_OFFSET, dest);
        Self::serialize_string_column(
            "username",
            &self.username,
            USERNAME_SIZE,
            USERNAME_OFFSET,
            dest,
        )?;
        Self::serialize_string_column("email", &self.email, EMAIL_SIZE, EMAIL_OFFSET, dest)
    }

    pub fn deserialize(src: &[u8]) -> Self {
        // TODO: handle extra null bytes better
        let id: [u8; 4] = src[ID_OFFSET..ID_SIZE]
            .try_into()
            .expect("could not deserialize id");
        let id = u32::from_be_bytes(id);

        let username =
            String::from_utf8(src[USERNAME_OFFSET..(USERNAME_OFFSET + USERNAME_SIZE)].to_vec())
                .expect("could not deserialize username")
                .trim_matches(char::from(0))
                .to_string();
        let email = String::from_utf8(src[EMAIL_OFFSET..(EMAIL_OFFSET + EMAIL_SIZE)].to_vec())
            .expect("could not deserialize email")
            .trim_matches(char::from(0))
            .to_string();

        Self::new(id, username, email)
    }
}
