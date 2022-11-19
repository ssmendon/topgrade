use serde::Deserialize;

/// The `config::remote` module groups configuration
/// data for remoting into a host.
///
/// `topgrade` can upgrade remote hosts using `ssh`.
/// It requires that each remote host have `topgrade`.
///
/// Remote configs can be specified in two parts:
/// global configs and per-host configs. Global configs
/// appear before per-host config data when constructing
/// the `ssh` command string.
///
/// [`steps::ssh`] is where this data is used when
/// updating the remote system.
///
/// # Examples
///
/// ```rust
/// let topgrade_toml = r#"
/// [remote]
/// ssh_arguments = ["-o", "ConnectTimeout=2"]
/// topgrade_path = "~/.cargo/bin/topgrade"
///
/// [[remote.hosts]]
/// destination = "ssh://foo@bar:8080"
/// topgrade_path = "topgrade"
///
/// [[remote.hosts]]
/// destination = "pi@raspberry"
///
/// [[remote.hosts]]
/// destination = "baz"
/// "#;
///
/// let config: Remote = toml::from_str(topgrade_toml).unwrap();
/// assert_eq!(config.ssh_arguments, vec!["-o", "ConnectTimeout=2"]);
/// assert_eq!(config.topgrade_path, "~/.cargo/bin/topgrade");
///
/// assert_eq!(config.hosts.len(), 3);
/// assert_eq!(config.hosts[0].destination, "ssh://foo@bar:8080");
/// assert_eq!(config.hosts[0].topgrade_path, "topgrade");
/// assert_eq!(config.hosts[0].ssh_arguments, None);
///
/// assert_eq!(config.hosts[1].destination, "pi@raspberry");
///
/// assert_eq!(config.hosts[2].destination, "baz");
/// ```

/// `Common` represents options that can be
/// globally specified or specified per-host.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Common {
    /// arguments passed to `ssh` when connecting to this remote
    ssh_arguments: Option<Vec<String>>,

    /// the path where `topgrade` is installed on the remote machine
    ///
    /// if left unspecified, we assume
    /// it exists in the remote system's `PATH`
    topgrade_path: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Remote {
    hosts: Vec<Host>,

    #[serde(flatten)]
    common: Option<Common>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Host {
    #[serde(flatten)]
    common: Option<Common>,

    /// the hostname of the remote to connect to
    ///
    /// [`ssh`][ssh-manpage] connects and logs into a specified *destination*,
    /// which may be specified as either `[user@]hostname`
    /// or a URI of the form `ssh://[user@]hostname[:port]`
    ///
    /// [ssh-manpage]: <https://www.man7.org/linux/man-pages/man1/ssh.1.html>
    destination: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Deprecated {
    remote_topgrades: Option<Vec<String>>,
    remote_topgrade_path: Option<String>,
    ssh_arguments: Option<String>,
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use toml::Value;

    use super::*;

    #[derive(Debug, Deserialize)]
    struct MockConfigFile {
        remote: Option<Remote>,

        #[serde(flatten)]
        deprecated: Option<super::Deprecated>,

        #[serde(flatten)]
        remainder: HashMap<String, Value>,
    }

    fn deprecated_config_10_1_2() -> &'static str {
        r#"
          # Don't ask for confirmations
          assume_yes = true

          # Disable specific steps - same options as the command line flag
          disable = ["system", "emacs"]

          # Ignore failures for these steps
          ignore_failures = ["powershell"]

          # Run specific steps - same options as the command line flag
          only = ["system", "emacs"]

          # Do not ask to retry failed steps (default: false)
          no_retry = true

          # Run inside tmux
          run_in_tmux = true

          # List of remote machines with Topgrade installed on them
          remote_topgrades = ["toothless", "pi", "parnas"]

          # Arguments to pass SSH when upgrading remote systems
          ssh_arguments = "-o ConnectTimeout=2"

          # Path to Topgrade executable on remote machines
          remote_topgrade_path = ".cargo/bin/topgrade"

          # Arguments to pass tmux when pulling Repositories
          tmux_arguments = "-S /var/tmux.sock"

          # Do not set the terminal title
          set_title = false

          # Display the time in step titles
           display_time = true

          # Cleanup temporary or old files
          cleanup = true

          # Skip sending a notification at the end of a run
          skip_notify = true

          [git]
          max_concurrency = 5
          # Additional git repositories to pull
          repos = [
              "~/src/*/",
              "~/.config/something"
          ]

          # Don't pull the predefined git repos
          pull_predefined = false

          # Arguments to pass Git when pulling Repositories
          arguments = "--rebase --autostash"

          [composer]
          self_update = true

          # Commands to run before anything
          [pre_commands]
          "Emacs Snapshot" = "rm -rf ~/.emacs.d/elpa.bak && cp -rl ~/.emacs.d/elpa ~/.emacs.d/elpa.bak"

          # Custom commands
          [commands]
          "Python Environment" = "~/dev/.env/bin/pip install -i https://pypi.python.org/simple -U --upgrade-strategy eager jupyter"

          [brew]
          greedy_cask = true
          autoremove = true

          [linux]
          # Arch Package Manager to use. Allowed values: autodetect, trizen, aura, paru, yay, pikaur, pacman, pamac.
          arch_package_manager = "pacman"
          # Arguments to pass yay (or paru) when updating packages
          yay_arguments = "--nodevel"
          aura_aur_arguments = "-kx"
          aura_pacman_arguments = ""
          show_arch_news = true
          trizen_arguments = "--devel"
          pikaur_arguments = ""
          pamac_arguments = "--no-devel"
          enable_tlmgr = true
          emerge_sync_flags = "-q"
          emerge_update_flags = "-uDNa --with-bdeps=y world"
          redhat_distro_sync = false
          rpm_ostree = false

          [windows]
          # Manually select Windows updates
          accept_all_updates = false
          open_remotes_in_new_terminal = true

          # Causes Topgrade to rename itself during the run to allow package managers
          # to upgrade it. Use this only if you installed Topgrade by using a package
          # manager such as Scoop or Cargo
          self_rename = true

          [npm]
          # Use sudo if the NPM directory isn't owned by the current user
          use_sudo = true

          [firmware]
          # Offer to update firmware; if false just check for and display available updates
          upgrade = true

          [flatpak]
          # Use sudo for updating the system-wide installation
          use_sudo = true

          [distrobox]
          use_root = false
          containers = ["archlinux-latest"]
      "#
    }

    #[test]
    fn parse_10_1_2_config() {
        let config: MockConfigFile = toml::from_str(deprecated_config_10_1_2()).unwrap();

        assert_eq!(config.remote, None);
        assert_eq!(
            config.deprecated,
            Some(Deprecated {
                remote_topgrades: Some(vec!["toothless".to_string(), "pi".to_string(), "parnas".to_string()]),
                remote_topgrade_path: Some(".cargo/bin/topgrade".to_string()),
                ssh_arguments: Some("-o ConnectTimeout=2".to_string()),
            })
        );
    }

    #[test]
    fn parse_simple_table() {
        let config: HashMap<String, Value> = toml::from_str(
            r#"
[remote]
ssh_arguments = ["-o", "ConnectTimeout=2"]
topgrade_path = ".cargo/bin/topgrade"

[[remote.hosts]]
destination = "foobar"
"#,
        )
        .unwrap();

        println!("{:#?}", config);

        // assert_eq!(
        //     config.common,
        //     Some(Common {
        //         ssh_arguments: Some(vec!["-o".to_string(), "ConnectTimeout=2".to_string()]),
        //         topgrade_path: Some(".cargo/bin/topgrade".to_string())
        //     })
        // );
        // assert_eq!(
        //     config.hosts,
        //     vec![Host {
        //         destination: "foobar".to_string(),
        //         common: None,
        //     }]
        // );
    }
}
