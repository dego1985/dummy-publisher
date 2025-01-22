use clap::{App, Arg};
use zenoh::config::Config;
use zenoh::prelude::sync::SyncResolve;

use imu_message::IMUMessage;
fn main() {
    // initiate logging
    env_logger::init();

    let (config, key_expr) = parse_args();

    println!("Openning session...");
    let session = zenoh::open(config).res().unwrap();
    let zpub = session.declare_publisher(&key_expr).res().unwrap();

    let mut id = 0;
    loop {
        let imu_message = IMUMessage {
            id: id,
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
        std::thread::sleep(std::time::Duration::from_millis(1));
        id += 1;
    }
}

fn parse_args() -> (Config, String) {
    let args = App::new("zenoh imu example")
        .arg(
            Arg::from_usage("-m, --mode=[MODE] 'The zenoh session mode.")
                .possible_values(["peer", "client"])
                .default_value("peer"),
        )
        .arg(Arg::from_usage(
            "-e, --connect=[LOCATOR]...  'Endpoints to connect to.'",
        ))
        .arg(
            Arg::from_usage(
                "-k, --key=[KEY_EXPR] 'The key expression on which the video will be published.",
            )
            .default_value("demo/imu"),
        )
        .arg(Arg::from_usage(
            "-c, --config=[FILE]      'A configuration file.'",
        ))
        .get_matches();

    let mut config = if let Some(conf_file) = args.value_of("config") {
        Config::from_file(conf_file).unwrap()
    } else {
        Config::default()
    };
    if let Some(Ok(mode)) = args.value_of("mode").map(|mode| mode.parse()) {
        config.set_mode(Some(mode)).unwrap();
    }
    if let Some(connect) = args.values_of("connect") {
        config
            .connect
            .endpoints
            .extend(connect.map(|p| p.parse().unwrap()))
    }

    let key_expr = args.value_of("key").unwrap().to_string();

    (config, key_expr)
}
