use crate::{
    auth_n::oidc::utilities::set_user_password, fhir_client::ServerCTX, services::ServerState,
    ui::components::TenantName,
};
use haste_fhir_client::FHIRClient;
use haste_fhir_model::r4::generated::{
    resources::{Project, Resource, ResourceType, User},
    terminology::{IssueType, SupportedFhirVersion},
    types::FHIRString,
};
use haste_fhir_operation_error::OperationOutcomeError;
use haste_fhir_search::SearchEngine;
use haste_fhir_terminology::FHIRTerminology;
use haste_jwt::{ProjectId, TenantId, claims::SubscriptionTier};
use haste_repository::{
    Repository,
    admin::TenantModelAdmin,
    types::{
        tenant::{CreateTenant, Tenant},
        user::CreateUser,
    },
    utilities::generate_id,
};
use std::sync::Arc;

/// Creates a user for a tenant and optionally sets the user's password.
///
/// The user is created as a FHIR `User` resource within the tenant's system
/// project. If a password is provided, it is set after the user has been
/// successfully created.
///
/// # Arguments
///
/// * `services` - Server state providing the FHIR client, repository, and
///   rate-limiting services used to create and configure the user.
/// * `tenant` - Identifier of the tenant that will own the user.
/// * `user_resource` - FHIR `User` resource to create.
/// * `password` - Optional password to set for the newly created user.
///
/// # Returns
///
/// Returns the created [`User`] resource.
///
/// # Errors
///
/// Returns [`OperationOutcomeError`] if:
///
/// * creating the user through the FHIR client fails; or
/// * setting the user's password fails when a password is provided.
///
/// # Panics
///
/// Panics if the FHIR client returns a resource other than [`User`],
/// or if the created user does not have an ID.
pub async fn create_user<
    Repo: Repository + Send + Sync + 'static,
    Search: SearchEngine + Send + Sync + 'static,
    Terminology: FHIRTerminology + Send + Sync + 'static,
>(
    services: &ServerState<Repo, Search, Terminology>,
    tenant: &TenantId,
    user_resource: User,
    password: Option<&str>,
) -> Result<User, OperationOutcomeError> {
    let ctx = Arc::new(ServerCTX::system(
        tenant.clone(),
        ProjectId::System,
        services.fhir_client.clone(),
        services.rate_limit.clone(),
    ));

    let user = services
        .fhir_client
        .create(ctx, ResourceType::User, Resource::User(user_resource))
        .await?;

    let Resource::User(user) = user else { panic!("Created resource is not a User") };

    let user_id = user.id.clone().unwrap();

    if let Some(password) = password {
        set_user_password(
            &*services.repo,
            tenant,
             &user
                 .email
                 .as_ref()
                 .and_then(|e| e.value.as_ref()).cloned()
                .unwrap_or_default(),
            &user_id,
            password,
        )
        .await?;
    }

    Ok(user)
}

pub struct CreateTenantOutput {
    pub tenant: Tenant,
    pub owner: haste_repository::types::user::User,
}

/// Reads a tenant by its identifier.
///
/// The tenant is looked up from the system tenant's repository context. If
/// the tenant exists, its [`Tenant`] model is returned.
///
/// # Arguments
///
/// * `services` - Server state containing the repository used to read the
///   tenant.
/// * `tenant_id` - Identifier of the tenant to retrieve.
///
/// # Returns
///
/// Returns the [`Tenant`] associated with `tenant_id`.
///
/// # Errors
///
/// Returns [`OperationOutcomeError`] if:
///
/// * the repository fails while reading the tenant; or
/// * no tenant with the specified identifier exists, in which case a
///   `not-found` operation outcome is returned.
pub async fn read_tenant<
    Repo: Repository + Send + Sync + 'static,
    Search: SearchEngine + Send + Sync + 'static,
    Terminology: FHIRTerminology + Send + Sync + 'static,
>(
    services: &ServerState<Repo, Search, Terminology>,
    tenant_id: &TenantId,
) -> Result<Tenant, OperationOutcomeError> {
    TenantModelAdmin::<CreateTenant, _, _, _, _>::read(
        services.repo.as_ref(),
        &TenantId::System,
        &tenant_id.as_ref().to_string(),
    )
    .await?
    .ok_or_else(|| {
        OperationOutcomeError::error(
            IssueType::not_found(),
            format!("Tenant '{tenant_id}' was not found."),
        )
    })
}

#[must_use]
pub fn tenant_name(tenant: &Tenant) -> TenantName {
    TenantName(tenant.display_name.clone())
}

/// Creates a new tenant and provisions its initial FHIR project and owner.
///
/// The tenant is created inside a transaction. After the tenant has been
/// created, a system FHIR project is provisioned for it and the supplied user
/// is created as the tenant owner. The owner is then read back from the
/// repository so that the returned value reflects the persisted user.
///
/// The transaction is committed only after all tenant, project, and owner
/// operations have completed successfully. If any operation fails before the
/// commit, the transaction is not committed.
///
/// # Arguments
///
/// * `services` - Server state providing the repository, FHIR client, and
///   supporting services used during tenant provisioning.
/// * `tenant_id` - Optional identifier for the new tenant. When `None`, a
///   random 16-character identifier is generated.
/// * `_name` - Reserved tenant name parameter. Currently unused.
/// * `subscription_tier` - Subscription tier to assign to the new tenant.
/// * `owner` - FHIR `User` resource to create as the tenant owner.
/// * `owner_password` - Optional password for the tenant owner.
///
/// # Returns
///
/// Returns a [`CreateTenantOutput`] containing the newly created tenant and
/// its persisted owner.
///
/// # Errors
///
/// Returns [`OperationOutcomeError`] if:
///
/// * the tenant transaction cannot be started;
/// * creating the tenant fails;
/// * provisioning the tenant's system FHIR project fails;
/// * creating the tenant owner fails;
/// * the created owner does not contain an ID;
/// * the owner cannot be read back from the repository after creation; or
/// * committing the transaction fails.
///
/// An error is returned if the owner has no ID after creation because the
/// owner must be persisted and returned as part of the completed tenant
/// creation process. An error is also returned if the owner cannot be found
/// after creation, as this indicates that the tenant provisioning process did
/// not produce the expected persisted state.
pub async fn create_tenant<
    Repo: Repository + Send + Sync + 'static,
    Search: SearchEngine + Send + Sync + 'static,
    Terminology: FHIRTerminology + Send + Sync + 'static,
>(
    services: &ServerState<Repo, Search, Terminology>,
    tenant_id: Option<String>,
    _name: &str,
    subscription_tier: &SubscriptionTier,
    owner: haste_fhir_model::r4::generated::resources::User,
    owner_password: Option<&str>,
) -> Result<CreateTenantOutput, OperationOutcomeError> {
    let services = services.transaction().await?;

    let new_tenant = TenantModelAdmin::create(
        &*services.repo,
        &TenantId::System,
        CreateTenant {
            id: Some(TenantId::new(tenant_id.unwrap_or(generate_id(Some(16))))),
            subscription_tier: Some(subscription_tier.clone().into()),
            display_name: None,
            logo_data: None,
            logo_content_type: None,
        },
    )
    .await?;

    services
        .fhir_client
        .create(
            Arc::new(ServerCTX::system(
                new_tenant.id.clone(),
                ProjectId::System,
                services.fhir_client.clone(),
                services.rate_limit.clone(),
            )),
            ResourceType::Project,
            Resource::Project(Project {
                id: Some(ProjectId::System.to_string()),
                name: Box::new(FHIRString {
                    value: Some(ProjectId::System.to_string()),
                    ..Default::default()
                }),
                fhirVersion: SupportedFhirVersion::r4(),
                ..Default::default()
            }),
        )
        .await?;

    let user = create_user(&services, &new_tenant.id, owner, owner_password).await?;

    let Some(user_id) = user.id else {
        return Err(OperationOutcomeError::fatal(
            IssueType::invalid(),
            "The user ID is required to complete the tenant creation process.".to_string(),
        ));
    };

    let Some(user) = TenantModelAdmin::<CreateUser, _, _, _, _>::read(
        services.repo.as_ref(),
        &new_tenant.id,
        &user_id,
    )
    .await?
    else {
        return Err(OperationOutcomeError::fatal(
            IssueType::invalid(),
            "The user does not exist after creation.".to_string(),
        ));
    };

    services.commit().await?;

    Ok(CreateTenantOutput {
        tenant: new_tenant,
        owner: user,
    })
}
