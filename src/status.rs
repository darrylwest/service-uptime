///
/// use the service status struct to create uptime, errors, and access in one place,
///
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
        assert_eq!(status.uptime.to_string(), "0 days, 00:00:00 hms");
    }

    #[test]
    fn uptime() {
        let status = ServiceStatus::create();
        println!("{}", status.uptime.to_string());
        assert_eq!(status.uptime.to_string(), "0 days, 00:00:00 hms");
    }

    #[test]
    fn errors() {
        let status = ServiceStatus::create();
        println!("{:?}", status.errors);
        assert_eq!(status.errors.count(), 0);
    }
}
