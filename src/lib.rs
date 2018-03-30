extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
#[macro_use]
extern crate slog;
extern crate sloggers;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate reqwest;
extern crate tokio_core;

pub mod types;

use std::fmt::Debug;

use futures::prelude::*;
use reqwest::{header, Error, IntoUrl, Url, unstable::async::{Client, Request, Response}};
use serde::{Serialize, de::DeserializeOwned};

use self::types::*;

#[derive(Clone, Debug)]
pub enum Auth {
    None,
    Basic { username: String, password: String },
}

#[derive(Clone)]
pub struct Jira {
    auth: Auth,
    endpoint: Url,
    client: Client,
}

impl Jira {
    pub fn new(auth: Auth, endpoint: Url, handle: &::tokio_core::reactor::Handle) -> Self {
        let client = Client::new(handle);

        Jira {
            auth,
            endpoint,
            client,
        }
    }

    fn exec(&self, mut req: Request) -> impl Future<Item = Response, Error = Error> {
        use self::Auth::*;
        match &self.auth {
            &None => {}
            &Basic {
                ref username,
                ref password,
            } => {
                req.headers_mut().set(header::Authorization(header::Basic {
                    username: username.clone(),
                    password: Some(password.clone()),
                }));
            }
        };

        self.client
            .execute(req)
            .and_then(|res| res.error_for_status())
    }

    fn exec_json<T: DeserializeOwned + Debug>(
        &self,
        req: Request,
    ) -> impl Future<Item = T, Error = Error> {
        self.exec(req)
            .and_then(|mut req| req.json::<T>())
            .and_then(|data| {
                println!("{:#?}", &data);
                ::futures::future::ok(data)
            })
    }

    fn get_json<T, U>(&self, url: U) -> impl Future<Item = T, Error = Error>
    where
        T: DeserializeOwned + Debug,
        U: IntoUrl,
    {
        let req = self.client.get(url.into_url().unwrap()).build().unwrap();
        self.exec_json(req)
    }

    fn delete<U: IntoUrl>(&self, url: U) -> impl Future<Item = (), Error = Error> {
        let req = self.client.delete(url).build().unwrap();
        self.exec(req).and_then(|_| futures::future::ok(()))
    }

    fn application_properties(&self) -> ApplicationProperties {
        ApplicationProperties {
            client: self.clone(),
        }
    }

    fn application_roles(&self) -> ApplicationRoles {
        ApplicationRoles {
            client: self.clone(),
        }
    }

    fn attachments(&self) -> Attachments {
        Attachments {
            client: self.clone(),
        }
    }

    fn auditing(&self) -> Auditing {
        Auditing {
            client: self.clone(),
        }
    }
}

pub struct ApplicationProperties {
    client: Jira,
}

impl ApplicationProperties {
    pub fn list(
        &self,
        filter: ApplicationPropertyFilter,
    ) -> impl Future<Item = Vec<ApplicationProperty>, Error = Error> {
        let mut url = self.client.endpoint.join("application-properties").unwrap();
        if let Some(val) = filter.key {
            url.query_pairs_mut().append_pair("key", &val);
        }
        if let Some(val) = filter.key_filter {
            url.query_pairs_mut().append_pair("keyFilter", &val);
        }
        if let Some(val) = filter.permission_level {
            url.query_pairs_mut().append_pair("permissionLevel", &val);
        }
        self.client.get_json(url)
    }

    pub fn set<T: Serialize>(&self, id: &str, value: T) -> impl Future<Item = (), Error = Error> {
        let url = self.client
            .endpoint
            .join(&format!("application-properties/{}", id))
            .unwrap();
        let req = self.client
            .client
            .put(url)
            .json(&json!({
            "id": id,
            "value": value,
        }))
            .build()
            .unwrap();
        self.client.exec(req).map(|_| ())
    }

    pub fn advanced_settings(&self) -> impl Future<Item = Vec<ApplicationProperty>, Error = Error> {
        let mut url = self.client
            .endpoint
            .join("application-properties/advanced-settings")
            .unwrap();
        self.client.get_json(url)
    }
}

pub struct ApplicationRoles {
    client: Jira,
}

impl ApplicationRoles {
    pub fn list(&self) -> impl Future<Item = Vec<ApplicationRole>, Error = Error> {
        let url = self.client.endpoint.join("applicationrole").unwrap();
        self.client.get_json(url)
    }

    pub fn get(&self, key: &str) -> impl Future<Item = ApplicationRole, Error = Error> {
        let url = self.client
            .endpoint
            .join(&format!("applicationrole/{}", key))
            .unwrap();
        self.client.get_json(url)
    }
}

pub struct Attachments {
    client: Jira,
}

impl Attachments {
    pub fn get(&self, id: u64) -> impl Future<Item = Attachment, Error = Error> {
        let url = self.client
            .endpoint
            .join(&format!("attachment/{}", id))
            .unwrap();
        self.client.get_json(url)
    }

    pub fn delete(&self, id: u64) -> impl Future<Item = (), Error = Error> {
        let url = self.client
            .endpoint
            .join(&format!("attachment/{}", id))
            .unwrap();
        self.client.delete(url)
    }

    pub fn meta(&self) -> impl Future<Item = AttachmentMeta, Error = Error> {
        let url = self.client.endpoint.join("attachment/meta").unwrap();
        self.client.get_json(url)
    }
}

pub struct Auditing {
    client: Jira,
}

impl Auditing {
    fn list(
        &self,
        filter: AuditingFilter,
    ) -> impl Future<Item = PaginatedAuditRecords, Error = Error> {
        let mut url = self.client.endpoint.join("auditing/record").unwrap();
        if let Some(val) = filter.filter.as_ref() {
            url.query_pairs_mut().append_pair("filter", val);
        }
        if let Some(val) = filter.filter.as_ref() {
            url.query_pairs_mut().append_pair("filter", val);
        }
        if let Some(val) = filter.offset.as_ref() {
            url.query_pairs_mut()
                .append_pair("offset", &val.to_string());
        }
        if let Some(val) = filter.from.as_ref() {
            url.query_pairs_mut().append_pair("from", &val.to_string());
        }
        if let Some(val) = filter.to.as_ref() {
            url.query_pairs_mut().append_pair("to", &val.to_string());
        }
        self.client.get_json(url)
    }
}

mod tests {
    use super::*;

    use tokio_core::reactor::Core;

    fn client() -> (Core, Jira) {
        let core = ::tokio_core::reactor::Core::new().unwrap();
        let auth = Auth::Basic {
            username: "christoph.herzog@ready2order.com".into(),
            password: "pcadmin3".into(),
        };
        let url = "https://ready2order.atlassian.net/rest/api/2/"
            .parse()
            .unwrap();
        let client = Jira::new(auth, url, &core.handle());
        (core, client)
    }

    #[test]
    fn test_application_properties() {
        let (mut core, client) = client();
        let f = ApplicationPropertyFilter::default();
        let r = core.run(client.application_properties().list(f)).unwrap();
    }

    #[test]
    fn test_application_property_set() {
        let (mut core, client) = client();
        let r = core.run(
            client
                .application_properties()
                .set("xflow.product.suggestions.enabled", false),
        ).unwrap();
    }

    #[test]
    fn test_application_properties_advanced_settings() {
        let (mut core, client) = client();
        let r = core.run(client.application_properties().advanced_settings())
            .unwrap();
    }

    #[test]
    fn test_application_roles_list() {
        let (mut core, client) = client();
        let r = core.run(client.application_roles().list()).unwrap();
    }

    #[test]
    fn test_application_roles_get() {
        let (mut core, client) = client();
        let r = core.run(client.application_roles().get("jira-software"))
            .unwrap();
    }

    #[test]
    fn test_attachments_meta() {
        let (mut core, client) = client();
        let r = core.run(client.attachments().meta()).unwrap();
    }

    #[test]
    fn test_auditing_list() {
        let (mut core, client) = client();
        let r = core.run(client.auditing().list(AuditingFilter::default()))
            .unwrap();
    }
}
