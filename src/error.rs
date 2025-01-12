use std::string::FromUtf8Error;

/// An error which occurred while the application is running.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// D-Bus creation error
    ///
    /// Unable to create D-Bus service with provided parameters.
    /// Might be caused due to shareshot already being started.
    #[error("Failed to create dbus service: {0}")]
    DbusCreate(Box<dyn std::error::Error>),
    /// D-Bus error
    ///
    /// Unable to send message to D-Bus daemon.
    /// Either in Sandboxed environment or system does not have D-Bus.
    #[error("Failed to create dbus connection: {0}")]
    Dbus(#[from] zbus::Error),
    /// XDG Desktop Portal error
    ///
    /// Unable to send request to XDG Desktop Portal.
    /// System might not have a Portal (very rare)?
    #[error("Failed to send portal request: {0}")]
    Portal(#[from] ashpd::Error),
    /// Invalid Screenshot error
    ///
    /// The screenshot taken by the portal is invalid.
    /// It either doesn't exist or is malformed.
    #[error("Failed to read screenshot file")]
    InvalidScreenshotFile,
    /// IO error
    ///
    /// Failed to write or read file on system drive.
    #[error("Failed to execute io operation: {0}")]
    IO(#[from] std::io::Error),
    /// URI codec error
    ///
    /// The URI provided by the XDG Desktop Portal has an invalid format.
    /// Most likely related to Portal implementation.
    #[error("Failed to encode/decode uri: {0}")]
    UriCodec(#[from] FromUtf8Error),
    /// Mime resolve error
    ///
    /// Unable to decode mime type based off of file extension.
    /// The file most likely has an invalid or unknown file extension.
    /// Related to how the Portal stores Screenshots on the disk.
    #[error("Failed to resolve mime type of screenshot file")]
    MimeResolve,
    /// Config load error
    ///
    /// Configuration parseing failed.
    /// Config on drive is most likely outdated or contains invalid properties.
    #[error("Failed to load configuration file")]
    ConfigLoad,
    /// Config save error
    ///
    /// Configuration saving failed.
    /// The configuration folder was most likely not created properly or the user doesn't have permission to edit it.
    #[error("Failed to save configuration file")]
    ConfigSave,
    /// Invalid response error
    ///
    /// Invalid response received from server.
    /// The response was not as expected by configuration, double check your configuration.
    #[error("Cannot parse response, is the url parser configured properly? ({0})")]
    InvalidResponse(String),
    /// Request failed error
    ///
    /// Reqwest failed to make the request to the configurated endpoint.
    /// Check your upload server of choice or configuration file.
    #[error("Failed to make http request ({0})")]
    RequestFailed(#[from] reqwest::Error),
    /// Non ok status code error
    ///
    /// The upload failed with a non 200-209 response.
    /// Make sure the upload server properties are configurated properly.
    #[error("Server responded with non 200 status code: {0} ({1})")]
    NonOkStatusCode(String, String),
    /// Image not found error
    ///
    /// Could not read image file after screenshot was taken by portal.
    /// Did the currently used portal implement the screenshot protocol properly?
    #[error("Image not found, was the screenshot taken?")]
    ImageNotFound,
}
