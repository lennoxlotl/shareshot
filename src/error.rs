/// An error which occurred while the application is running.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// D-Bus creation error
    ///
    /// Unable to create D-Bus service with provided parameters.
    /// Might be caused due to shareshot already being started.
    #[error("failed to create dbus service: {0}")]
    DbusCreate(Box<dyn std::error::Error>),
    /// D-Bus error
    ///
    /// Unable to send message to D-Bus daemon.
    /// Either in Sandboxed environment or system does not have D-Bus.
    #[error("failed to create dbus connection: {0}")]
    Dbus(#[from] zbus::Error),
    /// XDG Desktop Portal error
    ///
    /// Unable to send request to XDG Desktop Portal.
    /// System might not have a Portal (very rare)?
    #[error("failed to send portal request: {0}")]
    Portal(#[from] ashpd::Error),
    /// Invalid Screenshot error
    ///
    /// The screenshot taken by the portal is invalid.
    /// It either doesn't exist or is malformed.
    #[error("failed to read screenshot file")]
    InvalidScreenshotFile,
    /// IO error
    ///
    /// Failed to write or read file on system drive.
    #[error("failed to execute io operation: {0}")]
    IO(#[from] std::io::Error),
    /// URI codec error
    ///
    /// The URI provided by the XDG Desktop Portal has an invalid format.
    /// Most likely related to Portal implementation.
    #[error("failed to encode/decode uri")]
    UriCodec,
    /// Mime resolve error
    ///
    /// Unable to decode mime type based off of file extension.
    /// The file most likely has an invalid or unknown file extension.
    /// Related to how the Portal stores Screenshots on the disk.
    #[error("failed to resolve mime type of screenshot file")]
    MimeResolve,
    /// Config load error
    ///
    /// Configuration parseing failed.
    /// Config on drive is most likely outdated or contains invalid properties.
    #[error("failed to load configuration file")]
    ConfigLoad,
    /// Invalid response error
    ///
    /// Invalid response received from server.
    /// The response was not as expected by configuration, double check your configuration.
    #[error("cannot parse response, is the url parser configured properly? ({0})")]
    InvalidResponse(String),
    /// Request failed error
    ///
    /// Reqwest failed to make the request to the configurated endpoint.
    /// Check your upload server of choice or configuration file.
    #[error("failed to make http request ({0})")]
    RequestFailed(#[from] reqwest::Error),
    /// Non ok status code error
    ///
    /// The upload failed with a non 200-209 response.
    /// Make sure the upload server properties are configurated properly.
    #[error("server responded with non 200 status code: {0} ({1})")]
    NonOkStatusCode(String, String),
}
