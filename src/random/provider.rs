use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderRandomData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
}

struct ProviderRandom_ {
    data: RefCell<ProviderRandomData>,
}

pub struct ProviderRandom(Rc<ProviderRandom_>);

impl ProviderRandom {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "random", alias)
        } else {
            "random".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }
}

impl Provider for ProviderRandom_ {
    fn extract_type_tf_id(&self) -> String {
        "random".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "hashicorp/random",
            "version": "3.4.3",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderRandom {}

impl BuildProviderRandom {
    pub fn build(self, stack: &mut Stack) -> ProviderRandom {
        let out =
            ProviderRandom(Rc::new(ProviderRandom_ { data: RefCell::new(ProviderRandomData { alias: None }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
