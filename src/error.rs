//! Errors for `irc` crate using `error_chain`.

#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Tls(::native_tls::Error);
        Recv(::std::sync::mpsc::RecvError);
        SendMessage(::futures::sync::mpsc::SendError<::proto::Message>);
        OneShotCancelled(::futures::sync::oneshot::Canceled);
    }

    errors {
        /// A parsing error for empty strings as messages.
        ParseEmpty {
            description("Cannot parse an empty string as a message.")
            display("Cannot parse an empty string as a message.")
        }

        /// A parsing error for invalid or missing commands in messages.
        InvalidCommand {
            description("Message contained a missing or invalid Command.")
            display("Message contained a missing or invalid Command.")
        }

        /// A parsing error for failures in subcommand parsing (e.g. CAP and metadata).
        SubCommandParsingFailed {
            description("Failed to parse an IRC subcommand.")
            display("Failed to parse an IRC subcommand.")
        }

        /// An error occurred on one of the internal channels of the `IrcServer`.
        ChannelError {
            description("An error occured on one of the IrcServer's internal channels.")
            display("An error occured on one of the IrcServer's internal channels.")
        }
    }
}
