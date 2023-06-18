#[macro_use]
extern crate rbatis;

pub mod init;

use crate::init::init_db;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::page::PageRequest;
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

impl_select_page!(BizActivity{select_page() =>"
     if !sql.contains('count(1)'):
       `order by create_time desc`"});
impl_select_page!(BizActivity{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});

#[tokio::main]
pub async fn main() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    )
    .expect("rbatis init fail");
    let mut rb = init_db().await;
    let data = BizActivity::select_page(&mut rb, &PageRequest::new(1, 10)).await;
    println!("select_page = {}", json!(data));

    let data = BizActivity::select_page_by_name(&mut rb, &PageRequest::new(1, 10), "").await;
    println!("select_page_by_name = {}", json!(data));
}
