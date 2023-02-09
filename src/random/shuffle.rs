use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderRandom;

#[derive(Serialize)]
struct ShuffleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    input: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keepers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<PrimField<String>>,
}

struct Shuffle_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ShuffleData>,
}

#[derive(Clone)]
pub struct Shuffle(Rc<Shuffle_>);

impl Shuffle {
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

    #[doc= "Set the field `result_count`.\nThe number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list."]
    pub fn set_result_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().result_count = Some(v.into());
        self
    }

    #[doc= "Set the field `seed`.\nArbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.\n\n**Important:** Even with an identical seed, it is not guaranteed that the same permutation will be produced across different versions of Terraform. This argument causes the result to be *less volatile*, but not fixed for all time."]
    pub fn set_seed(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().seed = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nA static value used internally by Terraform, this should not be referenced in configurations."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `input` after provisioning.\nThe list of strings to shuffle."]
    pub fn input(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `result` after provisioning.\nRandom permutation of the list of strings given in `input`."]
    pub fn result(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `result_count` after provisioning.\nThe number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list."]
    pub fn result_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.result_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `seed` after provisioning.\nArbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.\n\n**Important:** Even with an identical seed, it is not guaranteed that the same permutation will be produced across different versions of Terraform. This argument causes the result to be *less volatile*, but not fixed for all time."]
    pub fn seed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seed", self.extract_ref()))
    }
}

impl Resource for Shuffle {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Shuffle {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Shuffle {
    type O = ListRef<ShuffleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Shuffle_ {
    fn extract_resource_type(&self) -> String {
        "random_shuffle".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildShuffle {
    pub tf_id: String,
    #[doc= "The list of strings to shuffle."]
    pub input: ListField<PrimField<String>>,
}

impl BuildShuffle {
    pub fn build(self, stack: &mut Stack) -> Shuffle {
        let out = Shuffle(Rc::new(Shuffle_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ShuffleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                input: self.input,
                keepers: core::default::Default::default(),
                result_count: core::default::Default::default(),
                seed: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ShuffleRef {
    shared: StackShared,
    base: String,
}

impl Ref for ShuffleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ShuffleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nA static value used internally by Terraform, this should not be referenced in configurations."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `input` after provisioning.\nThe list of strings to shuffle."]
    pub fn input(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `result` after provisioning.\nRandom permutation of the list of strings given in `input`."]
    pub fn result(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `result_count` after provisioning.\nThe number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list."]
    pub fn result_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.result_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `seed` after provisioning.\nArbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.\n\n**Important:** Even with an identical seed, it is not guaranteed that the same permutation will be produced across different versions of Terraform. This argument causes the result to be *less volatile*, but not fixed for all time."]
    pub fn seed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seed", self.extract_ref()))
    }
}
