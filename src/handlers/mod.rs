use lsp_server::{ExtractError, Notification, Request, RequestId, Response};
use lsp_types::notification::Notification as NotificationTrait;
use lsp_types::notification::{DidChangeTextDocument, DidOpenTextDocument};
use lsp_types::request::{Completion, HoverRequest, Request as RequestTrait};
use lsp_types::{notification, CompletionResponse, Hover};

use crate::definitions::get_config_param_description;

fn cast_notification<N>(notif: Notification) -> Result<N::Params, ExtractError<Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    notif.extract(N::METHOD)
}

fn cast_request<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

pub fn handle_notification(notif: Notification, doc: &mut String) {
    match notif.method.as_str() {
        DidOpenTextDocument::METHOD => {
            eprintln!("Got DidOpenTextDocument notification");
            let params = cast_notification::<notification::DidOpenTextDocument>(notif).unwrap();
            *doc = params.text_document.text.clone();
            eprintln!("Got text: {:?}", params.text_document.text.as_str());
        }
        DidChangeTextDocument::METHOD => {
            eprintln!("Got DidChangeTextDocument notification");
            let params = cast_notification::<notification::DidChangeTextDocument>(notif).unwrap();
            params.content_changes.iter().for_each(|change| {
                if change.range.is_none() {
                    eprintln!("No range, updating full text");
                    *doc = change.text.clone();
                } else {
                    // TODO: Implement range updates if we change to
                    // incremental textDocument updates
                }
            });
            eprintln!("Got params: {params:?}");
        }
        _ => {}
    }
}

pub fn handle_request(req: Request, doc: &mut String) -> Option<Response> {
    match req.method.as_str() {
        Completion::METHOD => {
            eprintln!("Got completion request");
            let (id, _params) = cast_request::<Completion>(req).unwrap();
            let result = Some(CompletionResponse::Array(Vec::new()));
            let result = serde_json::to_value(result).unwrap();
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
            let maybe_hover_line =
                usize::try_from(params.text_document_position_params.position.line);

            let hover_line = match maybe_hover_line {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Failed to convert hover line to usize");
                    return None;
                }
            };

            let mut hover_contents: Option<String> = None;
            for (line_num, line) in doc.lines().enumerate() {
                if line_num == hover_line && line.find('=').is_some() {
                    let param_name = line.split('=').next().unwrap().trim();
                    eprintln!("Found param name: {:?}", param_name);
                    let param_desc = get_config_param_description(param_name);
                    hover_contents = Some(param_desc)
                }
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
            let result = serde_json::to_value(hover).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_server::Notification;
    use lsp_types::notification;

    #[test]
    fn test_cast_did_open_notification() {
        let notif = Notification {
            method: "textDocument/didOpen".to_string(),
            params: serde_json::json!({
                "textDocument": {
                    "uri": "file:///home/alex/Projects/ghostty/src/main.rs",
                    "languageId": "rust",
                    "version": 1,
                    "text": "hello world\n"
                }
            }),
        };
        let params = cast_notification::<notification::DidOpenTextDocument>(notif).unwrap();
        assert_eq!(params.text_document.text, "hello world\n",);
    }

    #[test]
    fn test_cast_did_change_notification() {
        let notif = Notification {
            method: "textDocument/didChange".to_string(),
            params: serde_json::json!({
                "textDocument": {
                    "uri": "file:///home/alex/Projects/ghostty/src/main.rs",
                    "version": 2,
                },
                "contentChanges": [
                    {
                        "range": {
                            "start": {
                                "line": 0,
                                "character": 0
                            },
                            "end": {
                                "line": 0,
                                "character": 0
                            }
                        },
                        "rangeLength": 0,
                        "text": "h"
                    }
                ]
            }),
        };
        let params = cast_notification::<notification::DidChangeTextDocument>(notif).unwrap();
        assert_eq!(params.content_changes.len(), 1);
        assert_eq!(params.content_changes[0].text, "h");
        assert_eq!(params.content_changes[0].range_length, Some(0));
        assert_eq!(params.content_changes[0].range.unwrap().start.line, 0);
        assert_eq!(params.content_changes[0].range.unwrap().start.character, 0);
        assert_eq!(params.content_changes[0].range.unwrap().end.line, 0);
        assert_eq!(params.content_changes[0].range.unwrap().end.character, 0);
    }

    #[test]
    fn test_handle_did_open_document_and_did_change_just_text() {
        let mut doc = String::new();
        let notif = Notification {
            method: "textDocument/didOpen".to_string(),
            params: serde_json::json!({
                "textDocument": {
                    "uri": "file:///home/alex/Projects/ghostty/src/main.rs",
                    "languageId": "rust",
                    "version": 1,
                    "text": "hello world\n"
                }
            }),
        };
        handle_notification(notif, &mut doc);
        assert_eq!(doc, "hello world\n");
        let notif = Notification {
            method: "textDocument/didChange".to_string(),
            params: serde_json::json!({
                "textDocument": {
                    "uri": "file:///home/alex/Projects/ghostty/src/main.rs",
                    "version": 2,
                },
                "contentChanges": [
                    {
                        "text": "h"
                    }
                ]
            }),
        };
        handle_notification(notif, &mut doc);
        assert_eq!(doc, "h");
    }

    #[test]
    fn test_single_line_handle_did_open_and_did_change() {
        let mut doc = String::new();
        let notif = Notification {
            method: "textDocument/didOpen".to_string(),
            params: serde_json::json!({
                "textDocument": {
                    "uri": "file:///home/alex/Projects/ghostty/src/main.rs",
                    "languageId": "rust",
                    "version": 1,
                    "text": "hello"
                }
            }),
        };
        handle_notification(notif, &mut doc);
        assert_eq!(doc, "hello");
        let notif = Notification {
            method: "textDocument/didChange".to_string(),
            params: serde_json::json!({
                "textDocument": {
                    "uri": "file:///home/alex/Projects/ghostty/src/main.rs",
                    "version": 2,
                },
                "contentChanges": [
                    {
                        "text": "h"
                    }
                ]
            }),
        };
        handle_notification(notif, &mut doc);
        assert_eq!(doc, "h");
    }
}
