
extern crate shared;
//#[macro_use]
extern crate util;

use shared::*;
use shared::repo::Repo;

use util::{Tool, ToolInvocation, ToolArgs, ToolArgAccessor, ToolArg};
use util::command_queue::{CommandQueue, };

use std::borrow::Cow;
use std::error::Error;
use std::path::{PathBuf, };
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Session {
  ctx: Context,
  monitor: monitor::Monitor,
  log_files: LogFiles,

  upstream_repo: Repo,
  rust_repo: Repo,

  /// these aren't checked out:
  merge_branches: Vec<Repo>,
}

impl Default for Session {
  fn default() -> Session {
    Session {
      ctx: Default::default(),
      monitor: Default::default(),
      log_files: Default::default(),

      upstream_repo: Repo::new_git("upstream-rust", UPSTREAM_RUST_URL, RUST_MAIN_BRANCH),
      rust_repo: Repo::new_git("rust", RUST_URL, RUST_MAIN_BRANCH),

      merge_branches: MERGE_BRANCHES.iter()
        .map(|&branch| {
          Repo::new_git(branch, BRANCHES_URL,
                        branch)
        })
        .collect()
    }
  }
}

impl Builder for Session {
  fn context(&self) -> &Context { &self.ctx }
}
impl Build for Session {
  fn target_dir(&self) -> PathBuf { self.ctx.target_dir() }
  fn output_dir(&self) -> PathBuf { self.ctx.output_dir() }
}
impl Shell for Session {
  fn monitor(&self) -> &monitor::Monitor { &self.monitor }
  fn log_dir(&self) -> PathBuf { self.ctx.log_dir() }
  fn log_files(&self) -> &LogFiles { &self.log_files }
  fn nicely(&self) -> bool { self.context().nicely() }
}

impl Session {
  pub fn rust_src_path(&self) -> PathBuf {
    self.target_dir().join("src")
  }
}

impl ToolInvocation for Session {
  fn check_state(&mut self, iteration: usize, _skip_inputs_check: bool)
                 -> Result<(), Box<Error>>
  {
    match iteration {
      0 => {
        self.ctx.check_complete();

        let rust_src = self.rust_src_path();

        // run this outside the command queue:
        // we don't care if it fails, but git will refuse to checkout if
        // there is an on going merge, which can happen if a previous run
        // has a branch which fails to merge
        if rust_src.exists() {
          let mut cmd = Command::new("git");
          cmd.current_dir(&rust_src)
            .arg("merge")
            .arg("--abort");
          let _ = cmd.spawn()?.wait()?;
        }

        Ok(())
      },
      1 => {
        let dest = self.rust_src_path();
        self.rust_repo.checkout_fat(&dest)
      },
      _ => Ok(()),
    }
  }

  fn args(&self, iteration: usize) -> Option<ToolArgs<Self>> {

    struct RustAccess;
    impl ToolArgAccessor<Session, Repo> for RustAccess {
      fn access(this: &mut Session) -> &mut Repo {
        &mut this.rust_repo
      }
    }
    struct UpstreamRustAccess;
    impl ToolArgAccessor<Session, Repo> for UpstreamRustAccess {
      fn access(this: &mut Session) -> &mut Repo {
        &mut this.upstream_repo
      }
    }

    macro_rules! merge_branch_access {
      ($ty_name:ident, $idx:expr) => (
        struct $ty_name;
        impl ToolArgAccessor<Session, Repo> for $ty_name {
          fn access(this: &mut Session) -> &mut Repo {
            &mut this.merge_branches[$idx]
          }
        }
      );
    }

    merge_branch_access!(Branch0, 0);
    merge_branch_access!(Branch1, 1);
    merge_branch_access!(Branch2, 2);
    merge_branch_access!(Branch3, 3);
    merge_branch_access!(Branch4, 4);
    merge_branch_access!(Branch5, 5);
    merge_branch_access!(Branch6, 6);
    merge_branch_access!(Branch7, 7);
    merge_branch_access!(Branch8, 8);
    merge_branch_access!(Branch9, 9);
    merge_branch_access!(Branch10, 10);
    merge_branch_access!(Branch11, 11);
    merge_branch_access!(Branch12, 12);
    merge_branch_access!(Branch13, 13);
    merge_branch_access!(Branch14, 14);
    merge_branch_access!(Branch15, 15);

    const C: &'static [ToolArg<Session>] = &[];
    let mut out = Cow::Borrowed(C);

    match iteration {
      0 => { return self.ctx.args(); },
      1 => {
        self.rust_repo
          .args::<Self, RustAccess>(&mut out);
      },
      2 => {
        self.upstream_repo
          .args::<Self, UpstreamRustAccess>(&mut out);
        if let Some(b) = self.merge_branches.get(0) {
          b.args::<Self, Branch0>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(1) {
          b.args::<Self, Branch1>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(2) {
          b.args::<Self, Branch2>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(3) {
          b.args::<Self, Branch3>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(4) {
          b.args::<Self, Branch4>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(5) {
          b.args::<Self, Branch5>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(6) {
          b.args::<Self, Branch6>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(7) {
          b.args::<Self, Branch7>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(8) {
            b.args::<Self, Branch8>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(9) {
            b.args::<Self, Branch9>(&mut out);
        }
        if let Some(b) = self.merge_branches.get(10) {
            b.args::<Self, Branch10>(&mut out);
        }
          if let Some(b) = self.merge_branches.get(11) {
              b.args::<Self, Branch11>(&mut out);
          }
          if let Some(b) = self.merge_branches.get(12) {
              b.args::<Self, Branch12>(&mut out);
          }
          if let Some(b) = self.merge_branches.get(13) {
              b.args::<Self, Branch13>(&mut out);
          }
          if let Some(b) = self.merge_branches.get(14) {
              b.args::<Self, Branch14>(&mut out);
          }
          if let Some(b) = self.merge_branches.get(15) {
              b.args::<Self, Branch15>(&mut out);
          }

        assert!(self.merge_branches.len() < 16)
      },
      3 => { return self.context().unknown_args(); },
      _ => { return None; },
    }

    Some(out)
  }
}

impl Tool for Session {
  fn enqueue_commands(&mut self, queue: &mut CommandQueue<Self>)
                      -> Result<(), Box<Error>>
  {
    let rust_src = self.rust_src_path();

    let mut cmd = Command::new("git");
    cmd.current_dir(&rust_src)
      .arg("config")
      .arg("--replace-all")
      .arg("merge.renamelimit")
      .arg("3000");
    queue.enqueue_simple_external(Some("merge.renamelimit"),
                                  cmd, None);

    self.rust_repo
      .add_remote_from(&rust_src,
                       &self.upstream_repo,
                       queue)?;

    for branch in self.merge_branches.iter() {
      self.rust_repo
        .add_remote_from(&rust_src, branch, queue)?;
    }

    self.rust_repo.update_remotes(&rust_src, queue);

    self.rust_repo.create_or_reset_branch(&rust_src,
                                          "mir-hsa-merge-head",
                                          &self.upstream_repo,
                                          queue)?;

    for branch in self.merge_branches.iter() {
      self.rust_repo
        .merge_branch(&rust_src, branch, queue)?;
    }

    let mut cmd = Command::new("git");
    cmd.current_dir(&rust_src)
      .arg("submodule")
      .arg("foreach")
      .arg("git fetch");
    queue.enqueue_simple_external(Some("fetch submodules"),
                                  cmd, None);

    let mut cmd = Command::new("git");
    cmd.current_dir(&rust_src)
      .arg("submodule")
      .arg("update")
      .arg("--checkout")
      .arg("--init")
      .arg("--jobs")
      .arg("4");
    queue.enqueue_simple_external(Some("update submodules"),
                                  cmd, None);

    Ok(())
  }

  fn get_name(&self) -> String {
    "rust-dist-merger".to_string()
  }
  fn add_tool_input(&mut self, _i: PathBuf) -> Result<(), Box<Error>> {
    unimplemented!();
  }
  fn get_output(&self) -> Option<&PathBuf> {
    None
  }
  fn override_output(&mut self, _i: PathBuf) {
    unimplemented!();
  }
}

const UPSTREAM_RUST_URL: &'static str = "https://github.com/rust-lang/rust.git";
const RUST_URL: &'static str = "git@github.com:DiamondLovesYou/rust.git";
const RUST_MAIN_BRANCH: &'static str = "master";


const BRANCHES_URL: &'static str = "git@bitbucket.org:DiamondLovesYou/rust-mir-hsa.git";
const MERGE_BRANCHES: &'static [&'static str] = &[
  "fix-clang-and-lldb-builds",
  "fix-rustc-logging",
  "getopts-deps",
  "rustc-trans-addr-space",
  "addr-space-attr",
  "plugin-intrinsics",
  "always-export-metadata",
  "make-metadata-schema-pub",
  "reexport-env_logger",
  "polly",
  "amdgpu-intrinsics",
  "fix-llvm-amdgpu",
  "tcx-driver-data",
  "syntax-global-new-pub",
  "fix-compiler-docs-parallel-queries",
];

impl Session {

}

pub fn main() {
  let _ = util::main::<Session>(None);
}
