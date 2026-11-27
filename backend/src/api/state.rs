use std::sync::Arc;

use crate::domain::unit_of_work::UnitOfWorkFactory;
use crate::service::auth::LoginGuard;

#[derive(Clone)]
pub struct AppState {
    /// 所有业务函数共享的工作单元工厂，业务函数以参数形式注入。
    pub db: Arc<dyn UnitOfWorkFactory>,
    /// 登录锁定状态：失败计数与锁定截止时间需要跨请求共享。
    pub auth: LoginGuard,
}
