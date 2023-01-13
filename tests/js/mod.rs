use anyhow::Result;
use std::fs::{create_dir_all, remove_dir_all, remove_file};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use instant::Duration;

use crate::_util::wait_for_localhost_port;

pub fn cleanup() {
    if Path::new("tests/js/node_modules").exists() {
        remove_dir_all("tests/js/node_modules").expect("Unable to run rm to delete node_modules");
    }
    if Path::new("tests/js/work").exists() {
        remove_dir_all("tests/js/work").expect("Unable to run rm to delete work");
    }
    if Path::new("tests/js/package-lock.json").exists() {
        remove_file("tests/js/package-lock.json")
            .expect("Unable to run rm to delete package-lock.json");
    }
}

pub fn install() {
    let status = Command::new("npm")
        .current_dir("tests/js")
        .args(&["install"])
        .status()
        .expect("Unable to run npm install");
    assert_eq!(
        Some(0),
        status.code(),
        "npm install did not run successfully. Do you have npm installed and a network connection?"
    );
}

pub fn prepare_test_set(test_set: &str) -> (String, String, String) {
    let path_result = format!("tests/js/work/{}/result.txt", test_set);
    let path_writer = format!("tests/js/work/{}/writer", test_set);
    let path_reader = format!("tests/js/work/{}/reader", test_set);
    create_dir_all(&path_writer).expect("Unable to create work writer directory");
    create_dir_all(&path_reader).expect("Unable to create work reader directory");
    (path_result, path_writer, path_reader)
}

pub struct JavascriptServer {
    handle: Option<async_std::task::JoinHandle<()>>,
}

impl JavascriptServer {
    pub fn new() -> JavascriptServer {
        JavascriptServer {
            handle: None,
        }
    }

    pub async fn run(
        &mut self,
        is_writer: bool,
        port: u32,
        data_count: usize,
        data_size: usize,
        data_char: char,
        test_set: String,
    ) {
        self.handle = Some(async_std::task::spawn(async move {
            // This sometimes fails on OSX immediately with unix signal 4, let's retry a few times
            let mut retries = 3;
            let mut code: Option<i32> = None;
            while code.is_none() && retries > 0 {
                let status = async_std::process::Command::new("node")
                    .current_dir("tests/js")
                    .args(&[
                        "interop.js",
                        "server",
                        if is_writer { "writer" } else { "reader" },
                        &port.to_string(),
                        &data_count.to_string(),
                        &data_size.to_string(),
                        &data_char.to_string(),
                        &test_set,
                    ])
                    .kill_on_drop(true)
                    .status()
                    .await
                    .expect("Unable to execute node");
                code = status.code();
                if code.is_none() {
                    async_std::task::sleep(Duration::from_millis(100)).await;
                    retries -= 1;
                }
            }

            assert_eq!(
                Some(0),
                code,
                "node server did not exit successfully, is_writer={}, port={}, data_count={}, data_size={}, data_char={}, test_set={}",
                is_writer,
                port,
                data_count,
                data_size,
                data_char,
                test_set,
            );
        }));
        wait_for_localhost_port(port).await;
    }
}

impl Drop for JavascriptServer {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
           async_std::task::block_on(handle.cancel());
        }
    }
}

pub async fn js_start_server(
    is_writer: bool,
    port: u32,
    data_count: usize,
    data_size: usize,
    data_char: char,
    test_set: String,
) -> Result<JavascriptServer> {
    let mut server = JavascriptServer::new();
    server.run(is_writer, port, data_count, data_size, data_char, test_set).await;
    Ok(server)
}

pub async fn js_run_client(
    is_writer: bool,
    port: u32,
    data_count: usize,
    data_size: usize,
    data_char: char,
    test_set: &str,
) {
    let status = async_std::process::Command::new("node")
        .current_dir("tests/js")
        .args(&[
            "interop.js",
            "client",
            if is_writer { "writer" } else { "reader" },
            &port.to_string(),
            &data_count.to_string(),
            &data_size.to_string(),
            &data_char.to_string(),
            test_set,
        ])
        .kill_on_drop(true)
        .status()
        .await
        .expect("Unable to execute node");
    assert_eq!(
        Some(0),
        status.code(),
        "node client did not run successfully, is_writer={}, port={}, data_count={}, data_size={}, data_char={}, test_set={}",
        is_writer, 
        port,
        data_count,
        data_size,
        data_char,
        test_set
    );
}