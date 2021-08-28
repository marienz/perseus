#![allow(missing_docs)]

pub use error_chain::bail;
use error_chain::error_chain;

// The `error_chain` setup for the whole crate
error_chain! {
    // The custom errors for this crate (very broad)
    errors {
        /// For when executing a prerequisite command fails.
        PrereqFailed(cmd: String, env_var: String, err: String) {
            description("prerequisite command execution failed")
            display("You seem to be missing the prerequisite '{}', which is required for the Perseus CLI to work. If you've installed it at another path, please provide the executable through the '{}' variable. Error was: '{}'.", cmd, env_var, err)
        }
        /// For when the user's curreent directory couldn't be found.
        CurrentDirUnavailable(err: String) {
            description("couldn't get current directory")
            display("Couldn't get your current directory. This is probably an issue with your system configuration. Error was: '{}'.", err)
        }
        /// For when extracting the subcrates failed.
        // The `PathBuf` will be converted to a string, and unwrapping is bad in that context
        ExtractionFailed(target_dir: Option<String>, err: String) {
            description("subcrate extraction failed")
            display("Couldn't extract internal subcrates to '{:?}'. You may not have the permissions necessary to write to this location, or the directory disappeared out from under the CLI. The '.perseus/' directory has been automatically deleted for safety. Error was: '{}'.", target_dir, err)
        }
        /// For when updating the user's .gitignore fails
        GitignoreUpdateFailed(err: String) {
            description("updating gitignore failed")
            display("Couldn't update your .gitignore file to ignore the Perseus subcrates. The '.perseus/' directory has been automatically deleted (necessary further steps not executed). Error was: '{}'.", err)
        }
        /// For when updating relative paths and package names in the manifest failed.
        ManifestUpdateFailed(target: Option<String>, err: String) {
            description("updating manifests failed")
            display("Couldn't update internal manifest file at '{:?}'. If the error persists, make sure you have file write permissions. The '.perseus/' directory has been automatically deleted. Error was: '{}'.", target, err)
        }
        /// For when we can't get the user's `Cargo.toml` file.
        GetUserManifestFailed(err: String) {
            description("reading user manifest failed")
            display("Couldn't read your crate's manifest (Cargo.toml) file. Please make sure this file exists, is valid, and that you're running Perseus in the right directory.The '.perseus/' directory has been automatically deleted. Error was: '{}'.", err)
        }
        /// For when a partially-formed '.perseus/' directory couldn't be removed, but did exist.
        RemoveBadDirFailed(target: Option<String>, err: String) {
            description("removing corrupted '.perseus/' directory failed")
            display("Couldn't remove '.perseus/' directory at '{:?}'. Please remove the '.perseus/' directory manually (particularly if you didn't intentionally run the 'clean' command, that means the directory has been corrupted). Error was: '{}'.", target, err)

        }
    }
}

/// Checks if the given error should cause the CLI to delete the '.perseus/' folder so the user doesn't have something incomplete.
/// When deleting the directory, it should only be deleted if it exists, if not don't worry. If it does and deletion fails, fail like hell.
pub fn err_should_cause_deletion(err: &Error) -> bool {
    matches!(
        err.kind(),
        ErrorKind::ExtractionFailed(_, _) |
        ErrorKind::GitignoreUpdateFailed(_) |
        ErrorKind::ManifestUpdateFailed(_, _)
    )
}
