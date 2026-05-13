use super::*;

impl Model {
    #[allow(dead_code)]
    pub(crate) fn tier(&self) -> Result<PlanTier, tonic::Status> {
        self.tier
            .parse()
            .map_err(|_| tonic::Status::invalid_argument("Invalid plan tier."))
    }
}

impl From<Model> for OrganizationPlan {
    fn from(plan: Model) -> Self {
        Self {
            organization_pid: plan.organization_id.to_string(),
            tier: plan.tier.clone(),
            expires_at: plan.expires_at.map(|d| d.and_utc().to_rfc3339()),
            created_at: plan.created_at.and_utc().to_rfc3339(),
            updated_at: plan.updated_at.and_utc().to_rfc3339(),
        }
    }
}
