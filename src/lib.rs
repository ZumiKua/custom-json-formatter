use std::collections::HashMap;
use serde::{Deserialize};


#[derive(Deserialize, Eq, PartialEq, Debug)]
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
    suffix: Option<String>
}

fn parse_type_definition(a: &str) -> serde_json::Result<TypeDefinition> {
    serde_json::from_str(a)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{parse_type_definition, TypeDefinition};
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
      "suffix": "\n"
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
                    properties: None
                })
            ])),
            prefix: None,
            suffix: None
        };
        assert_eq!(obj, expect);
        Ok(())
    }
}