use error_chain::error_chain;

error_chain! {
    errors {
       RestError(response: String)
    }
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        Json(serde_json::Error);
        TimestampError(std::time::SystemTimeError);
    }
}
