use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct OpenApiSpec {
    paths: HashMap<String, PathItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PathItem {
    #[serde(default)]
    get: Option<Operation>,
    #[serde(default)]
    post: Option<Operation>,
    #[serde(default)]
    put: Option<Operation>,
    #[serde(default)]
    delete: Option<Operation>,
    #[serde(default)]
    patch: Option<Operation>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Operation {
    #[serde(default)]
    #[serde(rename = "operationId")]
    operation_id: Option<String>,
    #[serde(default)]
    summary: Option<String>,
}

fn main() {
    // Tell cargo to rerun if openapi.yaml changes
    println!("cargo:rerun-if-changed=doc/openapi.yaml");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_routes.rs");

    // Parse OpenAPI spec
    let openapi_content = fs::read_to_string("doc/openapi.yaml")
        .expect("Failed to read doc/openapi.yaml");
    let spec: OpenApiSpec = serde_yaml::from_str(&openapi_content)
        .expect("Failed to parse OpenAPI spec");

    // Generate routing code
    let generated_code = generate_router_code(&spec);

    // Write to output
    fs::write(&dest_path, generated_code)
        .expect("Failed to write generated routes");
}

fn generate_router_code(spec: &OpenApiSpec) -> String {
    let mut code = String::new();
    
    // File header
    code.push_str("// This file is auto-generated from doc/openapi.yaml by build.rs\n");
    code.push_str("// DO NOT EDIT MANUALLY\n\n");
    
    // Collect which methods are actually used
    let mut uses_get = false;
    let mut uses_post = false;
    let mut uses_put = false;
    let mut uses_delete = false;
    let mut uses_patch = false;
    
    for path_item in spec.paths.values() {
        if path_item.get.is_some() { uses_get = true; }
        if path_item.post.is_some() { uses_post = true; }
        if path_item.put.is_some() { uses_put = true; }
        if path_item.delete.is_some() { uses_delete = true; }
        if path_item.patch.is_some() { uses_patch = true; }
    }
    
    // Generate selective imports
    code.push_str("use axum::{routing::{");
    let mut imports = Vec::new();
    if uses_get { imports.push("get"); }
    if uses_post { imports.push("post"); }
    if uses_put { imports.push("put"); }
    if uses_delete { imports.push("delete"); }
    if uses_patch { imports.push("patch"); }
    code.push_str(&imports.join(", "));
    code.push_str("}, Router};\n\n");
    
    code.push_str("pub fn build_router(state: crate::handlers::AppState) -> Router {\n");
    code.push_str("    Router::new()\n");

    // Generate routes
    for (path, path_item) in spec.paths.iter() {
        let axum_path = convert_path_to_axum(path);
        let methods = collect_methods(path, path_item);
        
        if !methods.is_empty() {
            code.push_str(&format!("        .route(\"{}\", {})\n", 
                axum_path, 
                methods.join("")
            ));
        }
    }

    code.push_str("        .with_state(state)\n");
    code.push_str("}\n");
    
    code
}

fn convert_path_to_axum(path: &str) -> String {
    // Convert OpenAPI path parameters {id} to Axum path parameters :id
    path.replace("{", ":").replace("}", "")
}

fn collect_methods(path: &str, path_item: &PathItem) -> Vec<String> {
    let mut methods = Vec::new();
    
    if let Some(_op) = &path_item.get {
        let handler = derive_handler_name(path, "get");
        if methods.is_empty() {
            methods.push(format!("get(crate::handlers::{})", handler));
        } else {
            methods.push(format!(".get(crate::handlers::{})", handler));
        }
    }
    
    if let Some(_op) = &path_item.post {
        let handler = derive_handler_name(path, "post");
        if methods.is_empty() {
            methods.push(format!("post(crate::handlers::{})", handler));
        } else {
            methods.push(format!(".post(crate::handlers::{})", handler));
        }
    }
    
    if let Some(_op) = &path_item.put {
        let handler = derive_handler_name(path, "put");
        if methods.is_empty() {
            methods.push(format!("put(crate::handlers::{})", handler));
        } else {
            methods.push(format!(".put(crate::handlers::{})", handler));
        }
    }
    
    if let Some(_op) = &path_item.delete {
        let handler = derive_handler_name(path, "delete");
        if methods.is_empty() {
            methods.push(format!("delete(crate::handlers::{})", handler));
        } else {
            methods.push(format!(".delete(crate::handlers::{})", handler));
        }
    }
    
    if let Some(_op) = &path_item.patch {
        let handler = derive_handler_name(path, "patch");
        if methods.is_empty() {
            methods.push(format!("patch(crate::handlers::{})", handler));
        } else {
            methods.push(format!(".patch(crate::handlers::{})", handler));
        }
    }
    
    methods
}

fn derive_handler_name(path: &str, method: &str) -> String {
    // Derive handler name from path and method
    // Examples:
    // /ingredients + get -> ingredients::list_ingredients
    // /ingredients + post -> ingredients::create_ingredient
    // /ingredients/{id} + put -> ingredients::update_ingredient
    //
    // NOTE: Singularization uses simple logic (remove trailing 's')
    // which works for regular English plurals but may fail for irregular plurals
    // (e.g., 'categories', 'people'). Consider using a pluralization library
    // or mapping table for production use with diverse resource names.
    
    let path_parts: Vec<&str> = path.split('/')
        .filter(|s| !s.is_empty() && !s.starts_with('{'))
        .collect();
    
    if path_parts.is_empty() {
        return format!("{}_{}", method, "root");
    }
    
    let resource = path_parts[0];
    let has_id = path.contains('{');
    
    let action = match (method, has_id) {
        ("get", false) => "list",
        ("get", true) => "get",
        ("post", _) => "create",
        ("put", _) => "update",
        ("delete", _) => "delete",
        ("patch", _) => "patch",
        _ => method,
    };
    
    // For list action, keep plural; for others use singular
    let resource_name = if action == "list" {
        resource
    } else if resource.ends_with('s') {
        &resource[..resource.len() - 1]
    } else {
        resource
    };
    
    format!("{}::{}_{}", resource, action, resource_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_path_to_axum() {
        assert_eq!(convert_path_to_axum("/ingredients"), "/ingredients");
        assert_eq!(convert_path_to_axum("/ingredients/{id}"), "/ingredients/:id");
        assert_eq!(convert_path_to_axum("/recipes/{id}/steps/{stepId}"), 
                   "/recipes/:id/steps/:stepId");
    }

    #[test]
    fn test_derive_handler_name() {
        assert_eq!(derive_handler_name("/ingredients", "get"), "ingredients::list_ingredients");
        assert_eq!(derive_handler_name("/ingredients", "post"), "ingredients::create_ingredient");
        assert_eq!(derive_handler_name("/ingredients/{id}", "put"), "ingredients::update_ingredient");
        assert_eq!(derive_handler_name("/ingredients/{id}", "get"), "ingredients::get_ingredient");
        assert_eq!(derive_handler_name("/recipes", "get"), "recipes::list_recipes");
        assert_eq!(derive_handler_name("/recipes/{id}", "put"), "recipes::update_recipe");
    }
}
