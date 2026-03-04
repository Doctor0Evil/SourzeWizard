export class LawfulWorkflowGateway {
  constructor(rustBridge) {
    this.rustBridge = rustBridge; // must expose evalLawfulTransition(bytes)
  }

  async appendState(record, state, transition) {
    const payload = { record, state, transition };
    const encoded = new TextEncoder().encode(JSON.stringify(payload));
    const resp = await this.rustBridge.evalLawfulTransition(encoded);
    const decoded = JSON.parse(new TextDecoder().decode(resp));

    if (decoded.Accepted) {
      return { ok: true, record: decoded.Accepted };
    }
    if (decoded.Rejected) {
      return { ok: false, violations: decoded.Rejected.violations };
    }
    return { ok: false, violations: ["invalid decision from LawfulWorkflowEngine"] };
  }
}
