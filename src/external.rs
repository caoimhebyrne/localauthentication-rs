#![allow(non_snake_case)]

use swift_rs::{swift, Bool, Int, SRObject, SRString};

// func lacontext_new() -> LAContext
swift!(pub(crate) fn lacontext_new() -> SRObject<LAContext>);

// func lacontext_canEvaluatePolicy(context: LAContext, policy: LAPolicy) -> Bool
swift!(pub(crate) fn lacontext_canEvaluatePolicy(context: &SRObject<LAContext>, policy: Int) -> Bool);

// func lacontext_evaluatePolicy(context: LAContext, policy: LAPolicy, reason: SRString)
swift!(pub(crate) fn lacontext_evaluatePolicy(context: &SRObject<LAContext>, policy: Int, reason: &SRString) -> Bool);

#[repr(C)]
pub(crate) struct LAContext {
    interactionNotAllowed: Bool,
}
