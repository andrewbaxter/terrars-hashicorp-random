#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderRandom;

#[derive(Serialize)]
struct PasswordData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keepers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    length: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lower: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_lower: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_numeric: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_special: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_upper: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    numeric: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_special: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    special: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upper: std::option::Option<terrars::PrimField<bool>>,
}

struct Password_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PasswordData_>,
}

#[derive(Clone)]
pub struct Password(std::rc::Rc<Password_>);

impl Password {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderRandom) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc =
        "Set the field `keepers`.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn set_keepers(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().keepers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lower`.\nInclude lowercase alphabet characters in the result. Default value is `true`."]
    pub fn set_lower(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lower = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_lower`.\nMinimum number of lowercase alphabet characters in the result. Default value is `0`."]
    pub fn set_min_lower(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_lower = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_numeric`.\nMinimum number of numeric characters in the result. Default value is `0`."]
    pub fn set_min_numeric(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_numeric = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_special`.\nMinimum number of special characters in the result. Default value is `0`."]
    pub fn set_min_special(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_special = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_upper`.\nMinimum number of uppercase alphabet characters in the result. Default value is `0`."]
    pub fn set_min_upper(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_upper = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `number`.\nInclude numeric characters in the result. Default value is `true`. If `number`, `upper`, `lower`, and `special` are all configured, at least one of them must be set to `true`. **NOTE**: This is deprecated, use `numeric` instead."]
    pub fn set_number(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().number = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `numeric`.\nInclude numeric characters in the result. Default value is `true`. If `numeric`, `upper`, `lower`, and `special` are all configured, at least one of them must be set to `true`."]
    pub fn set_numeric(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().numeric = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_special`.\nSupply your own list of special characters to use for password generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation."]
    pub fn set_override_special(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().override_special = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `special`.\nInclude special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`."]
    pub fn set_special(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().special = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upper`.\nInclude uppercase alphabet characters in the result. Default value is `true`."]
    pub fn set_upper(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().upper = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `bcrypt_hash` after provisioning.\nA bcrypt hash of the generated random password. **NOTE**: If the generated random password is greater than 72 bytes in length, `bcrypt_hash` will contain a hash of the first 72 bytes."]
    pub fn bcrypt_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bcrypt_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nA static value used internally by Terraform, this should not be referenced in configurations."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `length` after provisioning.\nThe length of the password desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`)."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lower` after provisioning.\nInclude lowercase alphabet characters in the result. Default value is `true`."]
    pub fn lower(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_lower` after provisioning.\nMinimum number of lowercase alphabet characters in the result. Default value is `0`."]
    pub fn min_lower(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_lower", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_numeric` after provisioning.\nMinimum number of numeric characters in the result. Default value is `0`."]
    pub fn min_numeric(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_numeric", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_special` after provisioning.\nMinimum number of special characters in the result. Default value is `0`."]
    pub fn min_special(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_special", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_upper` after provisioning.\nMinimum number of uppercase alphabet characters in the result. Default value is `0`."]
    pub fn min_upper(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_upper", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `number` after provisioning.\nInclude numeric characters in the result. Default value is `true`. If `number`, `upper`, `lower`, and `special` are all configured, at least one of them must be set to `true`. **NOTE**: This is deprecated, use `numeric` instead."]
    pub fn number(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `numeric` after provisioning.\nInclude numeric characters in the result. Default value is `true`. If `numeric`, `upper`, `lower`, and `special` are all configured, at least one of them must be set to `true`."]
    pub fn numeric(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.numeric", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `override_special` after provisioning.\nSupply your own list of special characters to use for password generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation."]
    pub fn override_special(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_special", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe generated random password."]
    pub fn result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `special` after provisioning.\nInclude special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`."]
    pub fn special(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.special", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upper` after provisioning.\nInclude uppercase alphabet characters in the result. Default value is `true`."]
    pub fn upper(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper", self.extract_ref()))
    }
}

impl terrars::Referable for Password {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Password { }

impl terrars::ToListMappable for Password {
    type O = terrars::ListRef<PasswordRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Password_ {
    fn extract_resource_type(&self) -> std::string::String {
        "random_password".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPassword {
    pub tf_id: std::string::String,
    #[doc =
        "The length of the password desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`)."]
    pub length: terrars::PrimField<f64>,
}

impl BuildPassword {
    pub fn build(self, stack: &mut terrars::Stack) -> Password {
        let out = Password(std::rc::Rc::new(Password_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PasswordData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PasswordRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PasswordRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bcrypt_hash` after provisioning.\nA bcrypt hash of the generated random password. **NOTE**: If the generated random password is greater than 72 bytes in length, `bcrypt_hash` will contain a hash of the first 72 bytes."]
    pub fn bcrypt_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bcrypt_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nA static value used internally by Terraform, this should not be referenced in configurations."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource. See [the main provider documentation](../index.html) for more information."]
    pub fn keepers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `length` after provisioning.\nThe length of the password desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`)."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lower` after provisioning.\nInclude lowercase alphabet characters in the result. Default value is `true`."]
    pub fn lower(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_lower` after provisioning.\nMinimum number of lowercase alphabet characters in the result. Default value is `0`."]
    pub fn min_lower(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_lower", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_numeric` after provisioning.\nMinimum number of numeric characters in the result. Default value is `0`."]
    pub fn min_numeric(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_numeric", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_special` after provisioning.\nMinimum number of special characters in the result. Default value is `0`."]
    pub fn min_special(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_special", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_upper` after provisioning.\nMinimum number of uppercase alphabet characters in the result. Default value is `0`."]
    pub fn min_upper(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_upper", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `number` after provisioning.\nInclude numeric characters in the result. Default value is `true`. If `number`, `upper`, `lower`, and `special` are all configured, at least one of them must be set to `true`. **NOTE**: This is deprecated, use `numeric` instead."]
    pub fn number(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `numeric` after provisioning.\nInclude numeric characters in the result. Default value is `true`. If `numeric`, `upper`, `lower`, and `special` are all configured, at least one of them must be set to `true`."]
    pub fn numeric(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.numeric", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `override_special` after provisioning.\nSupply your own list of special characters to use for password generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation."]
    pub fn override_special(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_special", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe generated random password."]
    pub fn result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `special` after provisioning.\nInclude special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`."]
    pub fn special(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.special", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upper` after provisioning.\nInclude uppercase alphabet characters in the result. Default value is `true`."]
    pub fn upper(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper", self.extract_ref()))
    }
}
