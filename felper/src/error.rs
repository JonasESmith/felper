/// Represents all possible errors that can occur in the application.
///
/// This enum uses the `thiserror` crate to derive the `Error` trait and
/// provide custom error messages for each variant.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Represents a generic error with a custom message.
    ///
    /// The `#[error("Generic{0}")]` attribute generates an error message
    /// that includes the string provided in the variant.
    ///
    /// # Example
    /// ```
    /// let error = Error::Generic("Something went wrong".to_string());
    /// assert_eq!(error.to_string(), "GenericSomething went wrong");
    /// ```
    // #[error("Generic{0}")]
    // Generic(String),

    /// Represents an I/O error.
    ///
    /// The `#[error(transparent)]` attribute makes this variant delegate
    /// its `Display` implementation to the inner `std::io::Error`.
    ///
    /// The `#[from]` attribute automatically implements `From<std::io::Error>`
    /// for this enum, allowing easy conversion from `std::io::Error` to `Error`.
    ///
    /// # Example
    /// ```
    /// use std::fs::File;
    /// let io_error = File::open("non_existent_file.txt").unwrap_err();
    /// let error: Error = io_error.into(); // Automatic conversion
    /// ```
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// Represents an error that occurred during Mason operations.
    ///
    /// The `#[error("Mason")]` attribute provides a static error message,
    /// while the `String` field allows for additional context.
    ///
    /// # Example
    /// ```
    /// let error = Error::MasonError("Failed to initialize Mason".to_string());
    /// assert_eq!(error.to_string(), "Mason");
    /// ```
    #[error("Mason")]
    MasonError(String),

    /// Represents an error that occurred during the build runner process.
    ///
    /// Similar to `MasonError`, this variant has a static error message
    /// and a `String` field for additional details.
    ///
    /// # Example
    /// ```
    /// let error = Error::BuildRunnerError("Build failed".to_string());
    /// assert_eq!(error.to_string(), "BuildRunner");
    /// ```
    #[error("BuildRunner")]
    BuildRunnerError(String),
}
