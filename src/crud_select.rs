#[macro_use]
extern crate rbatis;

pub mod init;

use crate::init::init_db;
use rbatis::rbdc::datetime::DateTime;
use serde_json::json;

/// table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
//impl_select!(BizActivity{select_all_by_id(table_name:&str,id:&str) => "`where id = #{id}`"}); //custom table name
impl_select!(BizActivity{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
impl_select!(BizActivity{select_by_id(id:&str) -> Option => "`where id = #{id} limit 1`"});

#[tokio::main]
pub async fn main() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    )
    .expect("rbatis init fail");
    let mut rb = init_db().await;

    let data = BizActivity::select_all_by_id(&mut rb, "1", "1").await;
    println!("select_all_by_id = {}", json!(data));

    let data = BizActivity::select_by_id(&mut rb, "1").await;
    println!("select_by_id = {}", json!(data));
}
