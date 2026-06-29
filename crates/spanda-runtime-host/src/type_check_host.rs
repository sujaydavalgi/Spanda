//! Core-backed [`TypeCheckHost`] hooks for the extracted program type checker.
//!
use spanda_ai::resolve_ai_import;
use spanda_ast::nodes::{HalMemberDecl, Span, SpandaType};
use spanda_ffi::resolve_ffi_import;
use spanda_hal::{get_soc_profile, hal_member_from_decl, soc::validate_hal_against_soc};
use spanda_lib_registry::{all_library_sensor_types, resolve_import};
use spanda_typecheck::{
    import_catalog::resolve_package_import, resolve_std_import, security_capabilities,
    validate_resource_budget, validate_task_priority, validate_task_timing, Diagnostic,
    TypeCheckHost,
};
use std::collections::HashMap;

use crate::robotics_validation::{
    validate_fleet_members, validate_mission_decl, validate_swarm_fleet,
};
use crate::slam_adapter;

/// Default host wiring domain registries into `spanda-typecheck`.
pub struct CoreTypeCheckHost;

impl TypeCheckHost for CoreTypeCheckHost {
    fn import_path_known(&self, path: &str, module_registry_has_export: bool) -> bool {
        // Description:
        //     Import path known.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     path: &str
        //         Caller-supplied path.
        //     odule_registry_has_expor: bool
        //         Caller-supplied odule registry has expor.
        //
        // Outputs:
        //     result: bool
        //         Return value from `import_path_known`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::import_path_known(&self, path, odule_registry_has_expor);

        resolve_import(path).is_some()
            || resolve_ai_import(path).is_some()
            || resolve_std_import(path)
            || resolve_ffi_import(path)
            || resolve_package_import(path)
            || module_registry_has_export
    }

    fn slam_import_known(&self, path: &str) -> bool {
        // Description:
        //     Slam import known.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     path: &str
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: bool
        //         Return value from `slam_import_known`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::slam_import_known(&self, path);

        slam_adapter::slam_import_paths().contains(&path)
    }

    fn library_exports_sensor(&self, library: &str, sensor_type: &str) -> Option<bool> {
        // Description:
        //     Library exports sensor.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     library: &str
        //         Caller-supplied library.
        //     sensor_type: &str
        //         Caller-supplied sensor type.
        //
        // Outputs:
        //     result: Option<bool>
        //         Return value from `library_exports_sensor`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::library_exports_sensor(&self, library, sensor_type);

        resolve_import(library).map(|module| module.sensors.contains_key(sensor_type))
    }

    fn library_sensor_type_known(&self, sensor_type: &str) -> bool {
        // Description:
        //     Library sensor type known.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     sensor_type: &str
        //         Caller-supplied sensor type.
        //
        // Outputs:
        //     result: bool
        //         Return value from `library_sensor_type_known`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::library_sensor_type_known(&self, sensor_type);

        all_library_sensor_types().contains_key(sensor_type)
    }

    fn library_sensor_robo_types(&self) -> HashMap<String, SpandaType> {
        // Description:
        //     Library sensor robo types.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: HashMap<String, SpandaType>
        //         Return value from `library_sensor_robo_types`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::library_sensor_robo_types(&self);

        all_library_sensor_types()
            .into_iter()
            .map(|(name, info)| (name, info.robo_type))
            .collect()
    }

    fn library_for_sensor_type(&self, sensor_type: &str) -> Option<String> {
        // Description:
        //     Library for sensor type.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     sensor_type: &str
        //         Caller-supplied sensor type.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `library_for_sensor_type`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::library_for_sensor_type(&self, sensor_type);

        all_library_sensor_types()
            .get(sensor_type)
            .map(|info| info.library.clone())
    }

    fn soc_profile_known(&self, profile: &str) -> bool {
        // Description:
        //     Soc profile known.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     profile: &str
        //         Caller-supplied profile.
        //
        // Outputs:
        //     result: bool
        //         Return value from `soc_profile_known`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::soc_profile_known(&self, profile);

        get_soc_profile(profile).is_some()
    }

    fn validate_hal_against_soc(&self, profile: &str, members: &[HalMemberDecl]) -> Vec<String> {
        // Description:
        //     Validate hal against soc.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     profile: &str
        //         Caller-supplied profile.
        //     embers: &[HalMemberDecl]
        //         Caller-supplied embers.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `validate_hal_against_soc`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::validate_hal_against_soc(&self, profile, embers);

        let Some(soc) = get_soc_profile(profile) else {
            return Vec::new();
        };
        let hal_members: Vec<_> = members.iter().map(hal_member_from_decl).collect();
        validate_hal_against_soc(&soc, &hal_members)
            .into_iter()
            .map(|d| d.message)
            .collect()
    }

    fn validate_fleet_members(
        &self,
        fleet_name: &str,
        members: &[String],
        robot_names: &[String],
    ) -> Option<String> {
        // Description:
        //     Validate fleet members.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     fleet_name: &str
        //         Caller-supplied fleet name.
        //     embers: &[String]
        //         Caller-supplied embers.
        //     robot_names: &[String]
        //         Caller-supplied robot names.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `validate_fleet_members`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::validate_fleet_members(&self, fleet_name, embers, robot_names);

        validate_fleet_members(fleet_name, members, robot_names)
    }

    fn validate_swarm_fleet(
        &self,
        swarm_name: &str,
        fleet_name: &str,
        fleet_names: &[String],
    ) -> Option<String> {
        // Description:
        //     Validate swarm fleet.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     swarm_name: &str
        //         Caller-supplied swarm name.
        //     fleet_name: &str
        //         Caller-supplied fleet name.
        //     fleet_names: &[String]
        //         Caller-supplied fleet names.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `validate_swarm_fleet`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::validate_swarm_fleet(&self, swarm_name, fleet_name, fleet_names);

        validate_swarm_fleet(swarm_name, fleet_name, fleet_names)
    }

    fn validate_mission_decl(
        &self,
        name: &Option<String>,
        duration_hours: Option<f64>,
        steps: &[String],
    ) -> Option<String> {
        // Description:
        //     Validate mission decl.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &Option<String>
        //         Caller-supplied name.
        //     duration_hours: Option<f64>
        //         Caller-supplied duration hours.
        //     steps: &[String]
        //         Caller-supplied steps.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `validate_mission_decl`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::validate_mission_decl(&self, name, duration_hours, steps);

        validate_mission_decl(name, duration_hours, steps)
    }

    fn security_capability_known(&self, capability: &str) -> bool {
        // Description:
        //     Security capability known.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     capability: &str
        //         Caller-supplied capability.
        //
        // Outputs:
        //     result: bool
        //         Return value from `security_capability_known`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::security_capability_known(&self, capability);

        security_capabilities::is_known_capability(capability)
    }

    fn validate_task_timing(&self, task: &spanda_ast::foundations::TaskDecl) -> Vec<Diagnostic> {
        // Description:
        //     Validate task timing.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     ask: &spanda_ast::foundations::TaskDecl
        //         Caller-supplied ask.
        //
        // Outputs:
        //     result: Vec<Diagnostic>
        //         Return value from `validate_task_timing`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::validate_task_timing(&self, ask);

        validate_task_timing(task)
    }

    fn validate_task_priority(&self, task: &spanda_ast::foundations::TaskDecl) -> Vec<Diagnostic> {
        // Description:
        //     Validate task priority.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     ask: &spanda_ast::foundations::TaskDecl
        //         Caller-supplied ask.
        //
        // Outputs:
        //     result: Vec<Diagnostic>
        //         Return value from `validate_task_priority`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::validate_task_priority(&self, ask);

        validate_task_priority(task)
    }

    fn validate_resource_budget(
        &self,
        budget: &spanda_ast::foundations::ResourceBudgetDecl,
        span: Span,
    ) -> Vec<Diagnostic> {
        // Description:
        //     Validate resource budget.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     budge: &spanda_ast::foundations::ResourceBudgetDecl
        //         Caller-supplied budge.
        //     span: Span
        //         Caller-supplied span.
        //
        // Outputs:
        //     result: Vec<Diagnostic>
        //         Return value from `validate_resource_budget`.
        //
        // Example:

        //     let result = spanda_runtime_host::type_check_host::validate_resource_budget(&self, budge, span);

        validate_resource_budget(budget, span)
    }
}

/// Shared core host instance for type-check entry points.
pub fn core_type_check_host() -> &'static CoreTypeCheckHost {
    // Description:
    //     Core type check host.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static CoreTypeCheckHost
    //         Return value from `core_type_check_host`.
    //
    // Example:

    //     let result = spanda_runtime_host::type_check_host::core_type_check_host();

    &CoreTypeCheckHost
}
