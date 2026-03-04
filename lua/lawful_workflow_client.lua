local cjson = require("cjson.safe")

local LawfulWorkflowClient = {}

-- `rust_eval_lawful_transition` is the single FFI hook into Rust.
local function rust_eval_lawful_transition(encoded)
  error("rust_eval_lawful_transition FFI not bound")
end

function LawfulWorkflowClient.append_state(record, state, transition)
  local payload = {
    record = record,
    state = state,
    transition = transition,
  }
  local encoded = cjson.encode(payload)
  local resp_bytes = rust_eval_lawful_transition(encoded)
  local decision = cjson.decode(resp_bytes)

  if decision.Accepted then
    return true, decision.Accepted
  elseif decision.Rejected then
    return false, decision.Rejected.violations
  else
    return false, { "invalid decision from LawfulWorkflowEngine" }
  end
end

return LawfulWorkflowClient
