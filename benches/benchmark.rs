use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use httpmock::MockServer;
use reqwest::Client;
use serde_json::json;
use tokio::runtime::Runtime;

fn bench_http_methods(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    let methods = ["GET", "DELETE", "PUT", "PATCH", "POST"];

    for &method in &methods {
        let server = MockServer::start();
        let url = server.url("/ping");

        let _mock = server.mock(|when, then| {
            when.method(method).path("/ping");
            then.status(200).body("ok");
        });

        c.bench_with_input(BenchmarkId::new("http_method", method), &method, |b, &m| {
            b.iter(|| {
                let url = url.clone();
                rt.block_on(async {
                    for _ in 0..10 {
                        let client = Client::new();
                        let request = match m {
                            "GET" => client.get(&url),
                            "DELETE" => client.delete(&url),
                            "PUT" => client.put(&url).json(&json!({ "i": 1 })),
                            "PATCH" => client.patch(&url).json(&json!({ "i": 1 })),
                            "POST" => client.post(&url).json(&json!({ "i": 1 })),
                            _ => unreachable!("Invalid method"),
                        };

                        let res = request.send().await.expect("Request failed");
                        assert_eq!(res.status().as_u16(), 200);
                        let body = res.text().await.expect("Read body failed");
                        assert_eq!(body, "ok");
                    }
                });
            });
        });
    }
}

criterion_group!(benches, bench_http_methods);
criterion_main!(benches);

