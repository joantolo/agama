//! D-Bus interface proxies for interfaces implemented by objects in the storage service.
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
use zbus::dbus_proxy;

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
trait ProposalCalculator {
    /// Calculate method
    fn calculate(
        &self,
        settings: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<u32>;

    /// DefaultVolume method
    fn default_volume(
        &self,
        mount_path: &str,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// AvailableDevices property
    #[dbus_proxy(property)]
    fn available_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// EncryptionMethods property
    #[dbus_proxy(property)]
    fn encryption_methods(&self) -> zbus::Result<Vec<String>>;

    /// ProductMountPoints property
    #[dbus_proxy(property)]
    fn product_mount_points(&self) -> zbus::Result<Vec<String>>;

    /// Result property
    #[dbus_proxy(property)]
    fn result(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1.Proposal",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1/Proposal"
)]
trait Proposal {
    /// Actions property
    #[dbus_proxy(property)]
    fn actions(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// BootDevice property
    #[dbus_proxy(property)]
    fn boot_device(&self) -> zbus::Result<String>;

    /// EncryptionMethod property
    #[dbus_proxy(property)]
    fn encryption_method(&self) -> zbus::Result<String>;

    /// EncryptionPBKDFunction property
    #[dbus_proxy(property, name = "EncryptionPBKDFunction")]
    fn encryption_pbkdfunction(&self) -> zbus::Result<String>;

    /// EncryptionPassword property
    #[dbus_proxy(property)]
    fn encryption_password(&self) -> zbus::Result<String>;

    /// LVM property
    #[dbus_proxy(property, name = "LVM")]
    fn lvm(&self) -> zbus::Result<bool>;

    /// SpacePolicy property
    #[dbus_proxy(property)]
    fn space_policy(&self) -> zbus::Result<String>;

    /// SystemVGDevices property
    #[dbus_proxy(property, name = "SystemVGDevices")]
    fn system_vg_devices(&self) -> zbus::Result<Vec<Vec<String>>>;

    /// Volumes property
    #[dbus_proxy(property)]
    fn volumes(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1.Block",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1"
)]
trait Block {
    /// Active property
    #[dbus_proxy(property)]
    fn active(&self) -> zbus::Result<bool>;

    /// Encrypted property
    #[dbus_proxy(property)]
    fn encrypted(&self) -> zbus::Result<bool>;

    /// RecoverableSize property
    #[dbus_proxy(property)]
    fn recoverable_size(&self) -> zbus::Result<u64>;

    /// Size property
    #[dbus_proxy(property)]
    fn size(&self) -> zbus::Result<u64>;

    /// Start property
    #[dbus_proxy(property)]
    fn start(&self) -> zbus::Result<u64>;

    /// Systems property
    #[dbus_proxy(property)]
    fn systems(&self) -> zbus::Result<Vec<String>>;

    /// UdevIds property
    #[dbus_proxy(property)]
    fn udev_ids(&self) -> zbus::Result<Vec<String>>;

    /// UdevPaths property
    #[dbus_proxy(property)]
    fn udev_paths(&self) -> zbus::Result<Vec<String>>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1.Drive",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1"
)]
trait Drive {
    /// Bus property
    #[dbus_proxy(property)]
    fn bus(&self) -> zbus::Result<String>;

    /// BusId property
    #[dbus_proxy(property)]
    fn bus_id(&self) -> zbus::Result<String>;

    /// Driver property
    #[dbus_proxy(property)]
    fn driver(&self) -> zbus::Result<Vec<String>>;

    /// Info property
    #[dbus_proxy(property)]
    fn info(&self) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Model property
    #[dbus_proxy(property)]
    fn model(&self) -> zbus::Result<String>;

    /// Transport property
    #[dbus_proxy(property)]
    fn transport(&self) -> zbus::Result<String>;

    /// Type property
    #[dbus_proxy(property)]
    fn type_(&self) -> zbus::Result<String>;

    /// Vendor property
    #[dbus_proxy(property)]
    fn vendor(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1.Multipath",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1"
)]
trait Multipath {
    /// Wires property
    #[dbus_proxy(property)]
    fn wires(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1.PartitionTable",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1"
)]
trait PartitionTable {
    /// Partitions property
    #[dbus_proxy(property)]
    fn partitions(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Type property
    #[dbus_proxy(property)]
    fn type_(&self) -> zbus::Result<String>;

    /// UnusedSlots property
    #[dbus_proxy(property)]
    fn unused_slots(&self) -> zbus::Result<Vec<(u64, u64)>>;
}

#[dbus_proxy(
    interface = "org.opensuse.Agama.Storage1.Device",
    default_service = "org.opensuse.Agama.Storage1",
    default_path = "/org/opensuse/Agama/Storage1"
)]
trait Device {
    /// Description property
    #[dbus_proxy(property)]
    fn description(&self) -> zbus::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    /// SID property
    #[dbus_proxy(property, name = "SID")]
    fn sid(&self) -> zbus::Result<u32>;
}
