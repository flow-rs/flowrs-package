use crate::package::argument::Argument;
use crate::package::argument_construction::ArgumentConstruction;
use crate::package::namespace::Namespace;
use crate::package::object_description::ObjectDescription;
use crate::package::type_description::TypeDescription;
use crate::package_manager::PackageManager;
use anyhow::{Error, Result};
use handlebars::Handlebars;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Constructor {
    New {
        function_name: Option<String>,
    },
    NewWithObserver {
        function_name: Option<String>,
    },
    NewWithObserverAndContext {
        function_name: Option<String>,
    },
    NewWithArbitraryArgs {
        function_name: Option<String>,
        arguments: Vec<Argument>,
    },
    FromJson,
    FromDefault,
    FromCode {
        code_template: String,
    },
}

impl Constructor {
    fn emit_fully_qualified_name(
        &self,
        name: &String,
        namespace: &Namespace,
        ignore: bool,
    ) -> String {
        if namespace.is_empty() || ignore {
            name.clone()
        } else {
            format!("{}_{}", namespace.to_string(), name)
        }
    }

    fn emit_args(&self, args: &Vec<Argument>, current_namespace: &Namespace) -> String {
        args.iter()
            .map(|arg| {
                format!(
                    "{}{}{}",
                    arg.emit_prefix_code(),
                    self.emit_fully_qualified_name(
                        &arg.name,
                        current_namespace,
                        matches!(arg.construction, ArgumentConstruction::ExistingObject())
                    ),
                    arg.emit_postfix_code()
                )
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn emit_arg_type_parameters_part_rec(
        &self,
        tp_part: &mut String,
        argument_type: &Box<TypeDescription>,
        resolved_type_parameters: &HashMap<String, String>,
    ) {
        let arg_type_params: &Option<Vec<Box<TypeDescription>>>;

        match argument_type.as_ref() {
            TypeDescription::Type {
                name,
                type_parameters,
            } => {
                tp_part.push_str(name);
                arg_type_params = type_parameters;
            }

            TypeDescription::Generic {
                name,
                type_parameters,
            } => {
                if let Some(tn) = resolved_type_parameters.get(name) {
                    tp_part.push_str(tn);
                } else {
                    //TODO: Error Handling
                }
                arg_type_params = type_parameters;
            }
        }

        if let Some(params) = arg_type_params {
            if !params.is_empty() {
                tp_part.push_str("<");

                for tp in params {
                    self.emit_arg_type_parameters_part_rec(tp_part, &tp, resolved_type_parameters);
                    tp_part.push_str(",")
                }

                tp_part.pop(); // pop last ,
                tp_part.push_str(">");
            }
        }
    }

    fn emit_arg_type_parameters_part(
        &self,
        arg_type_parameters: &Option<Vec<Box<TypeDescription>>>,
        type_parameters: &HashMap<String, String>,
    ) -> String {
        if let Some(arg_type_params) = arg_type_parameters {
            if !arg_type_params.is_empty() {
                let mut tp_part = "<".to_string();

                for tp in arg_type_params {
                    self.emit_arg_type_parameters_part_rec(&mut tp_part, tp, type_parameters);
                    tp_part.push_str(",")
                }

                //tp_part.pop(); // pop last ,
                tp_part.push_str(">");
                return tp_part;
            }
        }

        "".to_string()
    }

    fn emit_arg_construction_code(
        &self,
        arg: &Argument,
        arg_constructor_name: String,
        pack_man: &PackageManager,
        current_namespace: &Namespace,
        type_parameters: &HashMap<String, String>,
    ) -> Result<String, Error> {
        match arg.arg_type.as_ref() {
            TypeDescription::Type {
                name,
                type_parameters: arg_type_parameters,
            } => {
                if let Some(type_desc) = pack_man.get_type(name) {
                    if let Some(arg_constructor) = type_desc.constructors.get(&arg_constructor_name)
                    {
                        let object_desc = arg.into_object_description(
                            &name,
                            &self.emit_arg_type_parameters_part(
                                arg_type_parameters,
                                type_parameters,
                            ),
                        );

                        arg_constructor.emit_code_template(
                            &object_desc,
                            type_parameters,
                            pack_man,
                            current_namespace,
                        )
                    } else {
                        Err(Error::msg(format!(
                            "Constructor '{}' for type '{}' not found.",
                            arg_constructor_name, name
                        )))
                    }
                } else {
                    Err(Error::msg(format!(
                        "Type description for '{}' not found.",
                        name
                    )))
                }
            }

            TypeDescription::Generic {
                name,
                type_parameters: arg_type_parameters,
            } => {
                // check if generic was already resolved. if so, try to get type and emit constructor code.
                // TODO: Think about what should happen if it is not yet resolved.
                if let Some(type_name) = type_parameters.get(name) {
                    if let Some(type_desc) = pack_man.get_type(&type_name) {
                        if let Some(arg_constructor) =
                            type_desc.constructors.get(&arg_constructor_name)
                        {
                            let object_desc = arg.into_object_description(
                                &type_name,
                                &self.emit_arg_type_parameters_part(
                                    arg_type_parameters,
                                    &type_parameters,
                                ),
                            );

                            arg_constructor.emit_code_template(
                                &object_desc,
                                &type_parameters,
                                pack_man,
                                current_namespace,
                            )
                        } else {
                            Err(Error::msg(format!(
                                "Constructor '{}' for type '{}' not found.",
                                arg_constructor_name, name
                            )))
                        }
                    } else {
                        Err(Error::msg(format!(
                            "Type description for '{}' not found",
                            type_name
                        )))
                    }
                } else {
                    Err(Error::msg("Generic type was not resolved"))
                }
            }
        }
    }

    fn emit_args_construction_code(
        &self,
        pack_man: &PackageManager,
        args: &Vec<Argument>,
        current_namespace: &Namespace,
        type_parameters: &HashMap<String, String>,
    ) -> Result<String, Error> {
        let mut construction_blocks = Vec::<String>::new();

        for arg in args {
            // Only objects with a constructor need to be constructed.
            if let ArgumentConstruction::Constructor(constructor_name) = &arg.construction {
                // Generate construction for each argument.
                match self.emit_arg_construction_code(
                    arg,
                    constructor_name.clone(),
                    pack_man,
                    current_namespace,
                    type_parameters,
                ) {
                    Ok(code) => construction_blocks.push(code),
                    Err(err) => return Err(err),
                }
            }
        }

        Ok(construction_blocks.join("\n"))
    }

    fn emit_mutable(&self, is_mutable: bool) -> String {
        if is_mutable {
            " mut".to_string()
        } else {
            "".to_string()
        }
    }

    fn emit_function_name(&self, function_name: &Option<String>) -> String {
        if let Some(func_name) = function_name {
            func_name.clone()
        } else {
            "new".into()
        }
    }

    fn emit_new_with_args(
        &self,
        od: &ObjectDescription,
        type_parameters: &HashMap<String, String>,
        function_name: &Option<String>,
        pack_man: &PackageManager,
        args: &Vec<Argument>,
        current_namespace: &Namespace,
    ) -> Result<String, Error> {
        let mut new_namespace = current_namespace.clone();
        new_namespace.add_part(&od.name);

        let args_construction_code =
            self.emit_args_construction_code(pack_man, args, &new_namespace, type_parameters)?;

        Ok(format!(
            "{}\nlet{} {} = {}::{}{}({});",
            args_construction_code,
            self.emit_mutable(od.is_mutable),
            self.emit_fully_qualified_name(&od.name, current_namespace, false),
            od.type_name,
            if od.type_parameter_part.is_empty() {
                "".to_string()
            } else {
                od.type_parameter_part.clone() + "::"
            },
            self.emit_function_name(function_name),
            self.emit_args(args, &new_namespace)
        ))
    }

    fn emit_default(
        &self,
        od: &ObjectDescription,
        _pack_man: &PackageManager,
        current_namespace: &Namespace,
    ) -> Result<String, Error> {
        Ok(format!(
            "let{} {}:{}{} = Default::default();",
            self.emit_mutable(od.is_mutable),
            self.emit_fully_qualified_name(&od.name, current_namespace, false),
            od.type_name,
            od.type_parameter_part
        ))
    }

    fn emit_json_path(&self, cn: &Namespace, od: &ObjectDescription) -> String {
        format!(
            "{}[\"{}\"]",
            cn.parts
                .iter()
                .map(|item| format!("[\"{}\"]", item))
                .collect::<Vec<String>>()
                .join(""),
            od.name
        )
    }

    fn emit_new_from_json(
        &self,
        od: &ObjectDescription,
        current_namespace: &Namespace,
    ) -> Result<String, Error> {
        let emit_fully_qualified_name =
            self.emit_fully_qualified_name(&od.name, current_namespace, false);

        Ok(format!(
            "let{} {}: {}{} = serde_json::from_value(data{}.clone()).expect(\"Could not create '{}' from Json.\");",
            self.emit_mutable(od.is_mutable),
            emit_fully_qualified_name,
            od.type_name,
            od.type_parameter_part,
            self.emit_json_path(current_namespace, od),
            emit_fully_qualified_name
        ))
    }

    fn emit_constructor_from_code(
        &self,
        od: &ObjectDescription,
        current_namespace: &Namespace,
        code_template: &String,
        type_parameters: &HashMap<String, String>,
    ) -> Result<String, Error> {
        let mut handlebars = Handlebars::new();

        handlebars.register_template_string("code", code_template)?;

        let mut data = HashMap::new();
        data.insert(
            "fully_qualified_name".to_string(),
            self.emit_fully_qualified_name(&od.name, current_namespace, false),
        );
        data.insert("type_name".to_string(), od.type_name.clone());
        data.insert(
            "type_parameter_part".to_string(),
            od.type_parameter_part.clone(),
        );
        data.insert("mutable".to_string(), self.emit_mutable(od.is_mutable));
        for (param, resolved_param) in type_parameters {
            data.insert(format!("type_parameter_{}", param), resolved_param.clone());
        }

        match handlebars.render("code", &data) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.into()),
        }
    }
}

impl Constructor {
    pub fn emit_code_template(
        &self,
        obj_desc: &ObjectDescription,
        type_parameters: &HashMap<String, String>,
        pack_man: &PackageManager,
        namespace: &Namespace,
    ) -> Result<String, Error> {
        match self {
            Self::New { function_name } => self.emit_new_with_args(
                obj_desc,
                type_parameters,
                function_name,
                pack_man,
                &vec![],
                namespace,
            ),

            Self::NewWithObserver { function_name } => self.emit_new_with_args(
                obj_desc,
                type_parameters,
                function_name,
                pack_man,
                &vec![Argument::new_change_observer_arg()],
                namespace,
            ),

            Self::NewWithObserverAndContext { function_name } => self.emit_new_with_args(
                obj_desc,
                type_parameters,
                function_name,
                pack_man,
                &vec![
                    Argument::new_change_observer_arg(),
                    Argument::new_context_arg(),
                ],
                namespace,
            ),

            Self::NewWithArbitraryArgs {
                function_name,
                arguments,
            } => self.emit_new_with_args(
                obj_desc,
                type_parameters,
                function_name,
                pack_man,
                arguments,
                namespace,
            ),

            Self::FromJson => self.emit_new_from_json(obj_desc, namespace),

            Self::FromDefault => self.emit_default(obj_desc, pack_man, namespace),

            Self::FromCode { code_template } => {
                self.emit_constructor_from_code(obj_desc, namespace, code_template, type_parameters)
            }
        }
    }
}
