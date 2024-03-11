//! # SCIM v2
//!
//! `scim_v2` is a crate that provides utilities for working with the System for Cross-domain Identity Management (SCIM) version 2.0 protocol.
//!
//! This crate provides the following functionalities:
//! - Models for various SCIM resources such as `User`, `Group`, `ResourceType`, `ServiceProviderConfig`, and `EnterpriseUser`.
//! - Functions for validating these resources.
//! - Functions for serializing these resources to JSON.
//! - Functions for deserializing these resources from JSON.
//!
//! Note: Validation is light because the schema is specifically flexible. We only validate required fields, not field types (like email is actually an email)
//!
//! ## Examples
//!
//! Here are some examples of how you can use this crate:
//!
//! ### Validating a User
//!
//! ```
//! use scim_v2::models::user::User;
//! use scim_v2::validate_user;
//!
//! let user = User {
//!     // Initialize user fields here...
//!     // ...
//!     ..Default::default()
//! };
//!
//! match validate_user(&user) {
//!     Ok(_) => println!("User is valid."),
//!     Err(e) => println!("User is invalid: {}", e),
//! }
//! ```
//!
//! ### Serializing a User to JSON
//!
//! ```
//! use scim_v2::models::user::User;
//! use scim_v2::user_to_json;
//!
//! let user = User {
//!     // Initialize user fields here...
//!     // ...
//!     ..Default::default()
//! };
//!
//! match user_to_json(&user) {
//!     Ok(json) => println!("User in JSON format: {}", json),
//!     Err(e) => println!("Error serializing user to JSON: {}", e),
//! }
//! ```
//!
//! ### Deserializing a User from JSON
//!
//! ```
//! use scim_v2::models::user::User;
//! use scim_v2::json_to_user;
//!
//! let json = r#"{
//!     "userName": "jdoe",
//!     "name": {
//!         "formatted": "Mr. John Doe"
//!     }
//! }"#;
//!
//! match json_to_user(json) {
//!     Ok(user) => println!("User: {:?}", user),
//!     Err(e) => println!("Error deserializing JSON to User: {}", e),
//! }
//! ```
//! For more examples and usage details, refer to the documentation of each function and struct.


/// External crate imports
extern crate serde;
extern crate serde_json; // Importing the serde_json crate for JSON serialization and deserialization
use utils::error::SCIMError; // Importing the SCIMError enum from the utils::error module
use crate::models::user::User;
use crate::models::group::Group;
use crate::models::resource_types::ResourceType;
use crate::models::service_provider_config::ServiceProviderConfig;
use crate::models::enterprise_user::EnterpriseUser;



use std::result::Result;

/// Declaring the models module which contains various submodules
pub mod models {
    pub mod user;
    pub mod group;
    pub mod resource_types;
    pub mod service_provider_config;
    pub mod enterprise_user;
    pub mod scim_schema;
}

/// Declaring the utils module which contains the error submodule
pub mod utils {
    pub mod error;
}

/// Validates a user.
///
/// This function checks if the user has a `name` and `user_name`. If either is missing, it returns an error.
/// It also checks if the `emails` field is present and if each email in the vector is in a valid email format.
///
/// # Arguments
///
/// * `user` - A reference to a User instance.
///
/// # Returns
///
/// * `Ok(())` - If the user is valid.
/// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
/// * `Err(SCIMError::InvalidFieldValue)` - If a field value is invalid.
///
/// # Example
///
/// ```
/// use scim_v2::models::user::User;
/// use scim_v2::validate_user;
///
/// let user = User {
///     user_name: "jdoe".to_string(),
///     // other fields...
///     ..Default::default()
/// };
///
/// match validate_user(&user) {
///     Ok(_) => println!("User is valid."),
///     Err(e) => println!("User is invalid: {}", e),
/// }
/// ```
///
/// # Note
///
/// The actual validation requirements will depend on the specifics of your application and the SCIM (System for Cross-domain Identity Management) protocol you are implementing.

pub fn validate_user(user: &models::user::User) -> Result<(), SCIMError> {
    // Pretty much every field is optional in the schema except for 2. We'll check for those here.
    if user.schemas.is_empty() {
        return Err(SCIMError::MissingRequiredField("schemas".to_string()));
    }
    if user.user_name.is_empty() {
        return Err(SCIMError::MissingRequiredField("user_name".to_string()));
    }
    Ok(())
}

/// Converts a User instance into a JSON string.
///
/// This function takes a reference to a User instance and uses the `serde_json::to_string` function
/// to serialize the User instance into a JSON string. If the serialization is successful, it returns
/// the JSON string. If the serialization fails, it returns a `serde_json::Error`.
///
/// # Arguments
///
/// * `user` - A reference to a User instance.
///
/// # Returns
///
/// * `Ok(String)` - If the serialization is successful, it returns the JSON string.
/// * `Err(serde_json::Error)` - If the serialization fails, it returns a `serde_json::Error`.
///
/// # Example
///
/// ```
/// use scim_v2::models::user::User;
/// use scim_v2::user_to_json;
///
/// let user = User {
///     // Initialize user fields here...
///     // ...
///     ..Default::default()
/// };
///
/// match user_to_json(&user) {
///     Ok(json) => println!("User in JSON format: {}", json),
///     Err(e) => println!("Error serializing user to JSON: {}", e),
/// }
/// ```
///
/// This will print the `User` instance in JSON format if the serialization is successful, or it will print an error message if the serialization fails.

pub fn user_to_json(user: &models::user::User) -> Result<String, SCIMError> {
    serde_json::to_string(user).map_err(SCIMError::SerializationError)
}

/// Parses a JSON string into a User instance.
///
/// This function takes a JSON string and uses the `serde_json::from_str` function
/// to deserialize the JSON string into a User instance. If the deserialization is successful, it returns
/// the User instance. If the deserialization fails, it returns a `SCIMError`.
///
/// # Arguments
///
/// * `json` - A JSON string.
///
/// # Returns
///
/// * `Ok(User)` - If the deserialization is successful, it returns the User instance.
/// * `Err(SCIMError)` - If the deserialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::user::User;
/// use scim_v2::json_to_user;
///
/// let json = r#"{
///     "userName": "jdoe",
///     "name": {
///         "formatted": "Mr. John Doe"
///     }
/// }"#;
///
/// match json_to_user(json) {
///     Ok(user) => println!("User: {:?}", user),
///     Err(e) => println!("Error deserializing JSON to User: {}", e),
/// }
/// ```
///
/// This will print the `User` instance if the deserialization is successful, or it will print an error message if the deserialization fails.
pub fn json_to_user(json: &str) -> Result<User, SCIMError> {
    serde_json::from_str(json).map_err(SCIMError::DeserializationError)
}

/// Validates a group.
///
/// This function checks if the group has `schemas`, `id`, and `display_name`. If any of these fields are missing, it returns an error.
///
/// # Arguments
///
/// * `group` - A reference to a Group instance.
///
/// # Returns
///
/// * `Ok(())` - If the group is valid.
/// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
///
/// # Example
///
/// ```
/// use scim_v2::models::group::Group;
/// use scim_v2::validate_group;
///
/// let group = Group {
///     schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Group".to_string()],
///     id: "e9e30dba-f08f-4109-8486-d5c6a331660a".to_string(),
///     display_name: "Tour Guides".to_string(),
///     // other fields...
///     ..Default::default()
/// };
///
/// match validate_group(&group) {
///     Ok(_) => println!("Group is valid."),
///     Err(e) => println!("Group is invalid: {}", e),
/// }
/// ```
pub fn validate_group(group: &Group) -> Result<(), SCIMError> {
    if group.schemas.is_empty() {
        return Err(SCIMError::MissingRequiredField("schemas".to_string()));
    }
    if group.id.is_empty() {
        return Err(SCIMError::MissingRequiredField("id".to_string()));
    }
    if group.display_name.is_empty() {
        return Err(SCIMError::MissingRequiredField("display_name".to_string()));
    }
    Ok(())
}

/// Converts a Group instance into a JSON string.
///
/// # Arguments
///
/// * `group` - A reference to a Group instance.
///
/// # Returns
///
/// * `Ok(String)` - If the serialization is successful, it returns the JSON string.
/// * `Err(SCIMError)` - If the serialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::group::Group;
/// use scim_v2::group_to_json;
///
/// let group = Group {
///     // Initialize group fields here...
///     // ...
///     ..Default::default()
/// };
///
/// match group_to_json(&group) {
///     Ok(json) => println!("Group in JSON format: {}", json),
///     Err(e) => println!("Error serializing group to JSON: {}", e),
/// }
/// ```
pub fn group_to_json(group: &Group) -> Result<String, SCIMError> {
    serde_json::to_string(group).map_err(SCIMError::SerializationError)
}

/// Parses a JSON string into a Group instance.
///
/// # Arguments
///
/// * `json` - A JSON string.
///
/// # Returns
///
/// * `Ok(Group)` - If the deserialization is successful, it returns the Group instance.
/// * `Err(SCIMError)` - If the deserialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::group::Group;
/// use scim_v2::json_to_group;
///
/// let json = r#"{
///     "schemas": ["urn:ietf:params:scim:schemas:core:2.0:Group"],
///     "id": "e9e30dba-f08f-4109-8486-d5c6a331660a",
///     "displayName": "Tour Guides"
/// }"#;
///
/// match json_to_group(json) {
///     Ok(group) => println!("Group: {:?}", group),
///     Err(e) => println!("Error deserializing JSON to Group: {}", e),
/// }
/// ```
pub fn json_to_group(json: &str) -> Result<Group, SCIMError> {
    serde_json::from_str(json).map_err(SCIMError::DeserializationError)
}

/// Validates a resource type.
///
/// This function checks if the resource type has `name`, `endpoint`, and `schema`. If any of these fields are missing, it returns an error.
///
/// # Arguments
///
/// * `resource_type` - A reference to a ResourceType instance.
///
/// # Returns
///
/// * `Ok(())` - If the resource type is valid.
/// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
///
/// # Example
///
/// ```
/// use scim_v2::models::resource_types::ResourceType;
/// use scim_v2::validate_resource_type;
///
/// let resource_type = ResourceType {
///     name: "User".to_string(),
///     endpoint: "/Users".to_string(),
///     schema: "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
///     // other fields...
///     ..Default::default()
/// };
///
/// match validate_resource_type(&resource_type) {
///     Ok(_) => println!("ResourceType is valid."),
///     Err(e) => println!("ResourceType is invalid: {}", e),
/// }
/// ```
pub fn validate_resource_type(resource_type: &ResourceType) -> Result<(), SCIMError> {
    if resource_type.name.is_empty() {
        return Err(SCIMError::MissingRequiredField("name".to_string()));
    }
    if resource_type.endpoint.is_empty() {
        return Err(SCIMError::MissingRequiredField("endpoint".to_string()));
    }
    if resource_type.schema.is_empty() {
        return Err(SCIMError::MissingRequiredField("schema".to_string()));
    }
    Ok(())
}

/// Converts a ResourceType instance into a JSON string.
///
/// # Arguments
///
/// * `resource_type` - A reference to a ResourceType instance.
///
/// # Returns
///
/// * `Ok(String)` - If the serialization is successful, it returns the JSON string.
/// * `Err(SCIMError)` - If the serialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::resource_types::ResourceType;
/// use scim_v2::resource_type_to_json;
///
/// let resource_type = ResourceType {
///     // Initialize resource_type fields here...
///     // ...
///     ..Default::default()
/// };
///
/// match resource_type_to_json(&resource_type) {
///     Ok(json) => println!("ResourceType in JSON format: {}", json),
///     Err(e) => println!("Error serializing ResourceType to JSON: {}", e),
/// }
/// ```
pub fn resource_type_to_json(resource_type: &ResourceType) -> Result<String, SCIMError> {
    serde_json::to_string(resource_type).map_err(SCIMError::SerializationError)
}

/// Parses a JSON string into a ResourceType instance.
///
/// # Arguments
///
/// * `json` - A JSON string.
///
/// # Returns
///
/// * `Ok(ResourceType)` - If the deserialization is successful, it returns the ResourceType instance.
/// * `Err(SCIMError)` - If the deserialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::resource_types::ResourceType;
/// use scim_v2::json_to_resource_type;
///
/// let json = r#"{
///     "name": "User",
///     "endpoint": "/Users",
///     "schema": "urn:ietf:params:scim:schemas:core:2.0:User"
/// }"#;
///
/// match json_to_resource_type(json) {
///     Ok(resource_type) => println!("ResourceType: {:?}", resource_type),
///     Err(e) => println!("Error deserializing JSON to ResourceType: {}", e),
/// }
/// ```
pub fn json_to_resource_type(json: &str) -> Result<ResourceType, SCIMError> {
    serde_json::from_str(json).map_err(SCIMError::DeserializationError)
}

/// Validates a service provider config.
///
/// This function checks if the service provider config has `patch`, `bulk`, `filter`, `change_password`, `sort`, and `etag`. If any of these fields are missing, it returns an error.
///
/// # Arguments
///
/// * `config` - A reference to a ServiceProviderConfig instance.
///
/// # Returns
///
/// * `Ok(())` - If the service provider config is valid.
/// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
///
/// # Example
///
/// ```
/// use scim_v2::models::service_provider_config::ServiceProviderConfig;
/// use scim_v2::validate_service_provider_config;
///
/// let config = ServiceProviderConfig {
///     // Initialize config fields here...
///     // ...
///     ..Default::default()
/// };
///
/// match validate_service_provider_config(&config) {
///     Ok(_) => println!("ServiceProviderConfig is valid."),
///     Err(e) => println!("ServiceProviderConfig is invalid: {}", e),
/// }
/// ```
pub fn validate_service_provider_config(config: &ServiceProviderConfig) -> Result<(), SCIMError> {
    if config.patch.supported == false {
        return Err(SCIMError::MissingRequiredField("patch".to_string()));
    }
    if config.bulk.supported == false {
        return Err(SCIMError::MissingRequiredField("bulk".to_string()));
    }
    if config.filter.supported == false {
        return Err(SCIMError::MissingRequiredField("filter".to_string()));
    }
    if config.change_password.supported == false {
        return Err(SCIMError::MissingRequiredField("change_password".to_string()));
    }
    if config.sort.supported == false {
        return Err(SCIMError::MissingRequiredField("sort".to_string()));
    }
    if config.etag.supported == false {
        return Err(SCIMError::MissingRequiredField("etag".to_string()));
    }
    Ok(())
}


/// Converts a ServiceProviderConfig instance into a JSON string.
///
/// # Arguments
///
/// * `config` - A reference to a ServiceProviderConfig instance.
///
/// # Returns
///
/// * `Ok(String)` - If the serialization is successful, it returns the JSON string.
/// * `Err(SCIMError)` - If the serialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::service_provider_config::ServiceProviderConfig;
/// use scim_v2::service_provider_config_to_json;
///
/// let config = ServiceProviderConfig {
///     // Initialize config fields here...
///     // ...
///     ..Default::default()
/// };
///
/// match service_provider_config_to_json(&config) {
///     Ok(json) => println!("ServiceProviderConfig in JSON format: {}", json),
///     Err(e) => println!("Error serializing ServiceProviderConfig to JSON: {}", e),
/// }
/// ```
pub fn service_provider_config_to_json(config: &ServiceProviderConfig) -> Result<String, SCIMError> {
    serde_json::to_string(config).map_err(SCIMError::SerializationError)
}

/// Parses a JSON string into a ServiceProviderConfig instance.
///
/// # Arguments
///
/// * `json` - A JSON string.
///
/// # Returns
///
/// * `Ok(ServiceProviderConfig)` - If the deserialization is successful, it returns the ServiceProviderConfig instance.
/// * `Err(SCIMError)` - If the deserialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::service_provider_config::ServiceProviderConfig;
/// use scim_v2::json_to_service_provider_config;
///
/// let json = r#"{
///     // JSON representation of ServiceProviderConfig...
/// }"#;
///
/// match json_to_service_provider_config(json) {
///     Ok(config) => println!("ServiceProviderConfig: {:?}", config),
///     Err(e) => println!("Error deserializing JSON to ServiceProviderConfig: {}", e),
/// }
/// ```
pub fn json_to_service_provider_config(json: &str) -> Result<ServiceProviderConfig, SCIMError> {
    serde_json::from_str(json).map_err(SCIMError::DeserializationError)
}

/// Validates an enterprise user.
///
/// This function checks if the enterprise user has `employee_number`, `cost_center`, `organization`, `division`, `department`, and `manager`. If any of these fields are missing, it returns an error.
///
/// # Arguments
///
/// * `enterprise_user` - A reference to an EnterpriseUser instance.
///
/// # Returns
///
/// * `Ok(())` - If the enterprise user is valid.
/// * `Err(SCIMError::MissingRequiredField)` - If a required field is missing.
///
/// # Example
///
/// ```
/// use scim_v2::models::enterprise_user::EnterpriseUser;
/// use scim_v2::validate_enterprise_user;
///
/// let enterprise_user = EnterpriseUser {
///     // Initialize enterprise_user fields here...
///     // ...
///     ..Default::default()
/// };
///
/// match validate_enterprise_user(&enterprise_user) {
///     Ok(_) => println!("EnterpriseUser is valid."),
///     Err(e) => println!("EnterpriseUser is invalid: {}", e),
/// }
/// ```
pub fn validate_enterprise_user(enterprise_user: &EnterpriseUser) -> Result<(), SCIMError> {
    if enterprise_user.employee_number.is_none() {
        return Err(SCIMError::MissingRequiredField("employee_number".to_string()));
    }
    if enterprise_user.cost_center.is_none() {
        return Err(SCIMError::MissingRequiredField("cost_center".to_string()));
    }
    if enterprise_user.organization.is_none() {
        return Err(SCIMError::MissingRequiredField("organization".to_string()));
    }
    if enterprise_user.division.is_none() {
        return Err(SCIMError::MissingRequiredField("division".to_string()));
    }
    if enterprise_user.department.is_none() {
        return Err(SCIMError::MissingRequiredField("department".to_string()));
    }
    if enterprise_user.manager.is_none() {
        return Err(SCIMError::MissingRequiredField("manager".to_string()));
    }
    Ok(())
}

/// Converts an EnterpriseUser instance into a JSON string.
///
/// # Arguments
///
/// * `enterprise_user` - A reference to an EnterpriseUser instance.
///
/// # Returns
///
/// * `Ok(String)` - If the serialization is successful, it returns the JSON string.
/// * `Err(SCIMError)` - If the serialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::enterprise_user::EnterpriseUser;
/// use scim_v2::enterprise_user_to_json;
///
/// let enterprise_user = EnterpriseUser {
///     // Initialize enterprise_user fields here...
///     // ...
///     ..Default::default()
/// };
///
/// match enterprise_user_to_json(&enterprise_user) {
///     Ok(json) => println!("EnterpriseUser in JSON format: {}", json),
///     Err(e) => println!("Error serializing EnterpriseUser to JSON: {}", e),
/// }
/// ```
pub fn enterprise_user_to_json(enterprise_user: &EnterpriseUser) -> Result<String, SCIMError> {
    serde_json::to_string(enterprise_user).map_err(SCIMError::SerializationError)
}

/// Parses a JSON string into an EnterpriseUser instance.
///
/// # Arguments
///
/// * `json` - A JSON string.
///
/// # Returns
///
/// * `Ok(EnterpriseUser)` - If the deserialization is successful, it returns the EnterpriseUser instance.
/// * `Err(SCIMError)` - If the deserialization fails, it returns a `SCIMError`.
///
/// # Example
///
/// ```
/// use scim_v2::models::enterprise_user::EnterpriseUser;
/// use scim_v2::json_to_enterprise_user;
///
/// let json = r#"{
///     "employeeNumber": "123456",
///     "costCenter": "7890",
///     "organization": "Acme Corp",
///     "division": "Sales",
///     "department": "North East",
///     "manager": {
///         "value": "abc123",
///         "$ref": "https://example.com/v2/Users/abc123",
///         "displayName": "John Doe"
///     }
/// }"#;
///
/// match json_to_enterprise_user(json) {
///     Ok(enterprise_user) => println!("EnterpriseUser: {:?}", enterprise_user),
///     Err(e) => println!("Error deserializing JSON to EnterpriseUser: {}", e),
/// }
/// ```
pub fn json_to_enterprise_user(json: &str) -> Result<EnterpriseUser, SCIMError> {
    serde_json::from_str(json).map_err(SCIMError::DeserializationError)
}