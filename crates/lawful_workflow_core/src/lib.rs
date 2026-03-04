use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Did {
    pub method: String,
    pub id: String,
    pub context_tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SovereigntyFlag {
    NeuroSovereign,
    PublicEcological,
    RestrictedPrivate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CitizenContext {
    pub did: Did,
    pub sovereignty_flag: SovereigntyFlag,
    pub jurisdiction: String,
    pub neuro_profile_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiContext {
    pub model_id: String,
    pub version: String,
    pub host_did: Did,
    pub capabilities: Vec<String>,
    pub ndm_ceiling: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentToken {
    pub token_id: String,
    pub subject_did: Did,
    pub scope: String,
    pub revocable: bool,
    pub issued_at: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SovereigntyProfile {
    pub subject_did: Did,
    pub allow_neural_ops: bool,
    pub allow_robotic_ops: bool,
    pub allow_ecological_ops: bool,
    pub max_risk: f64,
    pub ecoscore_floor: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Step {
    Describe,
    Normalize,
    Graph,
    StressTest,
    Architect,
    Question,
    Experiment,
    Record,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowState {
    pub state_id: String,
    pub citizen: CitizenContext,
    pub ai: AiContext,
    pub step: Step,
    pub assumptions: Vec<String>,
    pub constraints: Vec<String>,
    pub sovereignty: SovereigntyProfile,
    pub consent: ConsentToken,
    pub ndm_score: f64,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransitionProof {
    pub from_state_id: String,
    pub to_state_id: String,
    pub performer_did: Did,
    pub consent_token: ConsentToken,
    pub justification: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceRecord {
    pub session_id: String,
    pub states: Vec<WorkflowState>,
    pub transitions: Vec<TransitionProof>,
    pub applied_invariants: Vec<String>,
    pub violated_invariants: Vec<String>,
    pub open_research_questions: Vec<String>,
    pub suggested_experiments: Vec<String>,
    pub anchor_rowsnapshot_id: String,
    pub anchor_organichain_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LawViolation {
    StepOrder(String),
    ConsentMissing(String),
    SovereigntyDowngrade(String),
    NdmExceeded { current: f64, ceiling: f64 },
    EcoscoreFloor(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LawDecision {
    Accepted(GovernanceRecord),
    Rejected { violations: Vec<LawViolation> },
}

pub struct LawfulWorkflowEngine;

impl LawfulWorkflowEngine {
    pub fn append_state(
        mut record: GovernanceRecord,
        next: WorkflowState,
        transition: TransitionProof,
    ) -> LawDecision {
        let mut violations = Vec::new();

        if !Self::check_step_sequence(&record.states, next.step) {
            violations.push(LawViolation::StepOrder(format!(
                "illegal transition to {:?}",
                next.step
            )));
        }

        if !Self::check_consent(&transition.consent_token, &next.citizen.did) {
            violations.push(LawViolation::ConsentMissing(
                "invalid or mismatched consent token".into(),
            ));
        }

        if !Self::check_sovereignty_monotone(&record.states, &next.sovereignty) {
            violations.push(LawViolation::SovereigntyDowngrade(
                "sovereignty profile weakened".into(),
            ));
        }

        if next.ndm_score > next.ai.ndm_ceiling {
            violations.push(LawViolation::NdmExceeded {
                current: next.ndm_score,
                ceiling: next.ai.ndm_ceiling,
            });
        }

        if next.sovereignty.ecoscore_floor < 0.86 {
            violations.push(LawViolation::EcoscoreFloor(
                "ecoscore_floor below 0.86".into(),
            ));
        }

        if !violations.is_empty() {
            return LawDecision::Rejected { violations };
        }

        record.states.push(next);
        record.transitions.push(transition);
        LawDecision::Accepted(record)
    }

    fn check_step_sequence(existing: &[WorkflowState], next: Step) -> bool {
        use Step::*;
        if existing.is_empty() {
            return next == Describe;
        }
        let last = existing.last().unwrap().step;
        matches!(
            (last, next),
            (Describe, Normalize)
                | (Normalize, Graph)
                | (Graph, StressTest)
                | (StressTest, Architect)
                | (Architect, Question)
                | (Question, Experiment)
                | (Experiment, Record)
                | (Record, Record)
        )
    }

    fn check_consent(token: &ConsentToken, subject: &Did) -> bool {
        token.revocable
            && token.subject_did.method == subject.method
            && token.subject_did.id == subject.id
            && token.expires_at > token.issued_at
    }

    fn check_sovereignty_monotone(
        existing: &[WorkflowState],
        next: &SovereigntyProfile,
    ) -> bool {
        if let Some(last) = existing.last() {
            let l = &last.sovereignty;
            next.max_risk <= l.max_risk
                && next.ecoscore_floor >= l.ecoscore_floor
                && (!l.allow_neural_ops || next.allow_neural_ops)
                && (!l.allow_robotic_ops || next.allow_robotic_ops)
                && (!l.allow_ecological_ops || next.allow_ecological_ops)
        } else {
            true
        }
    }
}

#[derive(Clone)]
pub struct LawfulChatOrchestrator {
    required_sections: Vec<Step>,
}

impl LawfulChatOrchestrator {
    pub fn new() -> Self {
        Self {
            required_sections: vec![
                Step::Describe,
                Step::Normalize,
                Step::Graph,
                Step::StressTest,
                Step::Architect,
                Step::Question,
                Step::Experiment,
                Step::Record,
            ],
        }
    }

    pub fn append_step(
        &self,
        mut record: GovernanceRecord,
        next: WorkflowStep,
    ) -> LawDecision {
        let mut violations = Vec::new();

        if !self.check_step_sequence(&record.steps, next.step) {
            violations.push(LawViolation::StepOrder(format!(
                "illegal transition to {:?}",
                next.step
            )));
        }

        if !self.check_required_fields(&next) {
            violations.push(LawViolation::MissingSection(
                "assumptions or constraints missing".into(),
            ));
        }

        if next.ndm_score > next.ai.ndm_ceiling {
            violations.push(LawViolation::NdmExceeded {
                current: next.ndm_score,
                ceiling: next.ai.ndm_ceiling,
            });
        }

        if next.ecoscore_floor < 0.86 {
            violations.push(LawViolation::EcoFloor(
                "ecoscore_floor below 0.86".into(),
            ));
        }

        if !violations.is_empty() {
            record.violated_invariants
                .extend(violations.iter().map(|v| format!("{:?}", v)));
            return LawDecision::Rejected { violations };
        }

        record.steps.push(next);
        record.applied_invariants.push("AssumptionTransparency".into());
        record.applied_invariants.push("WorkflowGraph".into());
        record.applied_invariants.push("StressTest".into());
        record.applied_invariants.push("PluralArchitecture".into());
        record.applied_invariants.push("ResearchFeedback".into());

        LawDecision::Accepted(record)
    }

    fn check_step_sequence(&self, existing: &[WorkflowStep], next: Step) -> bool {
        use Step::*;
        if existing.is_empty() {
            return next == Describe;
        }
        let last = existing.last().unwrap().step;
        matches!(
            (last, next),
            (Describe, Normalize)
                | (Normalize, Graph)
                | (Graph, StressTest)
                | (StressTest, Architect)
                | (Architect, Question)
                | (Question, Experiment)
                | (Experiment, Record)
                | (Record, Record)
        )
    }

    fn check_required_fields(&self, step: &WorkflowStep) -> bool {
        !step.assumptions.is_empty() && !step.constraints.is_empty()
    }
}
