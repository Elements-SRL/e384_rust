use tracing::instrument;
use crate::{error_codes::ErrorCodes, r_message_dispatcher::E384Err};


#[instrument]
pub fn translate(err: E384Err) -> Result<(), ErrorCodes> {
    let e: ErrorCodes = err.into();
    e.to_res()
}
