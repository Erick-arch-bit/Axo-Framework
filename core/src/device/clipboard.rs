use std::sync::Mutex;

static CLIPBOARD: Mutex<Option<String>> = Mutex::new(None);

#[derive(Debug, Clone)]
pub struct Clipboard;

impl Clipboard {
    pub fn set(text: &str) {
        if let Ok(mut clip) = CLIPBOARD.lock() {
            *clip = Some(text.to_string());
        }
        println!("[Clipboard] Set: {}", text);
    }

    pub fn get() -> Option<String> {
        if let Ok(clip) = CLIPBOARD.lock() {
            return clip.clone();
        }
        None
    }

    pub fn has_text() -> bool {
        Self::get().is_some()
    }

    pub fn clear() {
        if let Ok(mut clip) = CLIPBOARD.lock() {
            *clip = None;
        }
    }
}
