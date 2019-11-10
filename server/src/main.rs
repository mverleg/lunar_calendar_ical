use std::net::SocketAddr;

use ::futures::executor::block_on;
use ::hyper;
use ::hyper::service::{make_service_fn, service_fn};

//use ::hyper::{Body, Client, Request, Response, Server, Uri};

//async fn server() -> Result<(), hyper::error::Error> {
//    let client = hyper::Client::new();
//
//    let mut resp = client.get("https://markv.nl".parse().unwrap()).await?;
//    println!("Status: {}", resp.status());
//    println!("Headers: {:#?}\n", resp.headers());
//
//    while let Some(chunk) = resp.body_mut().next().await {
//        stdout().write_all(&chunk.unwrap()).unwrap();
//    }
//
//    Ok(())
//}

async fn run_server(forward_url: &'static str, addr: SocketAddr) {
//	let server = hyper::server::Server::bind(&addr).serve(make_service_fn(move |_| {
//		async move { Ok::<_, i32>(service_fn(move |req|
//			println!("{:?}", req)
//		)) }
//	}));
//	if let Err(err) = server.await {
//		eprintln!("server error: {}", err);
//	}

	//let mut pool = LocalPool::new();
	//let mut exec = pool.executor();
	//
	//// ... spawn some initial tasks using `exec.spawn()` or `exec.spawn_local()`
	//
	//// run *all* tasks in the pool to completion, including any newly-spawned ones.
	//pool.run(&mut exec);
}

fn main() {
	let make_svc = make_service_fn(|_| async {
		Ok::<hyper::Response<_>, hyper::Error>(service_fn(|req| async {
			let msg = hyper::Response::new(hyper::Body::from("Hello World"));
			Ok::<_, hyper::Error>(msg)
		}))
	});

	let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
	let server = hyper::server::Server::bind(&addr).serve(make_svc);


//	block_on(run_server(url, addr))

//    //block_on(server()).unwrap();
//    let svc = ServiceBuilder::new()
//        // Reject the request early if concurrency limit is hit
//        .load_shed()
//        // Only allow 1,000 requests in flight at a time
//        .concurrency_limit(1_000)
//        // Cancel requests that hang too long
//        .timeout(Duration::from_secs(5))
//        //
//        .resource(server)
//        //
//        .run();
}


//use {
//	hyper::{
//		// Miscellaneous types from Hyper for working with HTTP.
//		Body, Client, Request, Response, Server, Uri,
//
//		// This function turns a closure which returns a future into an
//		// implementation of the the Hyper `Service` trait, which is an
//		// asynchronous function from a generic `Request` to a `Response`.
//		service::service_fn,
//
//		// A function which runs a future to completion using the Hyper runtime.
//		//rt::run,
//	},
////    futures::{
////        // Extension trait for futures 0.1 futures, adding the `.compat()` method
////        // which allows us to use `.await` on 0.1 futures.
////        compat::Future01CompatExt,
////        // Extension traits providing additional methods on futures.
////        // `FutureExt` adds methods that work for all futures, whereas
////        // `TryFutureExt` adds methods to futures that return `Result` types.
////        future::{FutureExt, TryFutureExt},
////    },
//	std::net::SocketAddr,
//};
//
//async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//	// Always return successfully with a response containing a body with
//	// a friendly greeting ;)
//	Ok(Response::new(Body::from("hello, world!")))
//}
//
//async fn run_server(addr: SocketAddr) {
//	println!("Listening on http://{}", addr);
//
//	// Create a server bound on the provided address
//	let serve_future = Server::bind(&addr)
//		// Serve requests using our `async serve_req` function.
//		// `serve` takes a closure which returns a type implementing the
//		// `Service` trait. `service_fn` returns a value implementing the
//		// `Service` trait, and accepts a closure which goes from request
//		// to a future of the response. To use our `serve_req` function with
//		// Hyper, we have to box it and put it in a compatibility
//		// wrapper to go from a futures 0.3 future (the kind returned by
//		// `async fn`) to a futures 0.1 future (the kind used by Hyper).
//		.serve(|| service_fn(|req| serve_req(req).boxed().compat()));
//
//	// Wait for the server to complete serving or exit with an error.
//	// If an error occurred, print it to stderr.
//	if let Err(e) = serve_future.compat().await {
//		eprintln!("server error: {}", e);
//	}
//}
//
//fn main() {
//	// Set the address to run our socket on.
//	let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
//
//	// Call our `run_server` function, which returns a future.
//	// As with every `async fn`, for `run_server` to do anything,
//	// the returned future needs to be run. Additionally,
//	// we need to convert the returned future from a futures 0.3 future into a
//	// futures 0.1 future.
//	let futures_03_future = run_server(addr);
//	let futures_01_future = futures_03_future.unit_error().boxed().compat();
//
//	// Finally, we can run the future to completion using the `run` function
//	// provided by Hyper.
//	run(futures_01_future);
//}

//TODO @mark: https://rust-lang.github.io/async-book/01_getting_started/05_http_server_example.html
//TODO @mark: https://stackoverflow.com/questions/58505000/how-to-write-a-simple-rust-asynchronous-proxy-using-futures-0-3-and-hyper-0-1
//TODO @mark: https://seanmonstar.com/post/187493499882/hyper-alpha-supports-asyncawait -- client
//TODO @mark: something for form validation?
