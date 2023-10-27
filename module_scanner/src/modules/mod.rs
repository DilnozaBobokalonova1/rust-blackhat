use crate::Error;
use async_trait::async_trait;
use reqwest::Client;

mod http;
mod subdomains;

pub fn all_http_modules() -> Vec<Box<dyn HttpModule>> {
    return vec![
        Box::new(http::GitConfigDisclosure::new())
    ];
}

pub fn all_subdomains_modules() -> Vec<Box<dyn SubdomainModule>> {
    return vec![Box::new(subdomains::Crtsh::new())];
}

pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}
                                /**                         **/
                                /**        SUBDOMAINS       **/
                                /**                         **/
/**
 * Role: find all subdomains for a given domain and source.
 */
#[async_trait]
pub trait SubdomainModule: Module {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error>;
}

#[derive(Debug, Clone)]
pub struct Subdomain {
    pub domain: String,
    pub open_ports: Vec<Port>,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
    pub findings: Vec<HttpFinding>
}

                                /**                         **/
                                /**           HTTP          **/
                                /**                         **/

/**
 * Role: for a given endpoint, check if a given vulnerability can be found.
 */
#[async_trait]
pub trait HttpModule: Module {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error>;
}

//define an attribute to automatically derive debug and clone traits.
#[derive(Debug, Clone)]
pub enum HttpFinding {
    DsStoreFileDisclosure(String),
    DotEnvFileDisclosure(String),
    DirectoryListingDisclosure(String),
    TraefikDashboardUnauthenticatedAccess(String),
    PrometheusDshboardUnauthenticatedAccess(String),
    KibanaUnauthenticatedAccess(String),
    GitlabOpenResgitrations(String),
    GitHeadDisclosure(String),
    GitDirectoryDisclosure(String),
    GitConfigDisclosure(String),
    EtcUnauthenticatedAccess(String),
    Cve2017_9506(String),
    Cve2018_7600(String),
    ElasticsearchUnauthenticatedAccess(String),
}

