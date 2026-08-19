#[derive(Debug, Clone, PartialEq)]
enum TicketStatus { Open, InProgress, Resolved, Closed }

#[derive(Debug, Clone)]
struct Ticket {
    id: u64,
    title: String,
    status: TicketStatus,
    assignee: Option<String>,
}

impl Ticket {
    fn new(id: u64, title: &str) -> Self {
        Ticket { id, title: title.into(), status: TicketStatus::Open, assignee: None }
    }

    fn assign(&self, assignee: &str) -> Result<Self, String> {
        if self.status != TicketStatus::Open { return Err("Can only assign open tickets".into()); }
        Ok(Ticket { status: TicketStatus::InProgress, assignee: Some(assignee.into()), ..self.clone() })
    }

    fn resolve(&self) -> Result<Self, String> {
        if self.status != TicketStatus::InProgress { return Err("Can only resolve in-progress tickets".into()); }
        Ok(Ticket { status: TicketStatus::Resolved, ..self.clone() })
    }

    fn close(&self) -> Result<Self, String> {
        if self.status != TicketStatus::Resolved { return Err("Can only close resolved tickets".into()); }
        Ok(Ticket { status: TicketStatus::Closed, ..self.clone() })
    }
}

fn main() {}
