
#[derive(Debug)]
pub struct ModuleInfo {
    module_name: String,
    module_va: usize,
}

impl ModuleInfo {
    pub fn new(module_name: &str, module_va: usize) -> Self {
        Self {
            module_name: module_name.to_string().to_lowercase(),
            module_va,
        }
    }
    pub fn get_mod_name(&self) -> &str {
        &self.module_name
    }
    pub fn get_mod_va(&self) -> &usize {
        &self.module_va
    }
}