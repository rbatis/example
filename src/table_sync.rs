use rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbs::to_value;

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

#[tokio::main]
pub async fn main() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    ).expect("rbatis init fail");
    let rb = RBatis::new();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
        .unwrap();

    let syncer = SqliteTableSync::default();
    syncer.sync(rb.acquire().await.unwrap(), to_value!(BizActivity {
        id: Some("1".into()),
        name: Some("1".into()),
        pc_link: Some("1".into()),
        h5_link: Some("1".into()),
        pc_banner_img: Some("1".into()),
        h5_banner_img: Some("1".into()),
        sort: Some("1".into()),
        status: Some(1),
        remark: Some("1".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    }), "biz_activity").await.unwrap();
}