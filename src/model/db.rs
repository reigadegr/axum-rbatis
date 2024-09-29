use rbatis::rbatis::RBatis;

pub async fn init_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:1234@127.0.0.1:3306/bs_desktop").unwrap();
    return rb;
}
