use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderRandom;

#[derive(Serialize)]
struct IntegerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keepers: Option<RecField<PrimField<String>>>,
    max: PrimField<f64>,
    min: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<PrimField<String>>,
}

struct Integer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IntegerData>,
}

#[derive(Clone)]
pub struct Integer(Rc<Integer_>);

impl Integer {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderRandom) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `keepers`.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn set_keepers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().keepers = Some(v.into());
        self
    }

    #[doc= "Set the field `seed`.\nA custom seed to always produce the same value."]
    pub fn set_seed(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().seed = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe string representation of the integer result."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max` after provisioning.\nThe maximum inclusive value of the range."]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min` after provisioning.\nThe minimum inclusive value of the range."]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `result` after provisioning.\nThe random integer result."]
    pub fn result(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `seed` after provisioning.\nA custom seed to always produce the same value."]
    pub fn seed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seed", self.extract_ref()))
    }
}

impl Resource for Integer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Integer {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Integer {
    type O = ListRef<IntegerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for Integer_ {
    fn extract_resource_type(&self) -> String {
        "random_integer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInteger {
    pub tf_id: String,
    #[doc= "The maximum inclusive value of the range."]
    pub max: PrimField<f64>,
    #[doc= "The minimum inclusive value of the range."]
    pub min: PrimField<f64>,
}

impl BuildInteger {
    pub fn build(self, stack: &mut Stack) -> Integer {
        let out = Integer(Rc::new(Integer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IntegerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                keepers: core::default::Default::default(),
                max: self.max,
                min: self.min,
                seed: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IntegerRef {
    shared: StackShared,
    base: String,
}

impl Ref for IntegerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IntegerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe string representation of the integer result."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max` after provisioning.\nThe maximum inclusive value of the range."]
    pub fn max(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min` after provisioning.\nThe minimum inclusive value of the range."]
    pub fn min(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `result` after provisioning.\nThe random integer result."]
    pub fn result(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `seed` after provisioning.\nA custom seed to always produce the same value."]
    pub fn seed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seed", self.extract_ref()))
    }
}
