use crate::protocol as pb;
use std::convert::From;
use super::common::SettableValue;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TableOptions {
    pub capacity_unit: CapacityUnit,
    pub time_to_live: SettableValue<chrono::Duration>,
    pub max_versions: SettableValue<i32>,
    pub deviated_duration: SettableValue<chrono::Duration>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CapacityUnit {
    pub read: SettableValue<i32>,
    pub write: SettableValue<i32>,
}

impl TableOptions {
    pub fn default_for_create() -> Self {
        Self{
            capacity_unit: CapacityUnit::default_for_create(),
            time_to_live: SettableValue::Default,
            max_versions: SettableValue::Default,
            deviated_duration: SettableValue::Default,
        }
    }

    pub fn default_for_update() -> Self {
        Self{
            capacity_unit: CapacityUnit::default_for_update(),
            time_to_live: SettableValue::NoChange,
            max_versions: SettableValue::NoChange,
            deviated_duration: SettableValue::NoChange,
        }
    }
}

impl CapacityUnit {
    pub fn default_for_create() -> Self {
        Self{
            read: SettableValue::Default,
            write: SettableValue::Default,
        }
    }

    pub fn default_for_update() -> Self {
        Self{
            read: SettableValue::NoChange,
            write: SettableValue::NoChange,
        }
    }
}

impl From<TableOptions> for (pb::TableOptions, pb::CapacityUnit) {
    fn from(x: TableOptions) -> Self {
        let proto_opts = {
            let time_to_live = match x.time_to_live {
                SettableValue::Value(ttl) => Some(ttl.num_seconds() as i32),
                SettableValue::Default => Some(-1),
                SettableValue::NoChange => None,
            };
            let max_versions = match x.max_versions {
                SettableValue::Value(mv) => Some(mv),
                _ => None,
            };
            let deviation_cell_version_in_sec = match x.deviated_duration {
                SettableValue::Value(dd) => Some(dd.num_seconds()),
                _ => None,
            };
            pb::TableOptions{
                time_to_live,
                max_versions,
                bloom_filter_type: None,
                block_size: None,
                deviation_cell_version_in_sec,
            }
        };
        let proto_cu = {
            let read = match x.capacity_unit.read {
                SettableValue::Value(r) => Some(r),
                _ => None,
            };
            let write = match x.capacity_unit.write {
                SettableValue::Value(w) => Some(w),
                _ => None,
            };
            pb::CapacityUnit{
                read,
                write,
            }
        };
        (proto_opts, proto_cu)
    }
}

impl From<(pb::TableOptions, pb::CapacityUnit)> for TableOptions {
    fn from(x: (pb::TableOptions, pb::CapacityUnit)) -> Self {
        let time_to_live = if let Some(ttl) = x.0.time_to_live {
            SettableValue::Value(chrono::Duration::seconds(ttl.into()))
        } else {
            SettableValue::Default
        };
        let max_versions = if let Some(mv) = x.0.max_versions {
            SettableValue::Value(mv)
        } else {
            SettableValue::Default
        };
        let deviated_duration = if let Some(dd) = x.0.deviation_cell_version_in_sec {
            SettableValue::Value(chrono::Duration::seconds(dd))
        } else {
            SettableValue::Default
        };
        let capacity_unit = {
            let read = match x.1.read {
                Some(x) => SettableValue::Value(x),
                None => SettableValue::Default,
            };
            let write = match x.1.write {
                Some(x) => SettableValue::Value(x),
                None => SettableValue::Default,
            };
            CapacityUnit{
                read,
                write,
            }
        };
        TableOptions{
            capacity_unit,
            time_to_live,
            max_versions,
            deviated_duration,
        }
    }
}

#[cfg(test)]
impl Arbitrary for TableOptions {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let time_to_live = {
            let ttl = u16::arbitrary(g);
            SettableValue::Value(chrono::Duration::seconds(ttl as i64))
        };
        let max_versions = {
            let mv = u16::arbitrary(g);
            SettableValue::Value(mv as i32)
        };
        let deviated_duration = {
            let dd = u32::arbitrary(g);
            SettableValue::Value(chrono::Duration::seconds(dd as i64))
        };
        Self{
            capacity_unit: CapacityUnit::arbitrary(g),
            time_to_live,
            max_versions,
            deviated_duration,
        }
    }
}

#[cfg(test)]
impl Arbitrary for CapacityUnit {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let read = {
            let read = i32::arbitrary(g);
            SettableValue::Value(read)
        };
        let write = {
            let write = i32::arbitrary(g);
            SettableValue::Value(write)
        };
        Self{
            read,
            write,
        }
    }
}

#[cfg(test)]
mod ut {
    use super::*;
    use tokio::stream::StreamExt;
    use tokio::sync::*;

    #[quickcheck]
    fn tableoptions_serde_is_identity_0(oracle: TableOptions) -> bool {
        println!("oracle: {:?}", oracle);
        let middle = <(pb::TableOptions, pb::CapacityUnit)>::from(oracle.clone());
        println!("middle: {:?}", middle);
        let trial = TableOptions::from(middle);
        println!("trial {:?}", trial);
        oracle == trial
    }

    fn permute<T0, T1, T2, T3, T4>(
        vs0: Vec<T0>,
        vs1: Vec<T1>,
        vs2: Vec<T2>,
        vs3: Vec<T3>,
        vs4: Vec<T4>,
    ) -> impl tokio::stream::StreamExt<Item=(T0, T1, T2, T3, T4)>
    where
        T0: Clone + Send + Sync + 'static,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        T4: Clone + Send + Sync + 'static,
    {
        let (mut tx, rx) = mpsc::channel(1);
        tokio::spawn(async move {
            for v0 in vs0.iter() {
                for v1 in vs1.iter() {
                    for v2 in vs2.iter() {
                        for v3 in vs3.iter() {
                            for v4 in vs4.iter() {
                                let v0 = v0.clone();
                                let v1 = v1.clone();
                                let v2 = v2.clone();
                                let v3 = v3.clone();
                                let v4 = v4.clone();
                                tx.send((v0, v1, v2, v3, v4)).await
                                    .unwrap_or_else(|x| {
                                        println!("Failed to send something into a mpsc channel.");
                                    });
                            }
                        }
                    }
                }
            }
        });
        rx
    }


    #[tokio::test]
    async fn tableoptions_for_update() {
        let ttls = vec![
            SettableValue::Value(chrono::Duration::seconds(1)), 
            SettableValue::NoChange];
        let mvs = vec![SettableValue::Value(1), SettableValue::NoChange];
        let dds = vec![
            SettableValue::Value(chrono::Duration::seconds(1)), 
            SettableValue::NoChange];
        let rcus = vec![
            SettableValue::Value(1),
            SettableValue::NoChange];
        let wcus = vec![
            SettableValue::Value(2),
            SettableValue::NoChange];
        let mut rx = permute(ttls, mvs, dds, rcus, wcus);
        while let Some((ttl, mv, dd, rcu, wcu)) = rx.next().await {
            let oracle = TableOptions{
                capacity_unit: CapacityUnit{
                    read: rcu,
                    write: wcu,
                },
                time_to_live: ttl,
                max_versions: mv,
                deviated_duration: dd,
            };
            type TOCU = (pb::TableOptions, pb::CapacityUnit);
            let (trial_opts, trial_cu) = TOCU::from(oracle.clone());
            
            if let SettableValue::Value(ttl) = oracle.time_to_live {
                assert!(trial_opts.time_to_live.is_some(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
                assert!(
                    ttl.num_seconds() == trial_opts.time_to_live.unwrap().into(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            } else {
                assert!(trial_opts.time_to_live.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            }

            if let SettableValue::Value(mv) = oracle.max_versions {
                assert!(trial_opts.max_versions.is_some(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
                assert!(mv == trial_opts.max_versions.unwrap(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            } else {
                assert!(trial_opts.max_versions.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            }

            if let SettableValue::Value(dd) = oracle.deviated_duration {
                assert!(trial_opts.deviation_cell_version_in_sec.is_some(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
                let trial_dd = trial_opts.deviation_cell_version_in_sec.unwrap();
                assert!(dd.num_seconds() == trial_dd.into(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            } else {
                assert!(trial_opts.deviation_cell_version_in_sec.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            }

            match oracle.capacity_unit.read {
                SettableValue::Value(oracle_rcu) => {
                    assert!(trial_cu.read.is_some(),
                        "oracle: {:?} trial: {:?}",
                        oracle,
                        trial_cu);
                    let trial_rcu = trial_cu.read.unwrap();
                    assert_eq!(oracle_rcu, trial_rcu);
                }
                _ => {
                    assert!(trial_cu.read.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_cu);
                }
            }

            match oracle.capacity_unit.write {
                SettableValue::Value(oracle_wcu) => {
                    assert!(trial_cu.write.is_some(),
                        "oracle: {:?} trial: {:?}",
                        oracle,
                        trial_cu);
                    let trial_wcu = trial_cu.write.unwrap();
                    assert_eq!(oracle_wcu, trial_wcu);
                }
                _ => {
                    assert!(trial_cu.write.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_cu);
                }
            }
        }
    }

    #[tokio::test]
    async fn tableoptions_for_create() {
        let ttls = vec![
            SettableValue::Value(chrono::Duration::seconds(1)), 
            SettableValue::Default];
        let mvs = vec![SettableValue::Value(1), SettableValue::Default];
        let dds = vec![
            SettableValue::Value(chrono::Duration::seconds(1)), 
            SettableValue::Default];
        let rcus = vec![
            SettableValue::Value(1),
            SettableValue::Default];
        let wcus = vec![
            SettableValue::Value(2),
            SettableValue::Default];
        let mut rx = permute(ttls, mvs, dds, rcus, wcus);
        while let Some((ttl, mv, dd, rcu, wcu)) = rx.next().await {
            let oracle = TableOptions{
                capacity_unit: CapacityUnit{
                    read: rcu,
                    write: wcu,
                },
                time_to_live: ttl,
                max_versions: mv,
                deviated_duration: dd,
            };
            type TOCU = (pb::TableOptions, pb::CapacityUnit);
            let (trial_opts, trial_cu) = TOCU::from(oracle.clone());

            if let SettableValue::Value(ttl) = oracle.time_to_live {
                let o_ttl = ttl.num_seconds();
                assert!(trial_opts.time_to_live.is_some());
                let t_ttl = trial_opts.time_to_live.unwrap();
                assert!(o_ttl == t_ttl.into(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            } else {
                assert!(trial_opts.time_to_live.is_some());
                let t_ttl = trial_opts.time_to_live.unwrap();
                assert!(t_ttl == -1,
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            }

            if let SettableValue::Value(mv) = oracle.max_versions {
                assert!(trial_opts.max_versions.is_some(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
                assert!(mv == trial_opts.max_versions.unwrap(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            } else {
                assert!(trial_opts.max_versions.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            }

            if let SettableValue::Value(dd) = oracle.deviated_duration {
                assert!(trial_opts.deviation_cell_version_in_sec.is_some(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
                let trial_dd = trial_opts.deviation_cell_version_in_sec.unwrap();
                assert!(dd.num_seconds() == trial_dd.into(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            } else {
                assert!(trial_opts.deviation_cell_version_in_sec.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_opts
                );
            }

            match oracle.capacity_unit.read {
                SettableValue::Value(oracle_rcu) => {
                    assert!(trial_cu.read.is_some(),
                        "oracle: {:?} trial: {:?}",
                        oracle,
                        trial_cu);
                    let trial_rcu = trial_cu.read.unwrap();
                    assert_eq!(oracle_rcu, trial_rcu);
                }
                _ => {
                    assert!(trial_cu.read.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_cu);
                }
            }

            match oracle.capacity_unit.write {
                SettableValue::Value(oracle_wcu) => {
                    assert!(trial_cu.write.is_some(),
                        "oracle: {:?} trial: {:?}",
                        oracle,
                        trial_cu);
                    let trial_wcu = trial_cu.write.unwrap();
                    assert_eq!(oracle_wcu, trial_wcu);
                }
                _ => {
                    assert!(trial_cu.write.is_none(),
                    "oracle: {:?} trial: {:?}",
                    oracle,
                    trial_cu);
                }
            }
        }
    }
}
