use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderRandom;

#[derive(Serialize)]
struct PasswordData {
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
    length: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lower: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_lower: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_numeric: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_special: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_upper: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    numeric: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_special: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upper: Option<PrimField<bool>>,
}

struct Password_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PasswordData>,
}

#[derive(Clone)]
pub struct Password(Rc<Password_>);

impl Password {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `lower`.\nInclude lowercase alphabet characters in the result. Default value is `true`."]
    pub fn set_lower(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lower = Some(v.into());
        self
    }

    #[doc= "Set the field `min_lower`.\nMinimum number of lowercase alphabet characters in the result. Default value is `0`."]
    pub fn set_min_lower(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_lower = Some(v.into());
        self
    }

    #[doc= "Set the field `min_numeric`.\nMinimum number of numeric characters in the result. Default value is `0`."]
    pub fn set_min_numeric(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_numeric = Some(v.into());
        self
    }

    #[doc= "Set the field `min_special`.\nMinimum number of special characters in the result. Default value is `0`."]
    pub fn set_min_special(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_special = Some(v.into());
        self
    }

    #[doc= "Set the field `min_upper`.\nMinimum number of uppercase alphabet characters in the result. Default value is `0`."]
    pub fn set_min_upper(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_upper = Some(v.into());
        self
    }

    #[doc= "Set the field `number`.\nInclude numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead."]
    pub fn set_number(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().number = Some(v.into());
        self
    }

    #[doc= "Set the field `numeric`.\nInclude numeric characters in the result. Default value is `true`."]
    pub fn set_numeric(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().numeric = Some(v.into());
        self
    }

    #[doc= "Set the field `override_special`.\nSupply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation."]
    pub fn set_override_special(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().override_special = Some(v.into());
        self
    }

    #[doc= "Set the field `special`.\nInclude special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`."]
    pub fn set_special(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().special = Some(v.into());
        self
    }

    #[doc= "Set the field `upper`.\nInclude uppercase alphabet characters in the result. Default value is `true`."]
    pub fn set_upper(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().upper = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `bcrypt_hash` after provisioning.\nA bcrypt hash of the generated random string."]
    pub fn bcrypt_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bcrypt_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nA static value used internally by Terraform, this should not be referenced in configurations."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `length` after provisioning.\nThe length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`)."]
    pub fn length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.length", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lower` after provisioning.\nInclude lowercase alphabet characters in the result. Default value is `true`."]
    pub fn lower(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lower", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_lower` after provisioning.\nMinimum number of lowercase alphabet characters in the result. Default value is `0`."]
    pub fn min_lower(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_lower", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_numeric` after provisioning.\nMinimum number of numeric characters in the result. Default value is `0`."]
    pub fn min_numeric(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_numeric", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_special` after provisioning.\nMinimum number of special characters in the result. Default value is `0`."]
    pub fn min_special(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_special", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_upper` after provisioning.\nMinimum number of uppercase alphabet characters in the result. Default value is `0`."]
    pub fn min_upper(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_upper", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\nInclude numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead."]
    pub fn number(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `numeric` after provisioning.\nInclude numeric characters in the result. Default value is `true`."]
    pub fn numeric(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.numeric", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `override_special` after provisioning.\nSupply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation."]
    pub fn override_special(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.override_special", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `result` after provisioning.\nThe generated random string."]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `special` after provisioning.\nInclude special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`."]
    pub fn special(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.special", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upper` after provisioning.\nInclude uppercase alphabet characters in the result. Default value is `true`."]
    pub fn upper(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.upper", self.extract_ref()))
    }
}

impl Resource for Password {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for Password {
    type O = ListRef<PasswordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Password_ {
    fn extract_resource_type(&self) -> String {
        "random_password".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPassword {
    pub tf_id: String,
    #[doc= "The length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`)."]
    pub length: PrimField<f64>,
}

impl BuildPassword {
    pub fn build(self, stack: &mut Stack) -> Password {
        let out = Password(Rc::new(Password_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PasswordData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                keepers: core::default::Default::default(),
                length: self.length,
                lower: core::default::Default::default(),
                min_lower: core::default::Default::default(),
                min_numeric: core::default::Default::default(),
                min_special: core::default::Default::default(),
                min_upper: core::default::Default::default(),
                number: core::default::Default::default(),
                numeric: core::default::Default::default(),
                override_special: core::default::Default::default(),
                special: core::default::Default::default(),
                upper: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PasswordRef {
    shared: StackShared,
    base: String,
}

impl Ref for PasswordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PasswordRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bcrypt_hash` after provisioning.\nA bcrypt hash of the generated random string."]
    pub fn bcrypt_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bcrypt_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nA static value used internally by Terraform, this should not be referenced in configurations."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `length` after provisioning.\nThe length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`)."]
    pub fn length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.length", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lower` after provisioning.\nInclude lowercase alphabet characters in the result. Default value is `true`."]
    pub fn lower(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lower", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_lower` after provisioning.\nMinimum number of lowercase alphabet characters in the result. Default value is `0`."]
    pub fn min_lower(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_lower", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_numeric` after provisioning.\nMinimum number of numeric characters in the result. Default value is `0`."]
    pub fn min_numeric(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_numeric", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_special` after provisioning.\nMinimum number of special characters in the result. Default value is `0`."]
    pub fn min_special(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_special", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_upper` after provisioning.\nMinimum number of uppercase alphabet characters in the result. Default value is `0`."]
    pub fn min_upper(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_upper", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\nInclude numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead."]
    pub fn number(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `numeric` after provisioning.\nInclude numeric characters in the result. Default value is `true`."]
    pub fn numeric(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.numeric", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `override_special` after provisioning.\nSupply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation."]
    pub fn override_special(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.override_special", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `result` after provisioning.\nThe generated random string."]
    pub fn result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `special` after provisioning.\nInclude special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`."]
    pub fn special(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.special", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upper` after provisioning.\nInclude uppercase alphabet characters in the result. Default value is `true`."]
    pub fn upper(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.upper", self.extract_ref()))
    }
}
