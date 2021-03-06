use std::str::FromStr;

use rusoto_ce::CostExplorerClient;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::{ChainProvider, CredentialsError, ProfileProvider};
use crate::client_error::ClientError;

pub fn get_client(aws_profile: &str, region: &str) -> Result<CostExplorerClient, ClientError> {

    match Region::from_str(region) {
        Ok(aws_region) => {
            if aws_profile.is_empty() {
                ChainProvider::new();
                Ok(CostExplorerClient::new(aws_region ))
            } else {
                let _profile_provider: Result<ProfileProvider, CredentialsError> = ProfileProvider::new();
                match _profile_provider {
                    Ok(mut _prov) => {
                        _prov.set_profile(aws_profile);
                        Ok(CostExplorerClient::new_with(HttpClient::new().expect("failed to create request dispatcher"), _prov, aws_region))
                    },
                    Err(_e) => Err(ClientError::from(_e.message)),
                }
            }
        },
        Err(_e) => Err(ClientError::from(_e.to_string())),
    }
}