use serde::Deserialize;

/// The `config::remote` module groups configuration
/// specific to remoting into other hosts.
///
/// `topgrade` can upgrade remote hosts using `ssh`.
/// It requires that each remote host have `topgrade`.
///
/// [`steps::ssh`] is where this data is used when
/// updating the remote system.

/// `Remote` represents configuration data specific
/// to a particular host.
#[derive(Deserialize, Debug)]
pub struct Remote {
  /// the hostname of the remote to connect to
  ///
  /// [`ssh`][ssh-manpage] connects and logs into a specified *destination*,
  /// which may be specified as either `[user@]hostname`
  /// or a URI of the form `ssh://[user@]hostname[:port]`
  ///
  /// [ssh-manpage]: <https://www.man7.org/linux/man-pages/man1/ssh.1.html>
  destination: String,

  /// arguments passed to `ssh` when connecting to this remote
  ssh_arguments: Option<Vec<String>>,

  /// the path where `topgrade` is installed on the remote machine
  ///
  /// if left unspecified, we assume
  /// it exists in the remote system's `PATH`
  topgrade_path: Option<String>,
}

pub struct Deprecated {
  remote_topgrades: Option<Vec<String>>,
  remote_topgrade_path: Option<String>,
  ssh_arguments: Option<String>,
}

/// `RemoteBuilder` is a builder-like pattern
/// for constructing a [`Remote`].`
pub struct RemoteBuilder {
  destination: String,
  ssh_arguments: Option<Vec<String>>,
  topgrade_path: Option<String>,
}

impl RemoteBuilder {
  pub fn new(destination: &str) -> RemoteBuilder {
    RemoteBuilder {
      destination: String::from(destination),
      ssh_arguments: None,
      topgrade_path: None,
    }
  }

  pub fn destination(mut self, destination: String) -> RemoteBuilder {
    self.destination = destination;
    self
  }

  pub fn ssh_arguments(mut self, ssh_arguments: Vec<String>) -> RemoteBuilder {
    self.ssh_arguments = Some(ssh_arguments);
    self
  }

  pub fn topgrade_path(mut self, topgrade_path: String) -> RemoteBuilder {
    self.topgrade_path = Some(topgrade_path);
    self
  }

  pub fn build(self) -> Remote {
    Remote {
      destination: self.destination.clone(),
      ssh_arguments: self.ssh_arguments.clone(),
      topgrade_path: self.topgrade_path.clone(),
    }
  }
}

#[cfg(test)]
mod tests {

}
