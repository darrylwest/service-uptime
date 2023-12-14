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

    /// format the elements in a standard way
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        buf.push_str("uptime: ");
        buf.push_str(self.uptime.to_string().as_str());
        let count = self.errors.count();
        buf.push_str(format!(", errors: {}", count).as_str());
        let count = self.access.count();
        buf.push_str(format!(", access: {}", count).as_str());

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let status = ServiceStatus::create();
        let ss = status.to_string();
        println!("{}", ss);
    }

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
        let mut status = ServiceStatus::create();
        println!("{:?}", status.errors);
        assert_eq!(status.errors.count(), 0);
        let count = status.errors.incr();
        assert_eq!(count, 1);
    }

    #[test]
    fn access() {
        let mut status = ServiceStatus::create();
        println!("{:?}", status.access);
        assert_eq!(status.access.count(), 0);
        let count = status.access.incr();
        assert_eq!(count, 1);
    }
}
