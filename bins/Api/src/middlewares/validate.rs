use commons::error::CustomErrors;

use tracing::error;
use validator::Validate;

pub fn validate_input<T: Validate>(input: &T) -> Result<(), CustomErrors> {
    match input.validate() {
        Ok(_) => return Ok(()),
        Err(message) => {
            error!("invalid request body");
            return Err(CustomErrors::ValidationError(message.to_string()));
        }
    };
}
