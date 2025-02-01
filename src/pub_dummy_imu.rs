use zenoh::config::Config;
use zenoh::prelude::sync::SyncResolve;

use imu_message::IMUMessage;
fn main() {
    // initiate logging
    env_logger::init();

    let config = Config::default();
    let key_expr = "demo/imu";

    let session = zenoh::open(config).res().unwrap();
    let zpub = session.declare_publisher(key_expr).res().unwrap();

    let mut id = 0;
    loop {
        let imu_message = IMUMessage {
            id,
            gyro: vec![1.0, 2.0, 3.0],
            accl: vec![4.0, 5.0, 6.0],
            mag: vec![7.0, 8.0, 9.0],
        };
        // 4. データをシリアライズ
        let serialized_data = bincode::serialize(&imu_message).unwrap();

        // 5. Zenohでデータをパブリッシュ
        zpub.put(serialized_data).res().unwrap();
        println!("{}", id);

        // 6. 1秒待つ
        std::thread::sleep(std::time::Duration::from_millis(100));
        id += 1;
    }
}
