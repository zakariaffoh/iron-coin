use crate::auditable::AuditableModel;
use serde::{Deserialize, Serialize};
use uuid;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleModel {
    pub role_id: String,
    pub realm_id: String,
    pub name: String,
    pub description: String,
    pub is_client_role: bool,
    pub display_name: String,
    pub metadata: Option<AuditableModel>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RoleCreateModel {
    pub realm_id: String,
    pub name: String,
    pub description: String,
    pub is_client_role: bool,
    pub display_name: String,
    pub metadata: AuditableModel,
}

impl Into<RoleModel> for RoleCreateModel {
    fn into(self) -> RoleModel {
        RoleModel {
            role_id: uuid::Uuid::new_v4().to_string(),
            realm_id: self.realm_id,
            name: self.name,
            description: self.description,
            is_client_role: self.is_client_role,
            display_name: self.display_name,
            metadata: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RoleUpdateModel {
    pub role_id: String,
    pub realm_id: String,
    pub name: String,
    pub description: String,
    pub is_client_role: bool,
    pub display_name: String,
    pub metadata: AuditableModel,
}

impl Into<RoleModel> for RoleUpdateModel {
    fn into(self) -> RoleModel {
        RoleModel {
            role_id: self.role_id,
            realm_id: self.realm_id,
            name: self.name,
            description: self.description,
            is_client_role: self.is_client_role,
            display_name: self.display_name,
            metadata: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupModel {
    pub group_id: String,
    pub realm_id: String,
    pub name: String,
    pub roles: Option<Vec<RoleModel>>,
    pub display_name: String,
    pub description: String,
    pub is_default: bool,
    pub metadata: Option<AuditableModel>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupCreateModel {
    pub realm_id: String,
    pub name: String,
    pub roles: Option<Vec<RoleModel>>,
    pub display_name: String,
    pub description: String,
    pub is_default: bool,
    pub metadata: Option<AuditableModel>,
}

impl Into<GroupModel> for GroupCreateModel {
    fn into(self) -> GroupModel {
        GroupModel {
            group_id: uuid::Uuid::new_v4().to_string(),
            realm_id: self.realm_id,
            name: self.name,
            description: self.description,
            roles: None,
            display_name: self.display_name,
            is_default: self.is_default,
            metadata: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupUpdateModel {
    pub group_id: String,
    pub realm_id: String,
    pub name: String,
    pub roles: Option<Vec<RoleModel>>,
    pub display_name: String,
    pub description: String,
    pub is_default: bool,
    pub metadata: Option<AuditableModel>,
}

impl Into<GroupModel> for GroupUpdateModel {
    fn into(self) -> GroupModel {
        GroupModel {
            group_id: self.group_id,
            realm_id: self.realm_id,
            name: self.name,
            description: self.description,
            roles: None,
            display_name: self.display_name,
            is_default: self.is_default,
            metadata: None,
        }
    }
}
