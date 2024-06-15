use std::fmt;
use std::fmt::Formatter;
use salvo::prelude::{Extractible, ToSchema};
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::entities::permission;
use crate::entities::permission::ActiveModel;
use crate::utils::date_util::get_current_time_as_string;

#[derive(Debug)]
pub enum PermissionValidType {
    URL,
    PAGE,
    REGEX,
}

#[derive(Debug)]
pub enum PermissionMethod {
    POST,
    GET,
    PUT,
    DELETE,
}

impl fmt::Display for PermissionValidType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for PermissionMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ActiveModel {
    ///从参数中生成
    pub fn from_permission_add_request(req: &PermissionAddRequest) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            name: Set(req.name.clone()),
            description: Set(req.description.clone()),
            create_time: Set(req.create_time.clone()),
            permission_url: Set(req.permission_url.clone()),
            r#type: Set(req.r#type.clone()),
            method: Set(req.method.clone()),
            group: Set(req.group.clone()),
        }
    }
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct PermissionAddRequest {
    ///权限名称
    pub name: String,
    ///介绍
    pub description: Option<String>,
    ///创建时间
    pub create_time: Option<String>,
    ///访问URL
    pub permission_url: Option<String>,
    ///验证类型,URL,PAGE,REGEX
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    ///请求方法
    pub method: Option<String>,
    ///分组标识
    pub group: Option<String>,
}

impl PermissionAddRequest {

    ///名称:方法:URL:group
    /// 快速生成
    pub fn from_simple_string_url_type(str: String) -> PermissionAddRequest {
        let parts : Vec<&str> = str.split(":").collect();
        if parts.len() == 4 {
            let p1 = parts[0];
            let p2 = parts[1];
            let p3 = parts[2];
            let p4 = parts[3];
            PermissionAddRequest {
                name: String::from(p1),
                description: None,
                create_time: Some(get_current_time_as_string()),
                permission_url: Some(String::from(p3)),
                r#type: Some("URL".to_string()),
                method: Some(p2.to_string()),
                group: Some(p4.to_string()),
            }
        }else {
            panic!("⚠️ parse data error {str}")
        }
    }

    pub fn from_regex(name: String,regex_str: String,method: String,group: String) -> PermissionAddRequest{
        PermissionAddRequest{
            name,
            description: None,
            create_time: Some(get_current_time_as_string()),
            permission_url: Some(regex_str.to_string()),
            r#type: Some("REGEX".to_string()),
            method: Some(method.to_string()),
            group: Some(group),
        }
    }
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct PermissionUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub permission_url: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub method: Option<String>,
    pub group: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default, Clone)]
pub struct PermissionResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub permission_url: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub method: Option<String>,
    pub group: Option<String>,
}


impl PermissionResponse {

    //尝试获取url匹配类型
    pub fn try_get_url(&self) -> Option<String> {
        if let Some(ref url) = self.r#type {
            if url == "URL" {
                if let Some(m) = self.method.clone() {
                    if let Some(u) = self.permission_url.clone() {
                        let f_s = format!("{}:{}",m,u);
                        return Some(f_s);
                    }
                }
            }
        }
        None
    }
}

impl Into<PermissionResponse> for permission::Model {
    fn into(self) -> PermissionResponse {
        PermissionResponse {
            id: self.id,
            name: self.name,
            description: self.description,
            create_time: self.create_time,
            permission_url: self.permission_url,
            r#type: self.r#type,
            method: self.method,
            group: self.group,
        }
    }
}