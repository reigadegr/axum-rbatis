use redis::Connection;

pub async fn init_redis() -> Connection {

    let client =redis::Client::open("redis://root@127.0.0.1:6379").unwrap();
    client.get_connection().unwrap()
}
