use crate::{Command, GameState};

/// Returns the next state without mutating `state`.
#[must_use]
pub fn step(state: &GameState, command: &Command) -> GameState {
    let mut next = state.clone();

    match command {
        Command::AdvanceTime { ticks } => {
            next.time.tick = next.time.tick.saturating_add(*ticks);
        }
        Command::MoveUnit {
            unit_id,
            destination,
        } => {
            if destination.x.is_finite()
                && destination.y.is_finite()
                && let Some(unit) = next.units.iter_mut().find(|unit| unit.id == *unit_id)
            {
                unit.position = *destination;
            }
        }
    }

    next
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::{GameTime, UnitId, Vec2};

    #[test]
    fn initial_state_is_valid() {
        let state = GameState::default();
        assert_eq!(state.time, GameTime { tick: 0 });
        assert_eq!(state.units.len(), 2);
        assert!(state.units.iter().all(|unit| unit.health > 0));
    }

    #[test]
    fn command_creates_a_new_state_without_mutating_the_original() {
        let original = GameState::default();
        let snapshot = original.clone();
        let next = step(
            &original,
            &Command::MoveUnit {
                unit_id: UnitId(1),
                destination: Vec2::new(5.0, -2.0),
            },
        );

        assert_eq!(original, snapshot);
        assert_ne!(next, original);
        assert_eq!(next.units[0].position, Vec2::new(5.0, -2.0));
    }

    #[test]
    fn identical_inputs_produce_identical_results() {
        let state = GameState::default();
        let command = Command::AdvanceTime { ticks: 3 };
        assert_eq!(step(&state, &command), step(&state, &command));
    }

    #[test]
    fn invalid_command_does_not_damage_state() {
        let state = GameState::default();
        let missing_unit = Command::MoveUnit {
            unit_id: UnitId(999),
            destination: Vec2::new(1.0, 1.0),
        };
        let non_finite = Command::MoveUnit {
            unit_id: UnitId(1),
            destination: Vec2::new(f32::NAN, 1.0),
        };

        assert_eq!(step(&state, &missing_unit), state);
        assert_eq!(step(&state, &non_finite), state);
    }

    proptest! {
        #[test]
        fn advancing_time_is_monotonic(start in any::<u64>(), delta in any::<u64>()) {
            let mut state = GameState::default();
            state.time.tick = start;
            let next = step(&state, &Command::AdvanceTime { ticks: delta });

            prop_assert!(next.time.tick >= state.time.tick);
            prop_assert_eq!(next.time.tick, start.saturating_add(delta));
            prop_assert_eq!(next.units, state.units);
        }
    }
}
