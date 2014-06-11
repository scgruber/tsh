use std::fmt;

/// Record for jobs (top-level commands run from tsh).
struct Job {
  jid: int,
  pid: int,
  cmd: String
}

impl PartialEq for Job {
  fn eq(&self, other: &Job) -> bool {
    (self.jid == other.jid) && (self.pid == other.pid) && (self.cmd.equiv(&other.cmd))
  }
}

impl fmt::Show for Job {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\t{}\t{}", self.jid, self.pid, self.cmd)
  }
}

/// Record for the list of all active jobs.
/// We maintain the invariant that `jobs` is a sorted vector by JID
pub struct JobsList{
  jobs: Vec<Job>,
  max_jid: int
}

impl JobsList {
  /// Creates a new empty JobsList.
  pub fn new() -> JobsList {
    JobsList{ jobs: vec![], max_jid: 0 }
  }

  /// Adds a new job to the list.
  pub fn push(&mut self, pid: int, cmd: String) {
    self.jobs.push(Job{jid: self.max_jid, pid: pid, cmd: cmd});
    self.max_jid = self.max_jid + 1;
  }

  /// Looks up a job's JID from its PID.
  pub fn jid_from_pid(&self, pid: int) -> Option<int> {
    for job in self.jobs.iter() {
      if job.pid == pid {
        return Some(job.jid)
      } else {
        ()
      }
    }
    return None
  }

  /// Looks for a job by JID. If the job is found, returns the tuple of that 
  /// job's PID and its command line. If the job is not found, returns None.
  pub fn extract(&mut self, jid: int) -> Option<(int, String)> {
    let mut lower_bound = 0;
    let mut upper_bound = self.jobs.len();
    loop {
      if (jid > self.jobs.get(upper_bound-1).jid) || 
        (jid < self.jobs.get(lower_bound).jid) {
        // Search out of bounds, could not find job
        return None
      } else {
        let mid = ((upper_bound - lower_bound) / 2) + lower_bound;
        if jid == self.jobs.get(mid).jid {
          // This is the job we are trying to extract
          match self.jobs.remove(mid) {
            Some(Job {pid, cmd, ..}) => return Some((pid, cmd)),
            None => fail!("Unexpected out of bounds in jobs vector!")
          }
        } else if jid > self.jobs.get(mid).jid {
          lower_bound = mid+1;
        } else {
          upper_bound = mid;
        }
      }
    }
  }
}

#[test]
fn jobslist_push_zeroes() {
  let mut list = JobsList::new();
  list.push(0, "".to_string());
  assert_eq!(list.max_jid, 1);
  assert_eq!(list.jobs, vec![Job{jid:0, pid:0, cmd: "".to_string()}]);
}

#[test]
fn jobslist_jid_from_pid_simple() {
  let mut list = JobsList::new();
  list.push(1234, "foo".to_string());
  list.push(9876, "bar".to_string());
  list.push(1010, "baz".to_string());
  assert_eq!(list.jid_from_pid(9876), Some(1));
}

#[test]
fn jobslist_jid_from_pid_none() {
  let mut list = JobsList::new();
  list.push(1234, "foo".to_string());
  list.push(9876, "bar".to_string());
  list.push(1010, "baz".to_string());
  assert_eq!(list.jid_from_pid(2468), None);
}

#[test]
fn jobslist_extract_first() {
  let mut list = JobsList::new();
  list.push(1234, "foo".to_string());
  list.push(2048, "spiff".to_string());
  list.push(9876, "bar".to_string());
  list.push(7997, "buzz".to_string());
  list.push(1010, "baz".to_string());
  assert_eq!(list.extract(0), Some((1234, "foo".to_string())));
}

#[test]
fn jobslist_extract_middle() {
  let mut list = JobsList::new();
  list.push(1234, "foo".to_string());
  list.push(2048, "spiff".to_string());
  list.push(9876, "bar".to_string());
  list.push(7997, "buzz".to_string());
  list.push(1010, "baz".to_string());
  assert_eq!(list.extract(2), Some((9876, "bar".to_string())));
}

#[test]
fn jobslist_extract_last() {
  let mut list = JobsList::new();
  list.push(1234, "foo".to_string());
  list.push(2048, "spiff".to_string());
  list.push(9876, "bar".to_string());
  list.push(7997, "buzz".to_string());
  list.push(1010, "baz".to_string());
  assert_eq!(list.extract(4), Some((1010, "baz".to_string())));
}

impl fmt::Show for JobsList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut repr: String = "JID\tPID\tCMD\n".to_string();
    for job in self.jobs.iter() {
      repr = repr.append(format!("{}\n", job).as_slice());
    }
    write!(f, "{}", repr)
  }
}