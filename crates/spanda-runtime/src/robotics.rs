//! Mission lifecycle and fleet grouping runtime state.
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mission lifecycle states tracked at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MissionState {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
}

impl MissionState {
    pub fn as_str(self) -> &'static str {
        // Description:
        //     As str.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: &'static str
        //         Return value from `as_str`.
        //
        // Example:

        //     let result = instance.as_str();

        match self {
            Self::Pending => "Pending",
            Self::Running => "Running",
            Self::Paused => "Paused",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
        }
    }
}

/// Runtime mission controller for named step sequences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionRuntime {
    pub name: Option<String>,
    pub steps: Vec<String>,
    pub state: MissionState,
    pub step_index: usize,
    pub duration_hours: Option<f64>,
}

impl MissionRuntime {
    pub fn new(name: Option<String>, steps: Vec<String>, duration_hours: Option<f64>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     name: Option<String>
        //         Caller-supplied name.
        //     steps: Vec<String>
        //         Caller-supplied steps.
        //     duration_hours: Option<f64>
        //         Caller-supplied duration hours.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_runtime::robotics::new(name, steps, duration_hours);

        Self {
            name,
            steps,
            state: MissionState::Pending,
            step_index: 0,
            duration_hours,
        }
    }

    pub fn start(&mut self) {
        // Description:
        //     Start.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::start(&mut self);

        if self.state == MissionState::Pending {
            self.state = MissionState::Running;
        }
    }

    pub fn pause(&mut self) {
        // Description:
        //     Pause.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::pause(&mut self);

        if self.state == MissionState::Running {
            self.state = MissionState::Paused;
        }
    }

    pub fn resume(&mut self) {
        // Description:
        //     Resume.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::resume(&mut self);

        if self.state == MissionState::Paused {
            self.state = MissionState::Running;
        }
    }

    pub fn restart(&mut self) {
        self.step_index = 0;
        self.state = MissionState::Running;
    }

    pub fn restart_current_step(&mut self) {
        if self.step_index > 0 {
            self.step_index -= 1;
        }
        self.state = MissionState::Running;
    }

    pub fn advance(&mut self) -> Option<String> {
        // Description:
        //     Advance.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `advance`.
        //
        // Example:

        //     let result = spanda_runtime::robotics::advance(&mut self);

        if self.state != MissionState::Running {
            return None;
        }
        if self.step_index >= self.steps.len() {
            self.state = MissionState::Completed;
            return None;
        }
        let step = self.steps[self.step_index].clone();
        self.step_index += 1;
        if self.step_index >= self.steps.len() {
            self.state = MissionState::Completed;
        }
        Some(step)
    }

    pub fn complete(&mut self) {
        // Description:
        //     Complete.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::complete(&mut self);

        self.state = MissionState::Completed;
        self.step_index = self.steps.len();
    }

    pub fn fail(&mut self) {
        // Description:
        //     Fail.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::fail(&mut self);

        self.state = MissionState::Failed;
    }

    pub fn current_step(&self) -> Option<&str> {
        // Description:
        //     Current step.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Option<&str>
        //         Return value from `current_step`.
        //
        // Example:

        //     let result = spanda_runtime::robotics::current_step(&self);

        if self.state != MissionState::Running {
            return None;
        }
        self.steps.get(self.step_index).map(String::as_str)
    }
}

/// Registry of fleet groups declared at program scope.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FleetRegistry {
    fleets: HashMap<String, Vec<String>>,
}

impl FleetRegistry {
    pub fn register(&mut self, name: &str, members: Vec<String>) {
        // Description:
        //     Register.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     embers: Vec<String>
        //         Caller-supplied embers.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::register(&mut self, name, embers);

        self.fleets.insert(name.to_string(), members);
    }

    pub fn members(&self, name: &str) -> Option<&[String]> {
        // Description:
        //     Members.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<&[String]>
        //         Return value from `members`.
        //
        // Example:

        //     let result = spanda_runtime::robotics::members(&self, name);

        self.fleets.get(name).map(Vec::as_slice)
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        // Description:
        //     Names.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: impl Iterator<Item = &String>
        //         Return value from `names`.
        //
        // Example:

        //     let result = spanda_runtime::robotics::names(&self);

        self.fleets.keys()
    }
}

/// Program-level safety zone speed policies keyed by zone name.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProgramSafetyZoneRegistry {
    zones: HashMap<String, f64>,
}

impl ProgramSafetyZoneRegistry {
    pub fn register(&mut self, name: &str, max_speed_mps: f64) {
        // Description:
        //     Register.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     ax_speed_mps: f64
        //         Caller-supplied ax speed mps.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::register(&mut self, name, ax_speed_mps);

        self.zones.insert(name.to_string(), max_speed_mps);
    }

    pub fn max_speed_for(&self, zone_name: &str) -> Option<f64> {
        // Description:
        //     Max speed for.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     zone_name: &str
        //         Caller-supplied zone name.
        //
        // Outputs:
        //     result: Option<f64>
        //         Return value from `max_speed_for`.
        //
        // Example:

        //     let result = spanda_runtime::robotics::max_speed_for(&self, zone_name);

        self.zones.get(zone_name).copied()
    }

    pub fn speed_caps(&self) -> &HashMap<String, f64> {
        // Description:
        //     Speed caps.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &HashMap<String, f64>
        //         Return value from `speed_caps`.
        //
        // Example:

        //     let result = spanda_runtime::robotics::speed_caps(&self);

        &self.zones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mission_advances_through_steps() {
        // Description:
        //     Mission advances through steps.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::mission_advances_through_steps();

        let mut mission = MissionRuntime::new(
            Some("Delivery".into()),
            vec!["navigate".into(), "deliver".into()],
            Some(0.5),
        );
        mission.start();
        assert_eq!(mission.advance(), Some("navigate".into()));
        assert_eq!(mission.advance(), Some("deliver".into()));
        assert_eq!(mission.state, MissionState::Completed);
    }

    #[test]
    fn fleet_registry_resolves_members() {
        // Description:
        //     Fleet registry resolves members.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::robotics::fleet_registry_resolves_members();

        let mut reg = FleetRegistry::default();
        reg.register("alpha", vec!["r1".into(), "r2".into()]);
        assert_eq!(
            reg.members("alpha"),
            Some(["r1".into(), "r2".into()].as_slice())
        );
    }
}
