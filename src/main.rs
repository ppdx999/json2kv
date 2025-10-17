use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{self, Read};
use std::process;

fn main() {
    // 標準入力からJSONを読み取る
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    }

    // JSONをパース
    let json: Value = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            process::exit(1);
        }
    };

    // JSONをKeyValue形式に変換
    let mut kv_map = BTreeMap::new();
    if let Err(e) = flatten_json(&json, String::new(), &mut kv_map) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    // ソートされた順序で出力（BTreeMapは自動的にソートされる）
    for (key, value) in kv_map {
        println!("{} {}", key, value);
    }
}

fn flatten_json(
    value: &Value,
    prefix: String,
    result: &mut BTreeMap<String, String>,
) -> Result<(), String> {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let sanitized_key = sanitize_key(key)?;
                let new_prefix = if prefix.is_empty() {
                    sanitized_key
                } else {
                    format!("{}.{}", prefix, sanitized_key)
                };
                flatten_json(val, new_prefix, result)?;
            }
        }
        Value::Array(arr) => {
            for (index, val) in arr.iter().enumerate() {
                let new_prefix = if prefix.is_empty() {
                    index.to_string()
                } else {
                    format!("{}.{}", prefix, index)
                };
                flatten_json(val, new_prefix, result)?;
            }
        }
        Value::Null => {
            let sanitized_value = "null".to_string();
            result.insert(prefix, sanitized_value);
        }
        Value::Bool(b) => {
            let sanitized_value = b.to_string();
            result.insert(prefix, sanitized_value);
        }
        Value::Number(n) => {
            let sanitized_value = n.to_string();
            result.insert(prefix, sanitized_value);
        }
        Value::String(s) => {
            let sanitized_value = sanitize_value(s)?;
            result.insert(prefix, sanitized_value);
        }
    }
    Ok(())
}

fn sanitize_key(key: &str) -> Result<String, String> {
    // keyに改行が含まれているかチェック
    if key.contains('\n') || key.contains('\r') {
        return Err(format!("Key contains newline character: {}", key));
    }

    // ヌル文字のチェック
    if key.contains('\0') {
        return Err("Key contains null character".to_string());
    }

    // 空白とタブをハイフンに変換
    Ok(key.replace(' ', "-").replace('\t', "-"))
}

fn sanitize_value(value: &str) -> Result<String, String> {
    // ヌル文字のチェック
    if value.contains('\0') {
        return Err("Value contains null character".to_string());
    }

    // 改行を正規化してエスケープ
    // \r\n → \n → \\n
    // \r → \n → \\n
    // \n → \\n
    let normalized = value.replace("\r\n", "\n").replace('\r', "\n");
    let escaped = normalized.replace('\n', "\\n");

    Ok(escaped)
}
