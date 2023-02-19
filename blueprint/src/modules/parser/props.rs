use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
  static ref SCRIPT_PATTERN: Regex = Regex::new(r#"<script([ \n]+lang=['"]ts['"])?([ \n]*)?>([^#]*)</script>"#).unwrap();
  static ref PROPS_PATTERN: Regex = Regex::new(r"export let (.+)\n+").unwrap();
}

/*
* Structure to define data type of a component props
*/
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
  String,
  Number,
  Array,
  Object,
  Custom(String),
  Boolean,
  Function,
  Unknown
}

impl std::fmt::Display for DataType {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    let content = match self {
      Self::Custom(detail) =>  format!("{}", detail),
      Self::String => format!("string"),
      Self::Number => format!("number"),
      Self::Array => format!("array"),
      Self::Object => format!("object"),
      Self::Boolean => format!("boolean"),
      Self::Function => format!("function"),
      Self::Unknown => format!("unknown")
    };

    write!(formatter, "{}", content)
  }
}

/*
* Structure to mantain properties of a component
*/
#[derive(Debug, Clone)]
pub struct ComponentProp {
  pub name: String,
  pub data_type: DataType,
  pub default: Option<String>,
  pub description: Option<String>
}


pub fn parse(content: &String) -> Option<Vec<ComponentProp>> {
  match SCRIPT_PATTERN.captures(content) {
    Some(cap) => {
      let content = &cap[3].to_string();
      let mut props: Vec<ComponentProp> = vec![];

      // Every cycle is a property 
      for cap in PROPS_PATTERN.captures_iter(content) {

        let mut property = &cap[1];
        // Variables for create the Componet property struct
        let mut name = "";
        let mut data_type = DataType::Unknown;
        let mut default: Option<String> = None;
        let mut description: Option<String> = None;
        let mut last_index = property.len();

        // check for comments (description)
        if let Some(position) = property.find("//") {
          last_index = position;
          if let Some(comment) = property.get(position+2..) {
            description = Some(format!("{}", comment.trim()));
          }
        };

        if let Some(sub) = property.get(..last_index) {
          property = sub;
        }

        // check for default value
        if let Some(position) = property.find("=") {
          if let Some(def) = property.get(position+1..) {
            default = Some(format!("{}", def.trim()));
          }
          last_index = position;
        }

        if let Some(sub) = property.get(..last_index) {
          property = sub;
        }

        // check for property type
        if let Some(position) = property.find(":") {
          // If it is specified in ts
          if let Some(def) = property.get(position+1..last_index) {
            let dt = def.trim();

            if dt == "string".to_string() { 
              data_type = DataType::String
            } else if dt == "number".to_string() { 
              data_type = DataType::Number
            } else if dt == "boolean".to_string() { 
              data_type = DataType::Boolean
            } else {
              data_type = DataType::Custom(format!("{}", dt))
            }
          }
          last_index = position;
        } else {
          // If it is not specified in ts check by Defaut value
          if let Some(default_value) = default.clone() {
            let first_char = default_value.get(0..1);

            if first_char == Some("\"") || first_char == Some("'") {
              data_type = DataType::String
            } else if let Ok(_) = default_value.parse::<i128>() {
              data_type = DataType::Number
            } else if let Ok(_) = default_value.parse::<f64>() {
              data_type = DataType::Number
            } else  if first_char == Some("{") {
              data_type = DataType::Object
            } else  if first_char == Some("[") {
              data_type = DataType::Array
            } else if default_value == "true" || default_value == "false" {
              data_type = DataType::Boolean
            }
          }
        }

        // check for name
        if let Some(nam) = property.get(..last_index) {
          name = nam.trim()
        }

        let prop = ComponentProp {
          name: String::from(name),
          data_type,
          default: default,
          description: description
        };

        props.push(prop.clone());
      }

      Some(props)
    }, 
    None => {
      return None
    }
  }
}

// Run the following tests with cargo test modules::parser::props

#[cfg(test)]
mod test_props {
  use super::*;

  #[test]
  fn test_typescript_text() {
    let text = format!("{}", r#"
<script  lang="ts" >

  export let text = 'Button' // 1 Variable del texto en el botón
  export let text : string = 'Button' //2 Variable del texto en el botón

  export let index = 5 // 1 Número de botón
  export let index2 = 5.4 // 2 Número de botón
  export let index3: number = 5 // 3 Número de botón

  export let data = {}
  export let data: CustomType = {}

  export let flag: boolean
  export let flag = false
  export let flag = true // bandera, bandera de méxico

  export let options = [] // Opciones para el componente
  export let options: Array<bool> = [] 

  export let shadow // Sombra del componente
  export let typee: "some" |  "past" = "past"

</script>
    "#);

    let desc = parse(&text).unwrap();
    println!("{:?}", desc);
    
    let property = desc.get(0).unwrap();
    assert_eq!(property.name, String::from("text"));
    assert_eq!(property.data_type, DataType::String);
    assert_eq!(property.default, Some(String::from("'Button'")));
    assert_eq!(property.description, Some(String::from("1 Variable del texto en el botón")));

    let property = desc.get(1).unwrap();
    assert_eq!(property.name, String::from("text"));
    assert_eq!(property.data_type, DataType::String);
    assert_eq!(property.default, Some(String::from("'Button'")));
    assert_eq!(property.description, Some(String::from("2 Variable del texto en el botón")));

    let property = desc.get(2).unwrap();
    assert_eq!(property.name, String::from("index"));
    assert_eq!(property.data_type, DataType::Number);
    assert_eq!(property.default, Some(String::from("5")));
    assert_eq!(property.description, Some(String::from("1 Número de botón")));

    let property = desc.get(3).unwrap();
    assert_eq!(property.name, String::from("index2"));
    assert_eq!(property.data_type, DataType::Number);
    assert_eq!(property.default, Some(String::from("5.4")));
    assert_eq!(property.description, Some(String::from("2 Número de botón")));

    let property = desc.get(4).unwrap();
    assert_eq!(property.name, String::from("index3"));
    assert_eq!(property.data_type, DataType::Number);
    assert_eq!(property.default, Some(String::from("5")));
    assert_eq!(property.description, Some(String::from("3 Número de botón")));

    let property = desc.get(5).unwrap();
    assert_eq!(property.name, String::from("data"));
    assert_eq!(property.data_type, DataType::Object);
    assert_eq!(property.default, Some(String::from("{}")));
    assert_eq!(property.description, None);

    let property = desc.get(6).unwrap();
    assert_eq!(property.name, String::from("data"));
    assert_eq!(property.data_type, DataType::Custom(String::from("CustomType")));
    assert_eq!(property.default, Some(String::from("{}")));
    assert_eq!(property.description, None);

    let property = desc.get(7).unwrap();
    assert_eq!(property.name, String::from("flag"));
    assert_eq!(property.data_type, DataType::Boolean);
    assert_eq!(property.default, None);
    assert_eq!(property.description, None);

    let property = desc.get(8).unwrap();
    assert_eq!(property.name, String::from("flag"));
    assert_eq!(property.data_type, DataType::Boolean);
    assert_eq!(property.default, Some(String::from("false")));
    assert_eq!(property.description, None);

    let property = desc.get(9).unwrap();
    assert_eq!(property.name, String::from("flag"));
    assert_eq!(property.data_type, DataType::Boolean);
    assert_eq!(property.default, Some(String::from("true")));
    assert_eq!(property.description, Some(String::from("bandera, bandera de méxico")));

    let property = desc.get(10).unwrap();
    assert_eq!(property.name, String::from("options"));
    assert_eq!(property.data_type, DataType::Array);
    assert_eq!(property.default, Some(String::from("[]")));
    assert_eq!(property.description, Some(String::from("Opciones para el componente")));

    let property = desc.get(11).unwrap();
    assert_eq!(property.name, String::from("options"));
    assert_eq!(property.data_type, DataType::Custom(String::from("Array<bool>")));
    assert_eq!(property.default, Some(String::from("[]")));
    assert_eq!(property.description, None);

    let property = desc.get(12).unwrap();
    assert_eq!(property.name, String::from("shadow"));
    assert_eq!(property.data_type, DataType::Unknown);
    assert_eq!(property.default, None);
    assert_eq!(property.description, Some(String::from("Sombra del componente")));

    let property = desc.get(13).unwrap();
    assert_eq!(property.name, String::from("typee"));
    assert_eq!(property.data_type, DataType::Custom(String::from("\"some\" |  \"past\"")));
    assert_eq!(property.default, Some(String::from("\"past\"")));
    assert_eq!(property.description, None);

  }

}
