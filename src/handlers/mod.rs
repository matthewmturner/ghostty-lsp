// Purpose: Module for handling LSP messages
use lsp_server::{ExtractError, Notification};
use lsp_types::notification;
use lsp_types::notification::Notification as NotificationTrait;
use lsp_types::notification::{DidChangeTextDocument, DidOpenTextDocument};

fn cast_notification<N>(notif: Notification) -> Result<N::Params, ExtractError<Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    notif.extract(N::METHOD)
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
    fn test_single_line_handle_did_open_and_did_change_with_prepend() {
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
                        "text": "h"
                    }
                ]
            }),
        };
        handle_notification(notif, &mut doc);
        assert_eq!(doc, "hhello");
    }

    #[test]
    fn test_single_line_handle_did_open_and_did_change_with_append() {
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
                        "range": {
                            "start": {
                                "line": 0,
                                "character": 5
                            },
                            "end": {
                                "line": 0,
                                "character": 6
                            }
                        },
                        "text": "h"
                    }
                ]
            }),
        };
        handle_notification(notif, &mut doc);
        assert_eq!(doc, "helloh");
    }
}
