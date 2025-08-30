use crate::stream;

pub trait AsErrorCodeExt: stream::AsErrorCode {
    fn as_error_code_value(&self) -> Option<u32>;

    fn is_error_code(&self, expected_code: u32) -> bool;

    fn is_closed(&self) -> bool;
}

impl<T> AsErrorCodeExt for T
where
    T: ?Sized + stream::AsErrorCode,
{
    fn as_error_code_value(&self) -> Option<u32> {
        let error_code = self.as_error_code()?;
        let error_code = error_code.ok()?;
        Some(error_code)
    }

    fn is_error_code(&self, expected_error_code: u32) -> bool {
        let Some(error_code) = self.as_error_code_value() else {
            return false;
        };

        error_code == expected_error_code
    }

    fn is_closed(&self) -> bool {
        self.is_error_code(0)
    }
}
