use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};
use crate::eventsourcing::user_command::models::UserCommand;
use crate::eventsourcing::user_event::models::UserEvent;
use crate::eventsourcing::user_state::models::UserState;

impl Aggregate for PUser {
    type Event = UserEvent;
    type Command = UserCommand;
    type State = UserState;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        let user_data = match evt {
            EmployeeEvent::EmployeeCreated(Employee) => EmployeeState {
                emp: Employee,
                generation: state.generation + 1,
            },
            EmployeeEvent::EmployeeUpdated(salary) => EmployeeState {
                emp: Employee {
                    emp_salary: salary,
                    ..Employee
                },
                generation: state.generation + 1,
            },
            EmployeeEvent::EmployeeDeleted(emp_id) => EmployeeState {
                emp: Employee {
                    emp_id: 0,
                    emp_name: "",
                    emp_salary: 0,
                },
                generation: state.generation + 1,
            },
            _ => "invalid event",
        };
        Ok(emp_data)
    }

    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        let evts = match cmd {
            BankCommand::DepositFunds(acct, amt) => vec![BankEvent::FundsDeposited(acct, amt)],
            BankCommand::WithdrawFunds(acct, amt) => vec![BankEvent::FundsWithdrawn(acct, amt)],
        };
        Ok(evts)
    }
}
