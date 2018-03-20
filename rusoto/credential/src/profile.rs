//! The Credentials Provider for Credentials stored in a profile inside of a Credentials file.

use std::collections::HashMap;
use std::env::{home_dir};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use futures::{Future, Poll};
use futures::future::{FutureResult, result};
use regex::Regex;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, non_empty_env_var};

const AWS_PROFILE: &str = "AWS_PROFILE";
const AWS_SHARED_CREDENTIALS_FILE: &str = "AWS_SHARED_CREDENTIALS_FILE";
const DEFAULT: &str = "default";

/// Provides AWS credentials from a profile in a credentials file.
#[derive(Clone, Debug)]
pub struct ProfileProvider {
    /// The File Path the Credentials File is located at.
    file_path: PathBuf,
    /// The Profile Path to parse out of the Credentials File.
    profile: String,
}

impl ProfileProvider {

    /// Create a new `ProfileProvider` for the default credentials file path and profile name.
    pub fn new() -> Result<ProfileProvider, CredentialsError> {
        let profile_location = ProfileProvider::default_profile_location()?;
        Ok(ProfileProvider::with_default_configuration(profile_location))
    }

    /// Create a new `ProfileProvider` for the credentials file at the given path, using
    /// the given profile.
    pub fn with_configuration<F, P>(file_path: F, profile: P) -> ProfileProvider
    where
        F: Into<PathBuf>,
        P: Into<String>,
    {
        ProfileProvider {
            file_path: file_path.into(),
            profile: profile.into(),
        }
    }

    /// Create a new `ProfileProvider` for the credentials file at the given path, using
    /// the profile name from environment variable ```AWS_PROFILE``` or fall-back to ```"default"```
    /// if ```AWS_PROFILE``` is not set.
    pub fn with_default_configuration<F>(file_path: F) -> ProfileProvider
    where
        F: Into<PathBuf>
    {
        ProfileProvider::with_configuration(file_path, ProfileProvider::default_profile_name())
    }

    /// Default credentials file location:
    /// 1. if set and not empty, use value from environment variable ```AWS_SHARED_CREDENTIALS_FILE```
    /// 2. otherwise return `~/.aws/credentials` (Linux/Mac) resp. `%USERPROFILE%\.aws\credentials` (Windows)
    fn default_profile_location() -> Result<PathBuf, CredentialsError> {
        let env = non_empty_env_var(AWS_SHARED_CREDENTIALS_FILE);
        match env {
            Some(path) => Ok(PathBuf::from(path)),
            None => ProfileProvider::hardcoded_profile_location(),
        }
    }

    fn hardcoded_profile_location() -> Result<PathBuf, CredentialsError> {
        match home_dir() {
            Some(mut home_path) => {
                home_path.push(".aws");
                home_path.push("credentials");
                Ok(home_path)
            }
            None => Err(CredentialsError::new(
                "The environment variable HOME must be set.",
            )),
        }
    }

    /// Get the default profile name:
    /// 1. if set and not empty, use value from environment variable ```AWS_PROFILE```
    /// 2. otherwise return ```"default"```
    /// see https://docs.aws.amazon.com/sdk-for-java/v1/developer-guide/credentials.html.
    fn default_profile_name() -> String {
        non_empty_env_var(AWS_PROFILE).unwrap_or_else(|| DEFAULT.to_owned())
    }

    /// Get a reference to the credentials file path.
    pub fn file_path(&self) -> &Path {
        self.file_path.as_ref()
    }

    /// Get a reference to the profile name.
    pub fn profile(&self) -> &str {
        &self.profile
    }

    /// Set the credentials file path.
    pub fn set_file_path<F>(&mut self, file_path: F)
    where
        F: Into<PathBuf>,
    {
        self.file_path = file_path.into();
    }

    /// Set the profile name.
    pub fn set_profile<P>(&mut self, profile: P)
    where
        P: Into<String>,
    {
        self.profile = profile.into();
    }
}

/// Provides AWS credentials from a profile in a credentials file as a Future.
pub struct ProfileProviderFuture {
    inner: FutureResult<AwsCredentials, CredentialsError>
}

impl Future for ProfileProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

impl ProvideAwsCredentials for ProfileProvider {
    type Future = ProfileProviderFuture;

    fn credentials(&self) -> Self::Future {
        let inner = result(parse_credentials_file(self.file_path()).and_then(|mut profiles| {
            profiles.remove(self.profile()).ok_or_else(|| {
                CredentialsError::new("profile not found")
            })
        }));

        ProfileProviderFuture { inner: inner }
    }
}

/// Parses a Credentials file into a Map of <`ProfileName`, `AwsCredentials`>
fn parse_credentials_file(
    file_path: &Path,
) -> Result<HashMap<String, AwsCredentials>, CredentialsError> {
    match fs::metadata(file_path) {
        Err(_) => {
            return Err(CredentialsError::new(format!(
                "Couldn't stat credentials file: [ {:?} ]. Non existant, or no permission.",
                file_path
            )))
        }
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(CredentialsError::new(format!(
                    "Credentials file: [ {:?} ] is not a file.",
                    file_path
                )));
            }
        }
    };

    let file = try!(File::open(file_path));

    let profile_regex = Regex::new(r"^\[([^\]]+)\]$").expect("Failed to compile regex");
    let mut profiles: HashMap<String, AwsCredentials> = HashMap::new();
    let mut access_key: Option<String> = None;
    let mut secret_key: Option<String> = None;
    let mut token: Option<String> = None;
    let mut profile_name: Option<String> = None;

    let file_lines = BufReader::new(&file);
    for (line_no, line) in file_lines.lines().enumerate() {
        let unwrapped_line: String = line.expect(&format!(
            "Failed to read credentials file, line: {}",
            line_no
        ));

        // skip empty lines
        if unwrapped_line.is_empty() {
            continue;
        }

        // skip comments
        if unwrapped_line.starts_with('#') {
            continue;
        }

        // handle the opening of named profile blocks
        if profile_regex.is_match(&unwrapped_line) {
            if profile_name.is_some() && access_key.is_some() && secret_key.is_some() {
                let creds = AwsCredentials::new(
                    access_key.unwrap(),
                    secret_key.unwrap(),
                    token,
                    None,
                );
                profiles.insert(profile_name.unwrap(), creds);
            }

            access_key = None;
            secret_key = None;
            token = None;

            let caps = profile_regex.captures(&unwrapped_line).unwrap();
            profile_name = Some(caps.get(1).unwrap().as_str().to_string());
            continue;
        }

        // otherwise look for key=value pairs we care about
        let lower_case_line = unwrapped_line.to_ascii_lowercase().to_string();

        if lower_case_line.contains("aws_access_key_id") && access_key.is_none() {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                access_key = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_secret_access_key") && secret_key.is_none() {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                secret_key = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_session_token") && token.is_none() {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                token = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_security_token") {
            if token.is_none() {
                let v: Vec<&str> = unwrapped_line.split('=').collect();
                if !v.is_empty() {
                    token = Some(v[1].trim_matches(' ').to_string());
                }
            }
        } else {
            // Ignore unrecognized fields
            continue;
        }

    }

    if profile_name.is_some() && access_key.is_some() && secret_key.is_some() {
        let creds = AwsCredentials::new(
            access_key.unwrap(),
            secret_key.unwrap(),
            token,
            None,
        );
        profiles.insert(profile_name.unwrap(), creds);
    }

    if profiles.is_empty() {
        return Err(CredentialsError::new("No credentials found."));
    }

    Ok(profiles)
}

#[cfg(test)]
mod tests {

    use std::env;
    use std::path::Path;

    use {CredentialsError, ProvideAwsCredentials};
    use std::sync::{Mutex, MutexGuard};
    use super::*;

    // cargo runs tests in parallel, which leads to race conditions when changing
    // environment variables. Therefore we use a global mutex for all tests which
    // rely on environment variables.
    lazy_static! {
        static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
    }

    // As failed (panic) tests will poisen the global mutex, we use a helper which
    // recovers from poisoned mutex.
    fn lock<'a, T>(mutex: &'a Mutex<T>) -> MutexGuard<'a,T> {
        match mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[test]
    fn parse_credentials_file_default_profile() {
        let result = super::parse_credentials_file(
            Path::new("tests/sample-data/default_profile_credentials"),
        );
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles.get(DEFAULT).expect(
            "No Default profile in default_profile_credentials",
        );
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

    #[test]
    fn parse_credentials_file_multiple_profiles() {
        let result = super::parse_credentials_file(
            Path::new("tests/sample-data/multiple_profile_credentials"),
        );
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 2);

        let foo_profile = profiles.get("foo").expect(
            "No foo profile in multiple_profile_credentials",
        );
        assert_eq!(foo_profile.aws_access_key_id(), "foo_access_key");
        assert_eq!(foo_profile.aws_secret_access_key(), "foo_secret_key");

        let bar_profile = profiles.get("bar").expect(
            "No bar profile in multiple_profile_credentials",
        );
        assert_eq!(bar_profile.aws_access_key_id(), "bar_access_key");
        assert_eq!(bar_profile.aws_secret_access_key(), "bar_secret_key");
    }

    #[test]
    fn parse_all_values_credentials_file() {
        let result =
            super::parse_credentials_file(Path::new("tests/sample-data/full_profile_credentials"));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles.get(DEFAULT).expect(
            "No default profile in full_profile_credentials",
        );
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

    #[test]
    fn profile_provider_happy_path() {
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foo",
        );
        let result = provider.credentials().wait();

        assert!(result.is_ok());

        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "foo_access_key");
        assert_eq!(creds.aws_secret_access_key(), "foo_secret_key");
    }

    #[test]
    fn profile_provider_via_environment_variable() {
        let _guard = lock(&ENV_MUTEX);
        let credentials_path = "tests/sample-data/default_profile_credentials";
        env::set_var(AWS_SHARED_CREDENTIALS_FILE, credentials_path);
        let result = ProfileProvider::new();
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.file_path().to_str().unwrap(), credentials_path);
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
    }

    #[test]
    fn profile_provider_profile_name_via_environment_variable() {
        let _guard = lock(&ENV_MUTEX);
        let credentials_path = "tests/sample-data/multiple_profile_credentials";
        env::set_var(AWS_SHARED_CREDENTIALS_FILE, credentials_path);
        env::set_var(AWS_PROFILE, "bar");
        let result = ProfileProvider::new();
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.file_path().to_str().unwrap(), credentials_path);
        let creds = provider.credentials().wait();
        assert_eq!(creds.unwrap().aws_access_key_id(), "bar_access_key");
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
        env::remove_var(AWS_PROFILE);
    } 

    #[test]
    fn profile_provider_bad_profile() {
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "not_a_profile",
        );
        let result = provider.credentials().wait();

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(CredentialsError::new("profile not found"))
        );
    }

    #[test]
    fn profile_provider_profile_name() {
        let _guard = lock(&ENV_MUTEX);
        let mut provider = ProfileProvider::new().unwrap();
        assert_eq!(DEFAULT, provider.profile());
        provider.set_profile("foo");
        assert_eq!("foo", provider.profile());
    }

    #[test]
    fn existing_file_no_credentials() {
        let result = super::parse_credentials_file(Path::new("tests/sample-data/no_credentials"));
        assert_eq!(
            result.err(),
            Some(CredentialsError::new("No credentials found."))
        )
    }

    #[test]
    fn parse_credentials_bad_path() {
        let result = super::parse_credentials_file(Path::new("/bad/file/path"));
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(
                "Couldn\'t stat credentials file: [ \"/bad/file/path\" ]. Non existant, or no permission.",
            ))
        );
    }

    #[test]
    fn parse_credentials_directory_path() {
        let result = super::parse_credentials_file(Path::new("tests/"));
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(
                "Credentials file: [ \"tests/\" ] is not a file.",
            ))
        );
    }

    #[test]
    fn parse_credentials_unrecognized_field() {
        let result = super::parse_credentials_file(Path::new(
            "tests/sample-data/unrecognized_field_profile_credentials",
        ));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles.get(DEFAULT).expect(
            "No default profile in full_profile_credentials",
        );
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

    #[test]
    fn default_profile_name_from_env_var(){
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_PROFILE, "bar");
        assert_eq!("bar", ProfileProvider::default_profile_name());
        env::remove_var(AWS_PROFILE);
    }

    #[test]
    fn default_profile_name_from_empty_env_var(){
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_PROFILE, "");
        assert_eq!(DEFAULT, ProfileProvider::default_profile_name());
        env::remove_var(AWS_PROFILE);
    }

    #[test]
    fn default_profile_name(){
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_PROFILE);
        assert_eq!(DEFAULT, ProfileProvider::default_profile_name());
    }

    #[test]
    fn default_profile_location_from_env_var(){
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_SHARED_CREDENTIALS_FILE, "bar");
        assert_eq!(Ok(PathBuf::from("bar")), ProfileProvider::default_profile_location());
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
    }

    #[test]
    fn default_profile_location_from_empty_env_var(){
        let _guard = lock(&ENV_MUTEX);
        env::set_var(AWS_SHARED_CREDENTIALS_FILE, "");
        assert_eq!(ProfileProvider::hardcoded_profile_location(), ProfileProvider::default_profile_location());
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
    }

    #[test]
    fn default_profile_location(){
        let _guard = lock(&ENV_MUTEX);
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
        assert_eq!(ProfileProvider::hardcoded_profile_location(), ProfileProvider::default_profile_location());
    }

}
