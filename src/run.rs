use std::thread::sleep;
use std::time::Duration;
use std::vec;

use opentelemetry::KeyValue;
use opentelemetry_sdk::metrics::reader::DefaultAggregationSelector;
use opentelemetry_sdk::metrics::reader::DefaultTemporalitySelector;
use opentelemetry_sdk::metrics::MeterProviderBuilder;
use opentelemetry_sdk::metrics::PeriodicReader;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::Resource;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::MetricsLayer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// Create a Resource that captures information about the entity for which
// telemetry is recorded.
fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new("service_name", env!("CARGO_PKG_NAME")),
            KeyValue::new("service_version", env!("CARGO_PKG_VERSION")),
            KeyValue::new("deployment", "develop"),
        ],
        "https://opentelemetry.io/schemas/1.2.0",
    )
}

pub fn init_tracing() {
    // Init tracing
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
        .unwrap();

    let filter = tracing_subscriber::filter::Targets::new().with_default(LevelFilter::DEBUG);
    tracing_subscriber::registry()
        .with(filter)
        .with(MetricsLayer::new(init_meter_provider()))
        .with(OpenTelemetryLayer::new(tracer))
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_thread_ids(true)
                .with_ansi(false)
                .with_writer(std::io::stderr),
        )
        .init();
}

fn init_meter_provider() -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .build_metrics_exporter(
            Box::new(DefaultAggregationSelector::new()),
            Box::new(DefaultTemporalitySelector::new()),
        )
        .unwrap();

    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        .with_interval(std::time::Duration::from_secs(3))
        .build();

    let meter_provider =
        MeterProviderBuilder::default().with_resource(resource()).with_reader(reader).build();

    opentelemetry::global::set_meter_provider(meter_provider.clone());

    meter_provider
}

pub fn run() -> anyhow::Result<()> {
    init_tracing();

    loop {
        sleep(Duration::from_secs(2));
        do_something()?;
        info!("test1");
        info!(monotonic_counter.foo = 1);
        info!(counter.buz = -2);
    }
}

#[tracing::instrument]
pub fn do_something() -> anyhow::Result<()> {
    let mut v = vec![1, 2, 3];
    v.push(4);
    info!(?v, "do2");
    info!(monotonic_counter.foo = 1);
    info!(counter.buz = 3);
    Ok(())
}
