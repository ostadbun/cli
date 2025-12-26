use std::sync::mpsc;
use std::thread;
use tiny_http::{Response, Server};
use url::Url;
use webbrowser;

pub fn run() {
    let auth_url = "https://api.ostadban.tech/user/auth?provider=google";

    let (tx, rx) = mpsc::channel();
    let server_thread = thread::spawn(move || {
        let server = Server::http("127.0.0.1:8000").unwrap();
        if let Ok(request) = server.recv() {
            let query = request.url().split('?').nth(1).unwrap_or("").to_string();
            let params: Vec<(String, String)> = Url::parse(&format!("http://localhost/?{}", query))
                .unwrap()
                .query_pairs()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            tx.send(params).unwrap();

            let response = Response::from_string("Login successful! You can close this tab.");
            let _ = request.respond(response);
        }
    });

    webbrowser::open(auth_url).unwrap();
    println!("Browser opened for login...");

    let params = rx.recv().unwrap();
    println!("Received params from redirect: {:?}", params);

    server_thread.join().unwrap();
}
