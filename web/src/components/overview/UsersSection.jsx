/*
 * Copyright (c) [2023] SUSE LLC
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

import React, { useReducer, useEffect } from "react";

import { useCancellablePromise } from "~/utils";
import { useInstallerClient } from "~/context/installer";
import { Section, SectionSkeleton } from "~/components/core";
import { Label } from "@patternfly/react-core";

/**
 * Internal component for simplifying conditional rendering.
 * @component
 *
 * Borrowed from the old  Michael J. Ryan’s comment at https://github.com/facebook/jsx/issues/65#issuecomment-255484351
 * See more options at https://blog.logrocket.com/react-conditional-rendering-9-methods/
 *
 * TODO: evaluate if it should be a core component or even if worth it using an specialized library
 *
 * @param {object} props
 * @param {boolean} props.condition
 * @param {JSX.Element} [props.then=null] - the content to be rendered when the condition is true
 * @param {JSX.Element} [props.else=null] - the content to be rendered when the condition is false
 */
const If = ({ condition, then: positive = null, else: negative = null }) => {
  return condition ? positive : negative;
};

const initialState = {
  busy: true,
  errors: [],
  user: undefined,
  rootSSHKey: undefined,
  rootPasswordSet: false,
};

const reducer = (state, action) => {
  const { type: actionType, payload } = action;

  switch (actionType) {
    case "UPDATE_STATUS": {
      return { ...initialState, ...payload };
    }

    default: {
      return state;
    }
  }
};

export default function UsersSection({ showErrors }) {
  const { users: client } = useInstallerClient();
  const { cancellablePromise } = useCancellablePromise();
  const [state, dispatch] = useReducer(reducer, initialState);
  const { user, rootPasswordSet, rootSSHKey } = state;

  const updateStatus = ({ ...payload }) => {
    dispatch({ type: "UPDATE_STATUS", payload });
  };

  useEffect(() => {
    const loadData = async () => {
      const user = await cancellablePromise(client.getUser());
      const rootPasswordSet = await cancellablePromise(client.isRootPasswordSet());
      const rootSSHKey = await cancellablePromise(client.getRootSSHKey());
      const errors = await cancellablePromise(client.getValidationErrors());

      updateStatus({ user, rootPasswordSet, rootSSHKey, errors, busy: false });
    };

    loadData();

    return client.onValidationChange(
      (errors) => updateStatus({ errors })
    );
  }, [client, cancellablePromise]);

  const errors = showErrors ? state.errors : [];

  const UserSummary = () => {
    return (
      <div>
        <If
          condition={user?.userName !== ""}
          then={<>User <Label isCompact>{state.user.userName}</Label> will be created</>}
          else={<>No user defined yet</>}
        />
      </div>
    );
  };

  const RootAuthSummary = () => {
    const both = rootPasswordSet && rootSSHKey !== "";
    const none = !rootPasswordSet && rootSSHKey === "";
    const onlyPassword = rootPasswordSet && rootSSHKey === "";
    const onlySSHKey = !rootPasswordSet && rootSSHKey !== "";

    return (
      <div>
        <If condition={both} then={<>Root authentication set for using both, password and public SSH Key</>} />
        <If condition={none} then={<>None authentication method defined for root user</>} />
        <If condition={onlyPassword} then={<>Root authentication set for using password</>} />
        <If condition={onlySSHKey} then={<>Root authentication set for using public SSH Key</>} />
      </div>
    );
  };

  const Summary = () => (
    <>
      <UserSummary />
      <RootAuthSummary />
    </>
  );

  return (
    <Section
      key="users-section"
      title="Users"
      icon="manage_accounts"
      loading={state.busy}
      errors={errors}
      path="/users"
    >
      { state.busy ? <SectionSkeleton /> : <Summary /> }
    </Section>
  );
}
