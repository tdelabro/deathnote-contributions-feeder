mod debug;
mod error;
mod info;
mod trace;
mod warn;

mod stdlog;
pub use opentelemetry;
pub use stdlog::LogTracer;
pub use tracing;
pub use tracing_opentelemetry;

/// Retrieves current trace ID and converts it to Datadog format.
#[macro_export]
macro_rules! trace_id {
	() => {
		u128::from_be_bytes(
			$crate::opentelemetry::trace::TraceContextExt::span(
				&$crate::tracing_opentelemetry::OpenTelemetrySpanExt::context(
					&$crate::tracing::Span::current(),
				),
			)
			.span_context()
			.trace_id()
			.to_bytes(),
		) as u64
	};
}

#[macro_export]
macro_rules! trace_id_str {
	() => {
		format!("{}", $crate::trace_id!())
	};
}

/// Retrieves current span ID and converts it to Datadog format.
#[macro_export]
macro_rules! span_id {
	() => {
		u64::from_be_bytes(
			$crate::opentelemetry::trace::TraceContextExt::span(
				&$crate::tracing_opentelemetry::OpenTelemetrySpanExt::context(
					&$crate::tracing::Span::current(),
				),
			)
			.span_context()
			.span_id()
			.to_bytes(),
		)
	};
}

#[macro_export]
macro_rules! span_id_str {
	() => {
		format!("{}", $crate::span_id!())
	};
}

#[cfg(test)]
#[ctor::ctor]
fn init_tracing_for_tests() {
	use tracing::Level;
	use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

	use crate::stdlog::LogTracer;

	let tracer = opentelemetry::sdk::export::trace::stdout::new_pipeline()
		.with_writer(std::io::sink())
		.install_simple();
	let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

	let subscriber = tracing_subscriber::fmt::Subscriber::builder()
		.with_max_level(Level::TRACE)
		.finish()
		.with(telemetry);

	LogTracer::init().unwrap();

	// Trace executed code
	tracing::subscriber::set_global_default(subscriber).unwrap();
}
