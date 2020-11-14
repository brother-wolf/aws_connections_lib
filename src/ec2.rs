use std::str::FromStr;

use rusoto_core::{HttpClient, Region};
use rusoto_credential::{ChainProvider, CredentialsError, ProfileProvider};
use rusoto_ec2::Ec2Client;
use crate::client_error::ClientError;

pub fn get_client(aws_profile: &str, region: &str) -> Result<Ec2Client, ClientError> {

    match Region::from_str(region) {
        Ok(aws_region) => {
            if aws_profile.is_empty() {
                ChainProvider::new();
                Ok(Ec2Client::new(aws_region))
            } else {
                let _profile_provider: Result<ProfileProvider, CredentialsError> = ProfileProvider::new();
                match _profile_provider {
                    Ok(mut _prov) => {
                        _prov.set_profile(aws_profile);
                        Ok(Ec2Client::new_with(HttpClient::new().expect("failed to create request dispatcher"), _prov, aws_region))
                    },
                    Err(_e) => Err(ClientError::from(_e.message)),
                }
            }
        },
        Err(_e) => Err(ClientError::from(_e.to_string())),
    }
}