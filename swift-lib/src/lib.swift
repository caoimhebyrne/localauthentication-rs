import SwiftRs
import LocalAuthentication

@_cdecl("lacontext_new")
func newLAContext() -> LAContext {
    return LAContext()
}

@_cdecl("lacontext_canEvaluatePolicy")
func canEvaluatePolicy(context: LAContext, policy: LAPolicy) -> Bool {
    return context.canEvaluatePolicy(policy, error: nil)
}

@_cdecl("lacontext_evaluatePolicy")
func evaluatePolicy(context: LAContext, policy: LAPolicy, reason: SRString) -> Bool {
    let reason = reason.toString()

    let semaphore = DispatchSemaphore(value: 0)
    var didEvaluate = false

    context.evaluatePolicy(policy, localizedReason: reason) { (success, error) in   
        didEvaluate = success && error == nil
        semaphore.signal()
    }

    semaphore.wait()
    return didEvaluate
}
