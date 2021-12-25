use maplit::btreeset;

use crate::raft::MembershipConfig;

#[test]
fn test_membership() -> anyhow::Result<()> {
    let m1 = MembershipConfig::new_multi(vec![btreeset! {1}]);
    let m123 = MembershipConfig::new_multi(vec![btreeset! {1,2,3}]);
    let m123_345 = MembershipConfig::new_multi(vec![btreeset! {1,2,3}, btreeset! {3,4,5}]);

    assert_eq!(Some(btreeset! {1}), m1.get_ith_config(0).cloned());
    assert_eq!(Some(btreeset! {1,2,3}), m123.get_ith_config(0).cloned());
    assert_eq!(Some(btreeset! {1,2,3}), m123_345.get_ith_config(0).cloned());

    assert_eq!(None, m1.get_ith_config(1).cloned());
    assert_eq!(None, m123.get_ith_config(1).cloned());
    assert_eq!(Some(btreeset! {3,4,5}), m123_345.get_ith_config(1).cloned());

    assert_eq!(vec![1], m1.ith_config(0));
    assert_eq!(vec![1, 2, 3], m123.ith_config(0));
    assert_eq!(vec![1, 2, 3], m123_345.ith_config(0));
    assert_eq!(vec![3, 4, 5], m123_345.ith_config(1));

    assert_eq!(&btreeset! {1}, m1.all_nodes());
    assert_eq!(&btreeset! {1,2,3}, m123.all_nodes());
    assert_eq!(&btreeset! {1,2,3,4,5}, m123_345.all_nodes());

    assert!(!m1.contains(&0));
    assert!(m1.contains(&1));
    assert!(m123_345.contains(&4));
    assert!(!m123_345.contains(&6));

    assert!(!m123.is_in_joint_consensus());
    assert!(m123_345.is_in_joint_consensus());

    assert_eq!(
        MembershipConfig::new_single(btreeset! {3,4,5}),
        m123_345.to_final_config()
    );

    Ok(())
}

#[test]
fn test_membership_update() -> anyhow::Result<()> {
    // --- replace

    let mut m123 = MembershipConfig::new_single(btreeset! {1,2,3});
    m123.replace(vec![btreeset! {2,3}, btreeset! {3,4}]);

    assert_eq!(&btreeset! {2,3,4}, m123.all_nodes());
    assert_eq!(&vec![btreeset! {2,3}, btreeset! {3,4}], m123.get_configs());

    // --- push

    m123.push(btreeset! {3,5});

    assert_eq!(&btreeset! {2,3,4,5}, m123.all_nodes());
    assert_eq!(
        &vec![btreeset! {2,3}, btreeset! {3,4}, btreeset! {3,5}],
        m123.get_configs()
    );

    // --- to final

    let got = m123.to_final_config();

    assert_eq!(&btreeset! {3,5}, got.all_nodes());
    assert_eq!(&vec![btreeset! {3,5}], got.get_configs());

    Ok(())
}

#[test]
fn test_membership_majority() -> anyhow::Result<()> {
    {
        let m12345 = MembershipConfig::new_single(btreeset! {1,2,3,4,5});
        assert!(!m12345.is_majority(&btreeset! {0}));
        assert!(!m12345.is_majority(&btreeset! {0,1,2}));
        assert!(!m12345.is_majority(&btreeset! {6,7,8}));
        assert!(m12345.is_majority(&btreeset! {1,2,3}));
        assert!(m12345.is_majority(&btreeset! {3,4,5}));
        assert!(m12345.is_majority(&btreeset! {1,3,4,5}));
    }

    {
        let m12345_123 = MembershipConfig::new_multi(vec![btreeset! {1,2,3,4,5}, btreeset! {6,7,8}]);
        assert!(!m12345_123.is_majority(&btreeset! {0}));
        assert!(!m12345_123.is_majority(&btreeset! {0,1,2}));
        assert!(!m12345_123.is_majority(&btreeset! {6,7,8}));
        assert!(!m12345_123.is_majority(&btreeset! {1,2,3}));
        assert!(m12345_123.is_majority(&btreeset! {1,2,3,6,7}));
        assert!(m12345_123.is_majority(&btreeset! {1,2,3,4,7,8}));
    }

    Ok(())
}