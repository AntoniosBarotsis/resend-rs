use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{CreateContactRequest, CreateContactResponse};
use crate::types::{GetContactResponse, ListContactsResponse};
use crate::types::{UpdateContactRequest, UpdateContactResponse};
use crate::{Config, Result};

/// `Resend` APIs for `METHOD /audiences/:id/contacts` endpoints.
#[derive(Clone)]
pub struct Contacts(pub(crate) Arc<Config>);

impl Contacts {
    /// Create a contact inside an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/create-contact>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn add(
        &self,
        audience_id: &str,
        contact: CreateContactRequest,
    ) -> Result<CreateContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::POST, &path);
        let response = request.json(&contact).send().await?;
        let content = response.json::<CreateContactResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single contact from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn retrieve(
        &self,
        contact_id: &str,
        audience_id: &str,
    ) -> Result<GetContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::GET, &path);
        let response = request.send().await?;
        let content = response.json::<GetContactResponse>().await?;

        Ok(content)
    }

    /// Update an existing contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/update-contact>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn update(
        &self,
        contact_id: &str,
        audience_id: &str,
        contact: UpdateContactRequest,
    ) -> Result<UpdateContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = request.json(&contact).send().await?;
        let content = response.json::<UpdateContactResponse>().await?;

        Ok(content)
    }

    /// Remove an existing contact from an audience by their email or ID.
    ///
    /// <https://resend.com/docs/api-reference/contacts/delete-contact>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn delete(&self, audience_id: &str, email_or_id: &str) -> Result<()> {
        let path = format!("/audiences/{audience_id}/contacts/{email_or_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = request.send().await?;

        Ok(())
    }

    /// Show all contacts from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contacts>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn list(&self, audience_id: &str) -> Result<ListContactsResponse> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::GET, &path);
        let response = request.send().await?;
        let content = response.json::<ListContactsResponse>().await?;

        Ok(content)
    }

    /// Create a contact inside an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/create-contact>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn add(
        &self,
        audience_id: &str,
        contact: CreateContactRequest,
    ) -> Result<CreateContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::POST, &path);
        let response = request.json(&contact).send()?;
        let content = response.json::<CreateContactResponse>()?;

        Ok(content)
    }

    /// Retrieve a single contact from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn retrieve(&self, contact_id: &str, audience_id: &str) -> Result<GetContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::GET, &path);
        let response = request.send()?;
        let content = response.json::<GetContactResponse>()?;

        Ok(content)
    }

    /// Update an existing contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/update-contact>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn update(
        &self,
        contact_id: &str,
        audience_id: &str,
        contact: UpdateContactRequest,
    ) -> Result<UpdateContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = request.json(&contact).send()?;
        let content = response.json::<UpdateContactResponse>()?;

        Ok(content)
    }

    /// Remove an existing contact from an audience by their email or ID.
    ///
    /// <https://resend.com/docs/api-reference/contacts/delete-contact>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn delete(&self, audience_id: &str, email_or_id: &str) -> Result<()> {
        let path = format!("/audiences/{audience_id}/contacts/{email_or_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = request.send()?;

        Ok(())
    }

    /// Show all contacts from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contacts>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn list(&self, audience_id: &str) -> Result<ListContactsResponse> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::GET, &path);
        let response = request.send()?;
        let content = response.json::<ListContactsResponse>()?;

        Ok(content)
    }
}

impl fmt::Debug for Contacts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateContactRequest {
        /// Email address of the contact.
        pub email: String,
        /// First name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        /// Last name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        /// Indicates if the contact is unsubscribed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<bool>,
        /// Unique identifier of the audience to which the contact belongs.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub audience_id: Option<String>,
    }

    impl CreateContactRequest {
        /// Creates a new [`CreateContactRequest`].
        pub fn new(email: &str) -> Self {
            Self {
                email: email.to_owned(),
                first_name: None,
                last_name: None,
                unsubscribed: None,
                audience_id: None,
            }
        }

        /// Adds the first name to the contact.
        #[inline]
        pub fn with_first_name(mut self, name: &str) -> Self {
            self.first_name = Some(name.to_owned());
            self
        }

        /// Adds the last name to the contact.
        #[inline]
        pub fn with_last_name(mut self, name: &str) -> Self {
            self.last_name = Some(name.to_owned());
            self
        }

        /// Toggles the unsubscribe status to `unsubscribe`.
        #[inline]
        pub fn with_unsubscribed(mut self, unsubscribed: bool) -> Self {
            self.unsubscribed = Some(unsubscribed);
            self
        }

        /// Adds a contact to the audience.
        #[inline]
        pub fn with_audience(mut self, id: &str) -> Self {
            self.audience_id = Some(id.to_owned());
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateContactResponse {
        /// Type of the response object.
        pub object: Option<String>,
        /// Unique identifier for the created contact.
        pub id: Option<String>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListContactsResponse {
        /// Type of the response object.
        pub object: Option<String>,
        /// Array containing contact information.
        pub data: Option<Vec<ListContactsItem>>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct GetContactResponse {
        /// Type of the response object.
        pub object: Option<String>,
        /// Unique identifier for the contact.
        pub id: Option<String>,
        /// Email address of the contact.
        pub email: Option<String>,
        /// First name of the contact.
        pub first_name: Option<String>,
        /// Last name of the contact.
        pub last_name: Option<String>,
        /// Timestamp indicating when the contact was created.
        pub created_at: Option<String>,
        /// Indicates if the contact is unsubscribed.
        pub unsubscribed: Option<bool>,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct UpdateContactRequest {
        /// Email address of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        /// First name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        /// Last name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        /// Indicates the subscription status of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<bool>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateContactResponse {
        /// Type of the response object.
        pub object: Option<String>,
        /// Unique identifier for the updated contact.
        pub id: Option<String>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListContactsItem {
        /// Unique identifier for the contact.
        pub id: Option<String>,
        /// Email address of the contact.
        pub email: Option<String>,
        /// First name of the contact.
        pub first_name: Option<String>,
        /// Last name of the contact.
        pub last_name: Option<String>,
        /// Timestamp indicating when the contact was created.
        pub created_at: Option<String>,
        /// Indicates if the contact is unsubscribed.
        pub unsubscribed: Option<bool>,
    }
}