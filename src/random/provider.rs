#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Serialize)]
struct ProviderRandomData_ {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
}

struct ProviderRandom_ {
    data: std::cell::RefCell<ProviderRandomData_>,
}

pub struct ProviderRandom(std::rc::Rc<ProviderRandom_>);

impl ProviderRandom {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "random", alias)
        } else {
            "random".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }
}

impl terrars::Provider for ProviderRandom_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "random".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "hashicorp/random",
            "version": "3.9.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderRandom {}

impl BuildProviderRandom {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderRandom {
        let out =
            ProviderRandom(
                std::rc::Rc::new(
                    ProviderRandom_ {
                        data: std::cell::RefCell::new(ProviderRandomData_ { alias: std::option::Option::None }),
                    },
                ),
            );
        stack.add_provider(out.0.clone());
        out
    }
}
