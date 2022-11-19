use std::str::FromStr;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Common {
    ssh_arguments: Option<Vec<String>>,
    topgrade_path: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Deprecated {
    remote_topgrades: Vec<String>,
    ssh_arguments: Option<String>,
    remote_topgrade_path: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Remote {
    hosts: Vec<Host>,

    #[serde(flatten)]
    common: Common,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Host {
    destination: String,

    #[serde(flatten)]
    common: Common,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct MockConfig {
        remote: Option<Remote>,

        #[serde(flatten)]
        deprecated: Option<Deprecated>,
    }

    #[test]
    fn test_deprecated_config() {
        let config: MockConfig = toml::from_str(
            r#"
            remote_topgrades = ["toothless", "pi", "parnas"]
            ssh_arguments = "-o ConnectTimeout=2"
            remote_topgrade_path = ".cargo/bin/topgrade"
            "#,
        )
        .unwrap();

        assert_eq!(
            config.deprecated.unwrap(),
            Deprecated {
                remote_topgrades: vec![String::from("toothless"), String::from("pi"), String::from("parnas")],
                ssh_arguments: Some(String::from("-o ConnectTimeout=2")),
                remote_topgrade_path: Some(String::from(".cargo/bin/topgrade")),
            }
        );
        assert_eq!(config.remote, None);
    }

    #[test]
    fn test_new_config() {
        let config: MockConfig = toml::from_str(
            r#"
            [[remote.hosts]]
            destination = "toothless"

            [[remote.hosts]]
            destination = "pi"

            [[remote.hosts]]
            destination = "parnas"
            "#,
        )
        .unwrap();

        assert_eq!(
            config.remote.unwrap(),
            Remote {
                hosts: vec![
                    Host {
                        destination: String::from("toothless"),
                        common: Common::default(),
                    },
                    Host {
                        destination: String::from("pi"),
                        common: Common::default(),
                    },
                    Host {
                        destination: String::from("parnas"),
                        common: Common::default(),
                    }
                ],
                common: Common::default(),
            }
        );
        assert_eq!(config.deprecated, None);
    }

    #[test]
    fn test_global_config() {
        let config: MockConfig = toml::from_str(
            r#"
            [remote]
            ssh_arguments = ["-o", "ConnectTimeout=2"]
            topgrade_path = ".cargo/bin/topgrade"
            hosts = [ { destination = "foo" } ]
            "#,
        )
        .unwrap();

        assert_eq!(
            config.remote.unwrap().common,
            Common {
                ssh_arguments: Some(vec![String::from("-o"), String::from("ConnectTimeout=2")]),
                topgrade_path: Some(String::from(".cargo/bin/topgrade"))
            }
        );
    }

    #[test]
    fn test_deprecated_and_new() {
        let config: MockConfig = toml::from_str(
            r#"
            remote_topgrades = ["toothless", "pi"]
            remote_topgrade_path = ".cargo/bin/topgrade"

            [remote]
            ssh_arguments = ["-o", "ConnectTimeout=2"]

            [[remote.hosts]]
            destination = "toothless"
            topgrade_path = ".local/bin/topgrade"
            "#,
        )
        .unwrap();

        assert_eq!(
            config,
            MockConfig {
                deprecated: Some(Deprecated {
                    remote_topgrades: vec![String::from("toothless"), String::from("pi")],
                    remote_topgrade_path: Some(String::from(".cargo/bin/topgrade")),
                    ssh_arguments: None,
                }),
                remote: Some(Remote {
                    hosts: vec![Host {
                        destination: String::from("toothless"),
                        common: Common {
                            ssh_arguments: None,
                            topgrade_path: Some(String::from(".local/bin/topgrade")),
                        }
                    }],
                    common: Common {
                        ssh_arguments: Some(vec![String::from("-o"), String::from("ConnectTimeout=2")]),
                        topgrade_path: None
                    },
                })
            }
        );
    }
}
