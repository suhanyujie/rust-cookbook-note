use bincode::serialize_into;
use crc::crc32;
use protobuf::Message;
use raft::eraftpb::Snapshot as RaftSnapshot;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

const SNAP_EXT: &'static str = "snap";

pub struct Snapshotter {
    dir: PathBuf,
}

impl Snapshotter {
    pub fn new<P: AsRef<Path>>(p: P) -> Snapshotter {
        Snapshotter {
            dir: p.as_ref().to_path_buf(),
        }
    }

    pub fn save(&self, snapshot: &RaftSnapshot) -> crate::Result<()> {
        let md = snapshot.metadata.get_ref();
        let spath = self.new_snap_path(md.index, md.term);
        let b = snapshot.write_to_bytes()?;
        let crc = crc32::update(0, &crc32::CASTAGNOLI_TABLE, &b);
        // 直接写入字符串
        let test_s1 = "hello world";
        match serialize_into(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&spath)?,
            &test_s1.as_bytes(),
        ) {
            Err(err) => {
                eprintln!(
                    "failed to write a snap file {} error {}",
                    spath.to_string_lossy(),
                    err
                );
                match fs::remove_file(spath.as_path()) {
                    Err(err) => {
                        eprintln!(
                            "failed to remove a broken snap file {} error {}",
                            spath.to_string_lossy(),
                            err
                        );
                    }
                    _ => (),
                }
                eprintln!("{:?}", err);
                // 暂时返回 ok
                Ok(())
            }
            Ok(v) => Ok(v),
        }
    }

    pub fn new_snap_path(&self, index: u64, term: u64) -> PathBuf {
        self.dir
            .join(format!("{:016x}-{:016x}.{}", term, index, SNAP_EXT))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raft::eraftpb::Snapshot as RaftSnapshot;
    use raft::eraftpb::SnapshotMetadata;
    use std::fs;

    fn new_snapshot(index: u64, term: u64) -> RaftSnapshot {
        let mut snap = RaftSnapshot::new();
        let mut meta = SnapshotMetadata::new();
        meta.set_index(index);
        meta.set_term(term);
        snap.set_metadata(meta);
        snap
    }

    #[test]
    fn test_save() {
        let index = 1;
        let term = 1;
        let snap_dir = "./data";
        let snap_shotter = Snapshotter::new(snap_dir);
        let snap_path = snap_shotter.new_snap_path(index, term);
        let snap = new_snapshot(index, term);
        snap_shotter.save(&snap).unwrap();
        assert!(snap_path.exists());
        // fs::remove_file(snap_path).unwrap();
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
