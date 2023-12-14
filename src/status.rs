use crate::counter::Counter;
use crate::uptime::Uptime;
#[derive(Debug)]
pub struct ServiceStatus {
    pub uptime: Uptime,
    pub errors: Counter,
    pub access: Counter,
}

impl ServiceStatus {
    pub fn create() -> ServiceStatus {
        ServiceStatus {
            uptime: Uptime::new(),
            errors: Counter::create(),
            access: Counter::create(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let status = ServiceStatus::create();
        println!("{:?}", status);
    }
}
