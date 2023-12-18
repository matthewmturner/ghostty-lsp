//! A minimal example LSP server that can only respond to the `gotoDefinition` request. To use
//! this example, execute it and then send an `initialize` request.
//!
//! ```no_run
//! Content-Length: 85
//!
//! {"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {"capabilities": {}}}
//! ```
//!
//! This will respond with a server response. Then send it a `initialized` notification which will
//! have no response.
//!
//! ```no_run
//! Content-Length: 59
//!
//! {"jsonrpc": "2.0", "method": "initialized", "params": {}}
//! ```
//!
//! Once these two are sent, then we enter the main loop of the server. The only request this
//! example can handle is `gotoDefinition`:
//!
//! ```no_run
//! Content-Length: 159
//!
//! {"jsonrpc": "2.0", "method": "textDocument/definition", "id": 2, "params": {"textDocument": {"uri": "file://temp"}, "position": {"line": 1, "character": 1}}}
//! ```
//!
//! To finish up without errors, send a shutdown request:
//!
//! ```no_run
//! Content-Length: 67
//!
//! {"jsonrpc": "2.0", "method": "shutdown", "id": 3, "params": null}
//! ```
//!
//! The server will exit the main loop and finally we send a `shutdown` notification to stop
//! the server.
//!
//! ```
//! Content-Length: 54
//!
//! {"jsonrpc": "2.0", "method": "exit", "params": null}
//! ```
use std::error::Error;
use std::io::BufRead;

use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::OneOf;
use lsp_types::{
    notification, notification::DidChangeTextDocument, notification::DidOpenTextDocument,
    notification::Notification as NotificationTrait, request::Completion, request::HoverRequest,
    request::Request as RequestTrait, CompletionResponse, Hover, HoverProviderCapability,
    InitializeParams, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};

use ghostty_lsp::get_config_param_description;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("Starting server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        definition_provider: Some(OneOf::Left(true)),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: None,
            work_done_progress_options: Default::default(),
            all_commit_characters: None,
            completion_item: None,
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })
    .unwrap();
    eprintln!("Sending server capabilities: {server_capabilities:?}");
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}

fn handle_request(req: Request) -> Option<Response> {
    match req.method.as_str() {
        Completion::METHOD => {
            eprintln!("Got completion request");
            let (id, _params) = cast_request::<Completion>(req).unwrap();
            let result = Some(CompletionResponse::Array(Vec::new()));
            let result = serde_json::to_value(&result).unwrap();
            let resp = Response {
                id,
                result: Some(result),
                error: None,
            };
            Some(resp)
        }
        HoverRequest::METHOD => {
            eprintln!("Got hover request");
            let (id, params) = cast_request::<HoverRequest>(req).unwrap();
            // read the file at params.text_document_position_params.text_document.uri
            // and return the contents as a hover
            let uri = params.text_document_position_params.text_document.uri;
            eprintln!("Got uri: {:?}", uri);
            let file = std::fs::File::open(uri.path()).unwrap();
            let buf_reader = std::io::BufReader::new(file);
            let hover_line = params.text_document_position_params.position.line;

            let mut hover_contents: Option<String> = None;
            let mut line_num = 0;
            for line in buf_reader.lines() {
                if line_num == hover_line {
                    let line_contents = line.unwrap();
                    if let Some(_) = line_contents.find("=") {
                        let param_name = line_contents.split("=").next().unwrap().trim();
                        eprintln!("Found param name: {:?}", param_name);
                        let param_desc = get_config_param_description(param_name);
                        hover_contents = Some(param_desc)
                    }
                    // eprintln!("Found line: {:?}", line.unwrap());
                }
                line_num += 1;
            }

            let cont = match hover_contents {
                Some(val) => val,
                None => "No hover contents found".to_string(),
            };
            let hover = Hover {
                contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(
                    cont.to_string(),
                )),
                range: None,
            };
            let result = serde_json::to_value(&hover).unwrap();
            eprintln!("Sending hover response: {result:?}");
            let resp = Response {
                id,
                result: Some(result),
                error: None,
            };
            Some(resp)
        }
        _ => None,
    }
}

fn handle_notification(notif: Notification, doc: &mut String) {
    match notif.method.as_str() {
        DidOpenTextDocument::METHOD => {
            eprintln!("Got DidOpenTextDocument notification");
            let (_id, params) =
                cast_notification::<notification::DidOpenTextDocument>(notif).unwrap();
            *doc = params.text_document.text.clone();
            eprintln!("Got text: {:?}", params.text_document.text.as_str());
        }
        DidChangeTextDocument::METHOD => {
            eprintln!("Got DidChangeTextDocument notification");
            let (_id, params) =
                cast_notification::<notification::DidChangeTextDocument>(notif).unwrap();
            params.content_changes.iter().for_each(|change| {
                eprintln!("Got change: {change:?}");
            });
            eprintln!("Got params: {params:?}");
        }
        _ => {}
    }
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    let mut doc: String = String::new();
    eprintln!("Starting main loop");
    for msg in &connection.receiver {
        eprintln!("Got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("Got request: {req:?}");
                if let Some(res) = handle_request(req) {
                    connection.sender.send(Message::Response(res))?;
                }
            }
            Message::Response(resp) => {
                eprintln!("Got response: {resp:?}");
            }
            Message::Notification(notif) => {
                eprintln!("Got notification: {notif:?}");
                handle_notification(notif, &mut doc);
            }
        }
    }
    Ok(())
}

fn cast_request<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

fn cast_notification<N>(notif: Notification) -> Result<((), N::Params), ExtractError<Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    notif.extract(N::METHOD)
}
