use std::env;

use glean_core::metrics::*;
use glean_core::ping::PingMaker;
use glean_core::{CommonMetricData, Glean};
use tempfile::Builder;

fn main() {
    env_logger::init();
    color_backtrace::install();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        path
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().display().to_string()
    };

    let mut glean = Glean::new(&data_path, "org.mozilla.glean_core.example", true).unwrap();
    glean.register_ping_type(&PingType::new("baseline", true));
    glean.register_ping_type(&PingType::new("metrics", true));

    let local_metric: StringMetric = StringMetric::new(CommonMetricData {
        name: "local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["baseline".into()],
        ..Default::default()
    });

    let call_counter: CounterMetric = CounterMetric::new(CommonMetricData {
        name: "calls".into(),
        category: "local".into(),
        send_in_pings: vec!["baseline".into(), "metrics".into()],
        ..Default::default()
    });

    local_metric.set(&glean, "I can set this");
    call_counter.add(&glean, 1);

    println!("Baseline Data:\n{}", glean.snapshot("baseline", true));

    call_counter.add(&glean, 2);
    println!("Metrics Data:\n{}", glean.snapshot("metrics", true));

    call_counter.add(&glean, 3);

    println!();
    println!("Baseline Data 2:\n{}", glean.snapshot("baseline", false));
    println!("Metrics Data 2:\n{}", glean.snapshot("metrics", true));

    let list: StringListMetric = StringListMetric::new(CommonMetricData {
        name: "list".into(),
        category: "local".into(),
        send_in_pings: vec!["baseline".into()],
        ..Default::default()
    });
    list.add(&glean, "once");
    list.add(&glean, "upon");

    let ping_maker = PingMaker::new();
    let ping = ping_maker
        .collect_string(&glean, glean.get_ping_by_name("baseline").unwrap())
        .unwrap();
    println!("Baseline Ping:\n{}", ping);

    let ping = ping_maker.collect_string(&glean, glean.get_ping_by_name("metrics").unwrap());
    println!("Metrics Ping: {:?}", ping);
}
