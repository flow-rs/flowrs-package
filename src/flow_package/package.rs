pub mod argument;
pub mod argument_construction;
pub mod argument_passing;
pub mod constructor;
pub mod input;
pub mod modifier;
pub mod module;
pub mod namespace;
pub mod object_description;
pub mod output;
pub mod package;
pub mod package_crate;
pub mod package_type;
pub mod type_description;
pub mod type_parameter;

///================================================================================================
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::flow_package::{
        package::{namespace::Namespace, object_description::ObjectDescription, package::Package},
        package_manager::PackageManager,
    };

    const PACKAGE_JSON: &str = r#"
{
    "name": "my_package",
    "version": "1.0.0",
    "crates": {
    "my_crate": {
        "types": {
        "MyType": {
            "inputs": null,
            "outputs": null,
            "type_parameters": [{"name": "U", "where": []}, {"name": "T", "where": []}],
            "constructors":{
                "New":{"NewWithObserver": {}},
                "FromCode":{"FromCode":{"code_template": "let {{fully_qualified_name}}:{{type_parameter_U}} = 5;"}}
            }
        }
        },
        "modules": {}
    }
    }
}
        "#;

    #[test]
    fn test() {
        let package_1: Package = serde_json::from_str(&PACKAGE_JSON).expect("wrong format.");
        let mut pm_1 = PackageManager::new();
        pm_1.add_package(package_1);
        let t_1 = pm_1.get_type("my_crate::MyType").expect("msg");
        let c_1 = t_1.constructors.get("FromCode").expect("");
        let mut type_params_1 = HashMap::new();
        type_params_1.insert("U".to_string(), "i32".to_string());
        type_params_1.insert("T".to_string(), "i32".to_string());
        let ns_1 = Namespace::new();

        let obj_1 = ObjectDescription {
            type_name: "my_crate::MyType".to_string(),
            type_parameter_part: "".to_string(),
            name: "value".to_string(),
            is_mutable: false,
        };
        println!(
            "CODE: {}",
            c_1.emit_code_template(&obj_1, &type_params_1, &pm_1, &ns_1)
                .expect("")
        );
    }
}
