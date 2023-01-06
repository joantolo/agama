/*
 * Copyright (c) [2022] SUSE LLC
 *
 * All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2 of the GNU General Public License as published
 * by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, contact SUSE LLC.
 *
 * To contact SUSE LLC about this file by physical or electronic mail, you may
 * find current contact information at www.suse.com.
 */

// @ts-check

import DBusClient from "./dbus";
import { WithStatus, WithProgress } from "./mixins";
import cockpit from "../lib/cockpit";

const MANAGER_SERVICE = "org.opensuse.DInstaller";
const MANAGER_IFACE = "org.opensuse.DInstaller.Manager1";
const MANAGER_PATH = "/org/opensuse/DInstaller/Manager1";

/**
 * Manager base client
 *
 * @ignore
 */
class ManagerBaseClient {
  /**
   * @param {string|undefined} address - D-Bus address; if it is undefined, it uses the system bus.
   */
  constructor(address = undefined) {
    this.client = new DBusClient(MANAGER_SERVICE, address);
  }

  /**
   * Run probing process
   *
   * The progress of the probing process can be tracked through installer signals.
   *
   * @return {Promise<void>}
   */
  async startProbing() {
    const proxy = await this.client.proxy(MANAGER_IFACE);
    return proxy.Probe();
  }

  /**
   * Start the installation process
   *
   * The progress of the installation process can be tracked through installer signals.
   *
   * @return {Promise}
   */
  async startInstallation() {
    const proxy = await this.client.proxy(MANAGER_IFACE);
    return proxy.Commit();
  }

  /**
   * Checks whether it is possible to start the installation
   *
   * It might happen that there are some validation errors. In that case,
   * it is not possible to proceed with the installation.
   *
   * @return {Promise<boolean>}
   */
  async canInstall() {
    const proxy = await this.client.proxy(MANAGER_IFACE);
    return proxy.CanInstall();
  }

  /**
   * Returns a path where logs are stored
   *
   *
   * @return {Promise<string>}
   */
  async provideLogs() {
    const proxy = await this.client.proxy(MANAGER_IFACE);
    return proxy.ProvideLogs("root"); // lets hardcode root here
  }

  /**
   * Returns a content of log file
   *
   * @return {Promise<Uint8Array>}
   */
  async logsContent() {
    const path = await this.provideLogs();
    const file = cockpit.file(path, { binary: true });
    return file.read();
  }

  /**
   * Return the installer status
   *
   * @return {Promise<number>}
   */
  async getPhase() {
    const proxy = await this.client.proxy(MANAGER_IFACE);
    return proxy.CurrentInstallationPhase;
  }

  /**
   * Register a callback to run when the "CurrentInstallationPhase" changes
   *
   * @param {function} handler - callback function
   * @return {import ("./dbus").RemoveFn} function to disable the callback
   */
  onPhaseChange(handler) {
    return this.client.onObjectChanged(MANAGER_PATH, MANAGER_IFACE, (changes) => {
      if ("CurrentInstallationPhase" in changes) {
        handler(changes.CurrentInstallationPhase.v);
      }
    });
  }

  /**
   * Returns whether calling the system reboot succeeded or not.
   *
   * @return {Promise<boolean>}
   */
  rebootSystem() {
    return cockpit.spawn(["/usr/sbin/shutdown", "-r", "now"]);
  }
}

/**
  * Client to interact with the D-Installer manager service
  */
class ManagerClient extends WithProgress(WithStatus(ManagerBaseClient, MANAGER_PATH), MANAGER_PATH) {}

export { ManagerClient };
