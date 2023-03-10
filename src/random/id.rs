use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderRandom;

#[derive(Serialize)]
struct IdData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    byte_length: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keepers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

struct Id_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IdData>,
}

#[derive(Clone)]
pub struct Id(Rc<Id_>);

impl Id {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `prefix`.\nArbitrary string to prefix the output value with. This string is supplied as-is, meaning it is not guaranteed to be URL-safe or base64 encoded."]
    pub fn set_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().prefix = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `b64_std` after provisioning.\nThe generated id presented in base64 without additional transformations."]
    pub fn b64_std(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.b64_std", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `b64_url` after provisioning.\nThe generated id presented in base64, using the URL-friendly character set: case-sensitive letters, digits and the characters `_` and `-`."]
    pub fn b64_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.b64_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `byte_length` after provisioning.\nThe number of random bytes to produce. The minimum value is 1, which produces eight bits of randomness."]
    pub fn byte_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.byte_length", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dec` after provisioning.\nThe generated id presented in non-padded decimal digits."]
    pub fn dec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hex` after provisioning.\nThe generated id presented in padded hexadecimal digits. This result will always be twice as long as the requested byte length."]
    pub fn hex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe generated id presented in base64 without additional transformations or prefix."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\nArbitrary string to prefix the output value with. This string is supplied as-is, meaning it is not guaranteed to be URL-safe or base64 encoded."]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }
}

impl Referable for Id {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Id { }

impl ToListMappable for Id {
    type O = ListRef<IdRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Id_ {
    fn extract_resource_type(&self) -> String {
        "random_id".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildId {
    pub tf_id: String,
    #[doc= "The number of random bytes to produce. The minimum value is 1, which produces eight bits of randomness."]
    pub byte_length: PrimField<f64>,
}

impl BuildId {
    pub fn build(self, stack: &mut Stack) -> Id {
        let out = Id(Rc::new(Id_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IdData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                byte_length: self.byte_length,
                keepers: core::default::Default::default(),
                prefix: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IdRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IdRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `b64_std` after provisioning.\nThe generated id presented in base64 without additional transformations."]
    pub fn b64_std(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.b64_std", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `b64_url` after provisioning.\nThe generated id presented in base64, using the URL-friendly character set: case-sensitive letters, digits and the characters `_` and `-`."]
    pub fn b64_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.b64_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `byte_length` after provisioning.\nThe number of random bytes to produce. The minimum value is 1, which produces eight bits of randomness."]
    pub fn byte_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.byte_length", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dec` after provisioning.\nThe generated id presented in non-padded decimal digits."]
    pub fn dec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hex` after provisioning.\nThe generated id presented in padded hexadecimal digits. This result will always be twice as long as the requested byte length."]
    pub fn hex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe generated id presented in base64 without additional transformations or prefix."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\nArbitrary string to prefix the output value with. This string is supplied as-is, meaning it is not guaranteed to be URL-safe or base64 encoded."]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }
}
