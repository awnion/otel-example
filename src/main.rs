

use opentelemetry::{
    global,
    trace::{get_active_span, Tracer},
    KeyValue,
};

#[tokio::main]
async fn main() {
    // let tracer =
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    global::tracer("app").in_span("doing_work", |_cx| {
        get_active_span(|span| {
            span.add_event("An event!", vec![KeyValue::new("happened", true)]);
            // span.set_status(Status::Error {
            //     description: "Error?".into(),
            // });
            span.set_attribute(KeyValue {
                key: "test".into(),
                value: "123".into(),
            });
            span.end();
        });

        println!("Hello!")
    });

    global::shutdown_tracer_provider();
}
