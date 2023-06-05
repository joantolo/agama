//! D-Bus interface proxies for: `org.opensuse.Agama*.**.*`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.`.
use zbus::dbus_proxy;

/// Progress1Proxy can be used also with Software and Storage object.
///
/// TODO: example
#[dbus_proxy(
    interface = "org.opensuse.Agama1.Progress",
    default_service = "org.opensuse.Agama1",
    default_path = "/org/opensuse/Agama1/Manager"
)]
trait Progress {
    /// CurrentStep property
    #[dbus_proxy(property)]
    fn current_step(&self) -> zbus::Result<(u32, String)>;

    /// Finished property
    #[dbus_proxy(property)]
    fn finished(&self) -> zbus::Result<bool>;

    /// TotalSteps property
    #[dbus_proxy(property)]
    fn total_steps(&self) -> zbus::Result<u32>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama1.ServiceStatus",
    default_service = "org.opensuse.Agama1",
    default_path = "/org/opensuse/Agama1/Manager"
)]
trait ServiceStatus {
    /// All property
    #[dbus_proxy(property)]
    fn all(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Current property
    #[dbus_proxy(property)]
    fn current(&self) -> zbus::Result<u32>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama1.Manager",
    default_service = "org.opensuse.Agama1",
    default_path = "/org/opensuse/Agama1/Manager"
)]
trait Manager {
    /// CanInstall method
    fn can_install(&self) -> zbus::Result<bool>;

    /// CollectLogs method
    fn collect_logs(&self, user: &str) -> zbus::Result<String>;

    /// Commit method
    fn commit(&self) -> zbus::Result<()>;

    /// Probe method
    fn probe(&self) -> zbus::Result<()>;

    /// BusyServices property
    #[dbus_proxy(property)]
    fn busy_services(&self) -> zbus::Result<Vec<String>>;

    /// CurrentInstallationPhase property
    #[dbus_proxy(property)]
    fn current_installation_phase(&self) -> zbus::Result<u32>;

    /// InstallationPhases property
    #[dbus_proxy(property)]
    fn installation_phases(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[dbus_proxy(interface = "org.opensuse.Agama.Locale1", assume_defaults = true)]
trait Locale1 {
    /// Commit method
    fn commit(&self) -> zbus::Result<()>;

    /// LabelsForLocales method
    fn labels_for_locales(&self) -> zbus::Result<Vec<(String, String)>>;

    /// ListTimezones method
    fn list_timezones(&self, locale: &str) -> zbus::Result<Vec<(String, String)>>;

    /// ListVConsoleKeyboards method
    #[dbus_proxy(name = "ListVConsoleKeyboards")]
    fn list_vconsole_keyboards(&self) -> zbus::Result<Vec<String>>;

    /// Locales property
    #[dbus_proxy(property)]
    fn locales(&self) -> zbus::Result<Vec<String>>;
    fn set_locales(&self, value: &[&str]) -> zbus::Result<()>;

    /// SupportedLocales property
    #[dbus_proxy(property)]
    fn supported_locales(&self) -> zbus::Result<Vec<String>>;
    fn set_supported_locales(&self, value: &[&str]) -> zbus::Result<()>;

    /// Timezone property
    #[dbus_proxy(property)]
    fn timezone(&self) -> zbus::Result<String>;
    fn set_timezone(&self, value: &str) -> zbus::Result<()>;

    /// VConsoleKeyboard property
    #[dbus_proxy(property, name = "VConsoleKeyboard")]
    fn vconsole_keyboard(&self) -> zbus::Result<String>;
    fn set_vconsole_keyboard(&self, value: &str) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Questions1",
    default_service = "org.opensuse.Agama.Questions1",
    default_path = "/org/opensuse/Agama/Questions1"
)]
trait Questions1 {
    /// Delete method
    fn delete(&self, question: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// New method
    #[dbus_proxy(name = "New")]
    fn create(
        &self,
        text: &str,
        options: &[&str],
        default_option: &[&str],
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// NewLuksActivation method
    fn new_luks_activation(
        &self,
        device: &str,
        label: &str,
        size: &str,
        attempt: u8,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Software1",
    default_service = "org.opensuse.Agama.Software1",
    default_path = "/org/opensuse/Agama/Software1"
)]
trait Software1 {
    /// Finish method
    fn finish(&self) -> zbus::Result<()>;

    /// Install method
    fn install(&self) -> zbus::Result<()>;

    /// IsPackageInstalled method
    fn is_package_installed(&self, name: &str) -> zbus::Result<bool>;

    /// Probe method
    fn probe(&self) -> zbus::Result<()>;

    /// Propose method
    fn propose(&self) -> zbus::Result<()>;

    /// ProvisionSelected method
    fn provision_selected(&self, provision: &str) -> zbus::Result<bool>;

    /// ProvisionsSelected method
    fn provisions_selected(&self, provisions: &[&str]) -> zbus::Result<Vec<bool>>;

    /// SelectProduct method
    fn select_product(&self, product_id: &str) -> zbus::Result<()>;

    /// UsedDiskSpace method
    fn used_disk_space(&self) -> zbus::Result<String>;

    /// AvailableBaseProducts property
    #[dbus_proxy(property)]
    fn available_base_products(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// SelectedBaseProduct property
    #[dbus_proxy(property)]
    fn selected_base_product(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Software1.Proposal",
    default_service = "org.opensuse.Agama.Software1",
    default_path = "/org/opensuse/Agama/Software1/Proposal"
)]
trait SoftwareProposal {
    /// AddResolvables method
    fn add_resolvables(
        &self,
        id: &str,
        r#type: u8,
        resolvables: &[&str],
        optional: bool,
    ) -> zbus::Result<()>;

    /// GetResolvables method
    fn get_resolvables(&self, id: &str, r#type: u8, optional: bool) -> zbus::Result<Vec<String>>;

    /// RemoveResolvables method
    fn remove_resolvables(
        &self,
        id: &str,
        r#type: u8,
        resolvables: &[&str],
        optional: bool,
    ) -> zbus::Result<()>;

    /// SetResolvables method
    fn set_resolvables(
        &self,
        id: &str,
        r#type: u8,
        resolvables: &[&str],
        optional: bool,
    ) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama1.Validation",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1"
)]
trait Validation {
    /// Errors property
    #[dbus_proxy(property)]
    fn errors(&self) -> zbus::Result<Vec<String>>;

    /// Valid property
    #[dbus_proxy(property)]
    fn valid(&self) -> zbus::Result<bool>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1"
)]
trait Storage1 {
    /// Finish method
    fn finish(&self) -> zbus::Result<()>;

    /// Install method
    fn install(&self) -> zbus::Result<()>;

    /// Probe method
    fn probe(&self) -> zbus::Result<()>;

    /// DeprecatedSystem property
    #[dbus_proxy(property)]
    fn deprecated_system(&self) -> zbus::Result<bool>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1.Proposal.Calculator",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1"
)]
trait Calculator {
    /// Calculate method
    fn calculate(
        &self,
        settings: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<u32>;

    /// AvailableDevices property
    #[dbus_proxy(property)]
    fn available_devices(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// Result property
    #[dbus_proxy(property)]
    fn result(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// VolumeTemplates property
    #[dbus_proxy(property)]
    fn volume_templates(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1.Proposal",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1/Proposal"
)]
trait StorageProposal {
    /// Actions property
    #[dbus_proxy(property)]
    fn actions(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// CandidateDevices property
    #[dbus_proxy(property)]
    fn candidate_devices(&self) -> zbus::Result<Vec<String>>;

    /// EncryptionPassword property
    #[dbus_proxy(property)]
    fn encryption_password(&self) -> zbus::Result<String>;

    /// LVM property
    #[dbus_proxy(property, name = "LVM")]
    fn lvm(&self) -> zbus::Result<bool>;

    /// Volumes property
    #[dbus_proxy(property)]
    fn volumes(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Users1",
    default_service = "org.opensuse.Agama.Users1",
    default_path = "/org/opensuse/Agama/Users1"
)]
trait Users1 {
    /// RemoveFirstUser method
    fn remove_first_user(&self) -> zbus::Result<u32>;

    /// RemoveRootPassword method
    fn remove_root_password(&self) -> zbus::Result<u32>;

    /// SetFirstUser method
    fn set_first_user(
        &self,
        full_name: &str,
        user_name: &str,
        password: &str,
        auto_login: bool,
        data: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(bool, Vec<String>)>;

    /// SetRootPassword method
    fn set_root_password(&self, value: &str, encrypted: bool) -> zbus::Result<u32>;

    /// SetRootSSHKey method
    #[dbus_proxy(name = "SetRootSSHKey")]
    fn set_root_sshkey(&self, value: &str) -> zbus::Result<u32>;

    /// Write method
    fn write(&self) -> zbus::Result<u32>;

    /// FirstUser property
    #[dbus_proxy(property)]
    fn first_user(
        &self,
    ) -> zbus::Result<(
        String,
        String,
        String,
        bool,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;

    /// RootPasswordSet property
    #[dbus_proxy(property)]
    fn root_password_set(&self) -> zbus::Result<bool>;

    /// RootSSHKey property
    #[dbus_proxy(property, name = "RootSSHKey")]
    fn root_sshkey(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Network1.Connections",
    default_service = "org.opensuse.Agama.Network1",
    default_path = "/org/opensuse/Agama/Network1/connections"
)]
trait Connections {
    /// AddConnection method
    fn add_connection(&self, name: &str, ty: u8) -> zbus::Result<()>;

    /// Apply method
    fn apply(&self) -> zbus::Result<()>;

    /// GetConnections method
    fn get_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// RemoveConnection method
    fn remove_connection(&self, uuid: &str) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Network1.Connection.Wireless",
    default_service = "org.opensuse.Agama.Network1",
    default_path = "/org/opensuse/Agama/Network1"
)]
trait Wireless {
    /// Mode property
    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<u8>;
    fn set_mode(&self, value: u8) -> zbus::Result<()>;

    /// Password property
    #[dbus_proxy(property)]
    fn password(&self) -> zbus::Result<String>;
    fn set_password(&self, value: &str) -> zbus::Result<()>;

    /// SSID property
    #[dbus_proxy(property, name = "SSID")]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;
    fn set_ssid(&self, value: &[u8]) -> zbus::Result<()>;

    /// Security property
    #[dbus_proxy(property)]
    fn security(&self) -> zbus::Result<u8>;
    fn set_security(&self, value: u8) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Network1.Connection",
    default_service = "org.opensuse.Agama.Network1",
    default_path = "/org/opensuse/Agama/Network1"
)]
trait Connection {
    /// Id property
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    /// UUID property
    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Network1.Connection.IPv4",
    default_service = "org.opensuse.Agama.Network1",
    default_path = "/org/opensuse/Agama/Network1"
)]
trait IPv4 {
    /// Addresses property
    #[dbus_proxy(property)]
    fn addresses(&self) -> zbus::Result<Vec<(String, u32)>>;
    fn set_addresses(&self, value: &[(&str, u32)]) -> zbus::Result<()>;

    /// Gateway property
    #[dbus_proxy(property)]
    fn gateway(&self) -> zbus::Result<String>;
    fn set_gateway(&self, value: &str) -> zbus::Result<()>;

    /// Method property
    #[dbus_proxy(property)]
    fn method(&self) -> zbus::Result<u8>;
    fn set_method(&self, value: u8) -> zbus::Result<()>;

    /// Nameservers property
    #[dbus_proxy(property)]
    fn nameservers(&self) -> zbus::Result<Vec<String>>;
    fn set_nameservers(&self, value: &[&str]) -> zbus::Result<()>;
}
