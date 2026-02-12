// src/utils/mod_test.rs
use super::*; // importa todas las funciones públicas de mod.rs

#[test]
fn test_capitalize() {
    assert_eq!(capitalize("hash"), "Hash");
}

#[test]
fn test_render_markdown() {
    let text = "Hello";
    let rendered = render_markdown(text);
    assert!(rendered.contains("Hello"));
}
