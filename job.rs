use std::fmt;

/// Record for jobs (top-level commands run from tsh).
struct Job {
  jid: int,
  pid: int,
  cmd: String
}

impl fmt::Show for Job {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\t{}\t{}", self.jid, self.pid, self.cmd)
  }
}

/// Record for the list of all active jobs.
pub struct JobsList(Vec<Job>);

impl JobsList {
  pub fn new() -> JobsList {
    JobsList(vec![])
  }
}

impl fmt::Show for JobsList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut repr: String = "JID\tPID\tCMD\n".to_string();
    let JobsList(ref jobs) = *self;
    for job in jobs.iter() {
      repr = repr.append(format!("{}\n", job).as_slice());
    }
    write!(f, "{}", repr)
  }
}