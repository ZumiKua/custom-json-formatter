use std::collections::HashMap;
use serde::{Deserialize};


#[derive(Deserialize, Eq, PartialEq, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
enum Type {
    Object, Array, String, Number, Integer, Boolean, Null
}


#[derive(Deserialize, Eq, PartialEq, Debug)]
struct TypeDefinition {
    #[serde(rename = "type")]
    value_type: Type,
    properties: Option<HashMap<String, TypeDefinition>>,
    prefix: Option<String>,
    suffix: Option<String>,
    #[serde(rename = "key-prefix")]
    key_prefix: Option<String>,
    #[serde(rename = "comma-suffix")]
    comma_suffix: Option<String>,
}

fn parse_type_definition(a: &str) -> serde_json::Result<TypeDefinition> {
    serde_json::from_str(a)
}

fn is_match(v: &serde_json::Value, value_type: Type) -> bool {
    match value_type {
        Type::Object => {v.is_object()}
        Type::Array => {v.is_array()}
        Type::String => {v.is_string()}
        Type::Number => {v.is_number()}
        Type::Integer => {v.is_i64()}
        Type::Boolean => {v.is_boolean()}
        Type::Null => {v.is_null()}
    }
}

fn custom_format_json_recursively(v: &serde_json::Value, output: &mut String, type_definition: &TypeDefinition) {
    if !is_match(v, type_definition.value_type) {
        output.push_str(v.to_string().as_str());
        //todo log type mismatch.
        return;
    }
    if let Some(ref str) = type_definition.prefix {
        output.push_str(str);
    }
    if let Some(map) = v.as_object() {
        if let Some(ref properties) = type_definition.properties {
            output.push('{');
            map.iter().enumerate().for_each(|(i, (key, value))| {
                if let Some(tdv) = properties.get(key) {
                    if let Some(ref key_prefix) = tdv.key_prefix {
                        output.push_str(key_prefix);
                    }
                    output.push('\"');
                    output.push_str(key);
                    output.push_str("\":");
                    custom_format_json_recursively(value, output, tdv);
                    if i != map.len() - 1 {
                        output.push(',');
                    }
                    if let Some(ref comma_suffix) = tdv.comma_suffix {
                        output.push_str(comma_suffix);
                    }
                }
                else {
                    //can not find key in properties.
                    output.push('\"');
                    output.push_str(key);
                    output.push_str("\":");
                    output.push_str(value.to_string().as_str());
                    if i != map.len() - 1 {
                        output.push(',');
                    }
                }
            });
            output.push('}');
        }
        else {
            output.push_str(v.to_string().as_str());
        }
    }
    //todo handle array
    else {
        output.push_str(v.to_string().as_str());
    }
    if let Some(ref str) = type_definition.suffix {
        output.push_str(str);
    }
}

fn custom_format_json(a: &str, type_definition: &TypeDefinition) -> serde_json::Result<String> {
    let json = serde_json::from_str(a)?;
    let mut ret = String::new();
    custom_format_json_recursively(&json, &mut ret, type_definition);
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{custom_format_json, parse_type_definition, TypeDefinition};
    use crate::Type::{Number, Object};

    #[test]
    fn test_deserialize() -> serde_json::Result<()> {
        let json = r#"
        {
  "type": "object",
  "properties": {
    "key": {
      "type": "number",
      "prefix": "\n",
      "suffix": "\n",
      "key-prefix": "\t",
      "comma-suffix": "123"
    }
  }
}
        "#;
        let obj = parse_type_definition(json)?;
        let expect = TypeDefinition {
            value_type: Object,
            properties: Some(HashMap::from([
                ("key".to_string(), TypeDefinition {
                    value_type: Number,
                    prefix: Some("\n".to_string()),
                    suffix: Some("\n".to_string()),
                    properties: None,
                    key_prefix: Some("\t".to_string()),
                    comma_suffix: Some("123".to_string())
                })
            ])),
            prefix: None,
            suffix: None,
            key_prefix: None,
            comma_suffix: None
        };
        assert_eq!(obj, expect);
        Ok(())
    }

    #[test]
    fn test_custom_format_json() -> serde_json::Result<()> {
        let td = TypeDefinition {
            value_type: Object,
            properties: Some(HashMap::from([
                ("key".to_string(), TypeDefinition {
                    value_type: Number,
                    properties: None,
                    prefix: Some(" ".to_string()),
                    suffix: Some(" ".to_string()),
                    key_prefix: Some("\n".to_string()),
                    comma_suffix: Some("\n".to_string())
                })
            ])),
            prefix: None,
            suffix: None,
            key_prefix: None,
            comma_suffix: None
        };
        let result = custom_format_json(r#"{"foo":{},"key":1,"bar":[1,2,3]}"#, &td)?;
        assert_eq!(result, r#"{"foo":{},
"key": 1 ,
"bar":[1,2,3]}"#);
        Ok(())
    }
}