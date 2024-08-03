use crate::package::argument_construction::ArgumentConstruction;
use crate::package::argument_passing::ArgumentPassing;
use crate::package::object_description::ObjectDescription;
use crate::package::type_description::TypeDescription;
use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Argument {
    #[serde(rename = "type")]
    pub arg_type: Box<TypeDescription>,
    pub name: String,
    pub passing: ArgumentPassing,
    pub construction: ArgumentConstruction,
}

impl Argument {
    pub fn new_change_observer_arg() -> Self {
        Self {
            arg_type: Box::new(TypeDescription::Type {
                name: "()".to_string(),
                type_parameters: None,
            }),
            name: "change_observer".to_string(),
            passing: ArgumentPassing::Clone,
            construction: ArgumentConstruction::ExistingObject(),
        }
    }

    pub fn new_context_arg() -> Self {
        Self {
            arg_type: Box::new(TypeDescription::Type {
                name: "()".to_string(),
                type_parameters: None,
            }),
            name: "context".to_string(),
            passing: ArgumentPassing::Clone,
            construction: ArgumentConstruction::ExistingObject(),
        }
    }
    pub fn emit_prefix_code(&self) -> String {
        match self.passing {
            ArgumentPassing::Move => "".to_string(),
            ArgumentPassing::Clone => "".to_string(),
            ArgumentPassing::MutableReference => "&mut ".to_string(),
            ArgumentPassing::Reference => "&".to_string(),
        }
    }

    pub fn emit_postfix_code(&self) -> String {
        match self.passing {
            ArgumentPassing::Move => "".to_string(),
            ArgumentPassing::Clone => ".clone()".to_string(),
            ArgumentPassing::MutableReference => "".to_string(),
            ArgumentPassing::Reference => "".to_string(),
        }
    }

    pub fn into_object_description(
        &self,
        type_name: &String,
        type_parameter_part: &String,
    ) -> ObjectDescription {
        ObjectDescription {
            type_name: type_name.clone(),
            type_parameter_part: type_parameter_part.clone(),
            //type_parameters: HashMap::new(),
            name: self.name.clone(),
            is_mutable: if let ArgumentPassing::MutableReference { .. } = self.passing {
                true
            } else {
                false
            },
        }
    }
}
