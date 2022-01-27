//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.1
//!

#![allow(dead_code)]
#![allow(unused_imports)]
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ExecuteRequest")]
    pub struct ExecuteRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::Execute,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AicServiceFault")]
    pub struct AicServiceFault {
        #[yaserde(flatten, default)]
        pub fault: types::Fault,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ExecuteResponse")]
    pub struct ExecuteResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::ExecuteResponse,
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Execute",
        namespace = "tns: http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Execute {
        #[yaserde(rename = "flowName", prefix = "tns", default)]
        pub flow_name: String,
        #[yaserde(rename = "input", prefix = "tns", default)]
        pub input: Vec<AicCouple>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AicCouple",
        namespace = "tns: http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AicCouple {
        #[yaserde(rename = "name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "value", prefix = "tns", default)]
        pub value: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ExecuteResponse",
        namespace = "tns: http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ExecuteResponse {
        #[yaserde(rename = "ExecuteReturn", prefix = "tns", default)]
        pub execute_return: Vec<AicCouple>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AicServiceFault",
        namespace = "tns: http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AicServiceFault {}
    pub type Fault = AicServiceFault;
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Fault",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SoapAicServiceFault {
        #[yaserde(rename = "faultcode", default)]
        pub fault_code: Option<String>,
        #[yaserde(rename = "faultstring", default)]
        pub fault_string: Option<String>,
        #[yaserde(rename = "AicServiceFault", default)]
        pub detail: Option<AicServiceFault>,
    }
    pub type ExecuteRequest = messages::ExecuteRequest;

    pub type ExecuteResponse = messages::ExecuteResponse;

    pub type AicServiceFault = messages::AicServiceFault;

    #[async_trait]
    pub trait AicWorkflow {
        async fn execute(
            &self,
            execute_request: ExecuteRequest,
        ) -> Result<ExecuteResponse, Option<SoapAicServiceFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl AicWorkflowSoapBinding {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapExecuteRequest {
        #[yaserde(rename = "Execute", default)]
        pub body: ports::ExecuteRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ExecuteRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapExecuteRequest,
    }

    impl ExecuteRequestSoapEnvelope {
        pub fn new(body: SoapExecuteRequest) -> Self {
            ExecuteRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some(
                    "http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapExecuteResponse {
        #[yaserde(rename = "ExecuteResponse", default)]
        pub body: ports::ExecuteResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ExecuteResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapExecuteResponse,
    }

    impl ExecuteResponseSoapEnvelope {
        pub fn new(body: SoapExecuteResponse) -> Self {
            ExecuteResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some(
                    "http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for AicWorkflowSoapBinding {
        fn default() -> Self {
            AicWorkflowSoapBinding {
                client: reqwest::Client::new(),
                url: "http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl AicWorkflowSoapBinding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            AicWorkflowSoapBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct AicWorkflowSoapBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::AicWorkflow for AicWorkflowSoapBinding {
        async fn execute(
            &self,
            execute_request: ports::ExecuteRequest,
        ) -> Result<ports::ExecuteResponse, Option<ports::SoapAicServiceFault>> {
            let __request = ExecuteRequestSoapEnvelope::new(SoapExecuteRequest {
                body: execute_request,
                xmlns: Option::Some(
                    "http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: ExecuteResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    /**
    Service to invoke Workflow in Avaya Interaction Center
     */
    pub struct AicWorkflowService {}
    impl AicWorkflowService {
        pub fn new_client(
            credentials: Option<(String, String)>,
        ) -> bindings::AicWorkflowSoapBinding {
            bindings::AicWorkflowSoapBinding::new(
                "http://aiccore.avayacloud.com:9800/webservices/services/AicWorkflow",
                credentials,
            )
        }
    }
}
