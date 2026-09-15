//! 服务层模块
//!
//! 承载纯业务逻辑：余额查询、记账、开机自启、版本检查。

pub mod autostart;
pub mod balance;
pub mod ledger;
pub mod update;
