use mechanics_dns::ResponseCode;
use philharmonic_connector_impl_api::ImplementationError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub(crate) enum Error {
    #[error("{0}")]
    InvalidConfig(String),

    #[error("{0}")]
    InvalidRequest(String),

    #[error("upstream DNS status {status}: {body}")]
    DnsRcode { status: u16, body: String },

    #[error("{0}")]
    Internal(String),
}

impl Error {
    pub(crate) fn from_resolver(error: mechanics_dns::Error) -> Self {
        match &error {
            mechanics_dns::Error::Lookup {
                name,
                record_type,
                response_code: Some(response_code),
                ..
            } => {
                if let Some(name_for_wire) = response_code_name(*response_code) {
                    return Self::DnsRcode {
                        status: u16::from(*response_code),
                        body: format!("{name_for_wire}: {name} {record_type}"),
                    };
                }
                Self::Internal(error.to_string())
            }
            _ => Self::Internal(error.to_string()),
        }
    }
}

impl From<Error> for ImplementationError {
    fn from(value: Error) -> Self {
        match value {
            Error::InvalidConfig(detail) => ImplementationError::InvalidConfig { detail },
            Error::InvalidRequest(detail) => ImplementationError::InvalidRequest { detail },
            Error::DnsRcode { status, body } => ImplementationError::UpstreamError { status, body },
            Error::Internal(detail) => ImplementationError::Internal { detail },
        }
    }
}

fn response_code_name(response_code: ResponseCode) -> Option<&'static str> {
    match response_code {
        ResponseCode::NXDomain => Some("NXDOMAIN"),
        ResponseCode::ServFail => Some("SERVFAIL"),
        ResponseCode::NotImp => Some("NOTIMP"),
        ResponseCode::Refused => Some("REFUSED"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolver_rcodes_map_to_upstream_error_statuses() {
        let cases = [
            (ResponseCode::NXDomain, 3, "NXDOMAIN"),
            (ResponseCode::ServFail, 2, "SERVFAIL"),
            (ResponseCode::NotImp, 4, "NOTIMP"),
            (ResponseCode::Refused, 5, "REFUSED"),
        ];

        for (response_code, status, label) in cases {
            let error = Error::from_resolver(mechanics_dns::Error::Lookup {
                name: "missing.example".to_owned(),
                record_type: "A".to_owned(),
                response_code: Some(response_code),
                message: "rcode".to_owned(),
            });

            assert_eq!(
                ImplementationError::from(error),
                ImplementationError::UpstreamError {
                    status,
                    body: format!("{label}: missing.example A"),
                }
            );
        }
    }
}
