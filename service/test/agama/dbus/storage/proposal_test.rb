# frozen_string_literal: true

# Copyright (c) [2022-2024] SUSE LLC
#
# All Rights Reserved.
#
# This program is free software; you can redistribute it and/or modify it
# under the terms of version 2 of the GNU General Public License as published
# by the Free Software Foundation.
#
# This program is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
# more details.
#
# You should have received a copy of the GNU General Public License along
# with this program; if not, contact SUSE LLC.
#
# To contact SUSE LLC about this file by physical or electronic mail, you may
# find current contact information at www.suse.com.

require_relative "../../../test_helper"
require "agama/dbus/storage/proposal"
require "agama/storage/device_settings"
require "agama/storage/proposal"
require "agama/storage/proposal_settings"
require "agama/storage/volume"
require "y2storage"

describe Agama::DBus::Storage::Proposal do
  subject { described_class.new(backend, logger) }

  let(:backend) do
    instance_double(Agama::Storage::Proposal, settings: settings)
  end

  let(:logger) { Logger.new($stdout, level: :warn) }

  let(:settings) { nil }

  describe "#settings" do
    context "if a proposal has not been calculated yet" do
      let(:settings) { nil }

      it "returns an empty hash" do
        expect(subject.settings).to eq({})
      end
    end

    context "if a proposal has been calculated" do
      let(:settings) do
        Agama::Storage::ProposalSettings.new.tap do |settings|
          settings.device = Agama::Storage::DeviceSettings::Disk.new("/dev/vda")
          settings.boot.device = "/dev/vdb"
          settings.encryption.password = "n0ts3cr3t"
          settings.encryption.method = luks2
          settings.encryption.pbkd_function = argon2id
          settings.space.policy = :custom
          settings.space.actions = {
            "/dev/vda1" => :force_delete,
            "/dev/vda2" => :resize
          }
          settings.volumes = [
            Agama::Storage::Volume.new("/test1"),
            Agama::Storage::Volume.new("/test2")
          ]
        end
      end

      let(:luks2) { Y2Storage::EncryptionMethod::LUKS2 }

      let(:argon2id) { Y2Storage::PbkdFunction::ARGON2ID }

      it "returns the proposal settings in D-Bus format" do
        expect(subject.settings).to include(
          "Target"                 => "disk",
          "TargetDevice"           => "/dev/vda",
          "ConfigureBoot"          => true,
          "BootDevice"             => "/dev/vdb",
          "EncryptionPassword"     => "n0ts3cr3t",
          "EncryptionMethod"       => luks2.id.to_s,
          "EncryptionPBKDFunction" => argon2id.value,
          "SpacePolicy"            => "custom",
          "SpaceActions"           => [
            { "Device" => "/dev/vda1", "Action" => "force_delete" },
            { "Device" => "/dev/vda2", "Action" => "resize" }
          ],
          "Volumes"                => [
            include("MountPath" => "/test1"),
            include("MountPath" => "/test2")
          ]
        )
      end
    end
  end

  describe "#actions" do
    before do
      allow(backend).to receive(:actions).and_return(actions)
    end

    context "if there are no actions" do
      let(:actions) { [] }

      it "returns an empty list" do
        expect(subject.actions).to eq([])
      end
    end

    context "if there are actions" do
      let(:actions) { [action1, action2] }

      let(:action1) do
        instance_double(Y2Storage::CompoundAction,
          sentence: "test1", target_device: device1, device_is?: false, delete?: false)
      end

      let(:action2) do
        instance_double(Y2Storage::CompoundAction,
          sentence: "test2", target_device: device2, device_is?: true, delete?: true)
      end

      let(:device1) { instance_double(Y2Storage::Device, sid: 1) }

      let(:device2) { instance_double(Y2Storage::Device, sid: 2) }

      it "returns a list with a hash for each action" do
        expect(subject.actions.size).to eq(2)
        expect(subject.actions).to all(be_a(Hash))

        action1, action2 = subject.actions

        expect(action1).to eq({
          "Device" => 1,
          "Text"   => "test1",
          "Subvol" => false,
          "Delete" => false
        })

        expect(action2).to eq({
          "Device" => 2,
          "Text"   => "test2",
          "Subvol" => true,
          "Delete" => true
        })
      end
    end
  end
end
