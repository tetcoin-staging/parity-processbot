// TODO: Move bitflags to use `bitflags` crate.
#![allow(non_upper_case_globals)]
use crate::db::DBEntry;
use crate::Result;
use parking_lot::RwLock;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::SystemTime;

/// Bitflag indicating no action has been taken
pub const NoAction: u32 = 0b00000000;

/// Bitflag indicating an issue has been incorrectly assigned
/// for at least 24h and an appropriate action has been taken
pub const PullRequestCoreDevAuthorIssueNotAssigned24h: u32 = 0b00000010;

/// Bitflag indicating an issue has been incorrectly assigned
/// for at least 72h and an appropriate action has been taken
pub const PullRequestCoreDevAuthorIssueNotAssigned72h: u32 = 0b00000100;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IssueProjectState {
	Confirmed,
	Unconfirmed,
	Denied,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueProject {
	pub state: IssueProjectState,
	pub actor_login: String,
	pub project_column_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalState {
	pub key: Vec<u8>,
	actions_taken: u32,
	status_failure_ping: Option<SystemTime>,
	issue_not_assigned_ping: Option<SystemTime>,
	issue_no_project_ping: Option<SystemTime>,
	issue_no_project_npings: u64,
	issue_confirm_project_ping: Option<SystemTime>,
	issue_project: Option<IssueProject>,
	last_confirmed_issue_project: Option<IssueProject>,
}

impl Default for LocalState {
	fn default() -> LocalState {
		LocalState {
			key: vec![],
			actions_taken: NoAction,
			issue_not_assigned_ping: None,
			issue_no_project_ping: None,
			issue_no_project_npings: 0,
			status_failure_ping: None,
			issue_confirm_project_ping: None,
			issue_project: None,
			last_confirmed_issue_project: None,
		}
	}
}

impl LocalState {
	pub fn actions_taken(&self) -> u32 {
		self.actions_taken
	}

	pub fn update_actions_taken(
		&mut self,
		x: u32,
		db: &Arc<RwLock<DB>>,
	) -> Result<()> {
		self.actions_taken = x;
		self.update(db, &self.key)
	}

	pub fn status_failure_ping(&self) -> Option<&SystemTime> {
		self.status_failure_ping.as_ref()
	}

	pub fn update_status_failure_ping(
		&mut self,
		x: Option<SystemTime>,
		db: &Arc<RwLock<DB>>,
	) -> Result<()> {
		self.status_failure_ping = x;
		self.update(db, &self.key)
	}

	pub fn issue_not_assigned_ping(&self) -> Option<&SystemTime> {
		self.issue_not_assigned_ping.as_ref()
	}

	pub fn update_issue_not_assigned_ping(
		&mut self,
		x: Option<SystemTime>,
		db: &Arc<RwLock<DB>>,
	) -> Result<()> {
		self.issue_not_assigned_ping = x;
		self.update(db, &self.key)
	}

	pub fn issue_no_project_ping(&self) -> Option<&SystemTime> {
		self.issue_no_project_ping.as_ref()
	}

	pub fn update_issue_no_project_ping(
		&mut self,
		x: Option<SystemTime>,
		db: &Arc<RwLock<DB>>,
	) -> Result<()> {
		self.issue_no_project_ping = x;
		self.update(db, &self.key)
	}

	pub fn issue_no_project_npings(&self) -> u64 {
		self.issue_no_project_npings
	}

	pub fn update_issue_no_project_npings(
		&mut self,
		x: u64,
		db: &Arc<RwLock<DB>>,
	) -> Result<()> {
		self.issue_no_project_npings = x;
		self.update(db, &self.key)
	}

	pub fn issue_confirm_project_ping(&self) -> Option<&SystemTime> {
		self.issue_confirm_project_ping.as_ref()
	}

	pub fn update_issue_confirm_project_ping(
		&mut self,
		x: Option<SystemTime>,
		db: &Arc<RwLock<DB>>,
	) -> Result<()> {
		self.issue_confirm_project_ping = x;
		self.update(db, &self.key)
	}

	pub fn issue_project(&self) -> Option<&IssueProject> {
		self.issue_project.as_ref()
	}

	pub fn update_issue_project(
		&mut self,
		x: Option<IssueProject>,
		db: &Arc<RwLock<DB>>,
	) -> Result<()> {
		self.issue_project = x;
		self.update(db, &self.key)
	}

	pub fn last_confirmed_issue_project(&self) -> Option<&IssueProject> {
		self.last_confirmed_issue_project.as_ref()
	}

	pub fn update_last_confirmed_issue_project(
		&mut self,
		x: Option<IssueProject>,
		db: &Arc<RwLock<DB>>,
	) -> Result<()> {
		self.last_confirmed_issue_project = x;
		self.update(db, &self.key)
	}
}

impl DBEntry for LocalState {
	fn with_key(self, k: Vec<u8>) -> LocalState {
		let mut s = self;
		s.key = k;
		s
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bitflags() {
		assert_eq!(
			PullRequestCoreDevAuthorIssueNotAssigned24h
				& PullRequestCoreDevAuthorIssueNotAssigned72h,
			NoAction
		);
		assert_eq!(
			PullRequestCoreDevAuthorIssueNotAssigned24h
				| PullRequestCoreDevAuthorIssueNotAssigned72h,
			0b0000_0110
		);
		assert_eq!(
			PullRequestCoreDevAuthorIssueNotAssigned24h & NoAction,
			NoAction
		);
		assert_eq!(
			PullRequestCoreDevAuthorIssueNotAssigned24h | NoAction,
			PullRequestCoreDevAuthorIssueNotAssigned24h
		);
	}
}
