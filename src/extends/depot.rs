use salvo::Depot;
use crate::dtos::user::UserResponse;

pub trait DepotEx {
    //读取当前已登录用户变量
      async fn get_current_user(&self) -> Option<&UserResponse>;
}

impl DepotEx for Depot {
    async fn get_current_user(&self) -> Option<&UserResponse> {
        let user = self.get::<UserResponse>("user");
        match user {
            Ok(u) => Some(u),
            Err(_) => None
        }
    }
}
