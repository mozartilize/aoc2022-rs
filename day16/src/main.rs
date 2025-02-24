use std::collections::HashMap;

use day16::{Container, Vault};

fn main() {
    let zt_v = Vault::new("ZT".to_string(), 0);
    let jx_v = Vault::new("JX".to_string(), 22);
    let em_v = Vault::new("EM".to_string(), 0);
    let aa_v = Vault::new("AA".to_string(), 0);
    let hw_v = Vault::new("HW".to_string(), 0);
    let ik_v = Vault::new("IK".to_string(), 8);
    let ha_v = Vault::new("HA".to_string(), 0);
    let wh_v = Vault::new("WH".to_string(), 12);
    let ku_v = Vault::new("KU".to_string(), 0);
    let qd_v = Vault::new("QD".to_string(), 0);
    let cf_v = Vault::new("CF".to_string(), 18);
    let vc_v = Vault::new("VC".to_string(), 0);
    let jt_v = Vault::new("JT".to_string(), 0);
    let qq_v = Vault::new("QQ".to_string(), 11);
    let zp_v = Vault::new("ZP".to_string(), 0);
    let li_v = Vault::new("LI".to_string(), 0);
    let ci_v = Vault::new("CI".to_string(), 0);
    let vk_v = Vault::new("VK".to_string(), 6);
    let wl_v = Vault::new("WL".to_string(), 20);
    let ti_v = Vault::new("TI".to_string(), 0);
    let nu_v = Vault::new("NU".to_string(), 0);
    let ds_v = Vault::new("DS".to_string(), 9);
    let he_v = Vault::new("HE".to_string(), 0);
    let zh_v = Vault::new("ZH".to_string(), 0);
    let to_v = Vault::new("TO".to_string(), 0);
    let cm_v = Vault::new("CM".to_string(), 0);
    let wm_v = Vault::new("WM".to_string(), 14);
    let ez_v = Vault::new("EZ".to_string(), 16);
    let pb_v = Vault::new("PB".to_string(), 0);
    let xl_v = Vault::new("XL".to_string(), 0);
    let lb_v = Vault::new("LB".to_string(), 17);
    let wq_v = Vault::new("WQ".to_string(), 0);
    let bv_v = Vault::new("BV".to_string(), 13);
    let rn_v = Vault::new("RN".to_string(), 0);
    let lw_v = Vault::new("LW".to_string(), 0);
    let np_v = Vault::new("NP".to_string(), 0);
    let mt_v = Vault::new("MT".to_string(), 0);
    let et_v = Vault::new("ET".to_string(), 0);
    let hg_v = Vault::new("HG".to_string(), 19);
    let mv_v = Vault::new("MV".to_string(), 0);
    let rt_v = Vault::new("RT".to_string(), 0);
    let on_v = Vault::new("ON".to_string(), 0);
    let mo_v = Vault::new("MO".to_string(), 0);
    let uy_v = Vault::new("UY".to_string(), 5);
    let ur_v = Vault::new("UR".to_string(), 0);
    let ym_v = Vault::new("YM".to_string(), 0);
    let rz_v = Vault::new("RZ".to_string(), 0);
    let ad_v = Vault::new("AD".to_string(), 0);
    let eh_v = Vault::new("EH".to_string(), 0);
    let eq_v = Vault::new("EQ".to_string(), 0);
    let kx_v = Vault::new("KX".to_string(), 0);
    let br_v = Vault::new("BR".to_string(), 0);
    let lc_v = Vault::new("LC".to_string(), 0);
    let yw_v = Vault::new("YW".to_string(), 0);
    let ec_v = Vault::new("EC".to_string(), 0);
    let it_v = Vault::new("IT".to_string(), 10);
    let ms_v = Vault::new("MS".to_string(), 0);
    let fr_v = Vault::new("FR".to_string(), 0);
    let zo_v = Vault::new("ZO".to_string(), 0);

    let vaults = vec![
        zt_v,
        jx_v,
        em_v,
        aa_v,
        hw_v,
        ik_v,
        ha_v,
        wh_v,
        ku_v,
        qd_v,
        cf_v,
        vc_v,
        jt_v,
        qq_v,
        zp_v,
        li_v,
        ci_v,
        vk_v,
        wl_v,
        ti_v,
        nu_v,
        ds_v,
        he_v,
        zh_v,
        to_v,
        cm_v,
        wm_v,
        ez_v,
        pb_v,
        xl_v,
        lb_v,
        wq_v,
        bv_v,
        rn_v,
        lw_v,
        np_v,
        mt_v,
        et_v,
        hg_v,
        mv_v,
        rt_v,
        on_v,
        mo_v,
        uy_v,
        ur_v,
        ym_v,
        rz_v,
        ad_v,
        eh_v,
        eq_v,
        kx_v,
        br_v,
        lc_v,
        yw_v,
        ec_v,
        it_v,
        ms_v,
        fr_v,
        zo_v,
    ];
    let links = vec![
        vec!["QQ", "DS"],
        vec!["CI", "ZH", "UR"],
        vec!["WH", "IT"],
        vec!["EQ", "QD", "NP", "ZP", "KX"],
        vec!["CI", "BV"],
        vec!["ET", "NU", "ZO", "XL", "QD"],
        vec!["WQ", "LB"],
        vec!["EM", "LW"],
        vec!["BV", "CF"],
        vec!["AA", "IK"],
        vec!["KU", "JT", "CM"],
        vec!["AD", "UY"],
        vec!["CF", "ZH"],
        vec!["ZT"],
        vec!["EZ", "AA"],
        vec!["LB", "CM"],
        vec!["HW", "JX"],
        vec!["YM", "LC", "HE", "NU", "TI"],
        vec!["LW", "TO"],
        vec!["VK", "YW"],
        vec!["VK", "IK"],
        vec!["NP", "MV", "FR", "ZT", "YW"],
        vec!["VK", "EQ"],
        vec!["JT", "JX"],
        vec!["MT", "WL"],
        vec!["LI", "CF"],
        vec!["MO", "WQ", "EC", "RN"],
        vec!["RT", "RZ", "ZP"],
        vec!["YM", "UY"],
        vec!["IK", "MS"],
        vec!["LI", "HA", "ON", "UR", "AD"],
        vec!["WM", "HA"],
        vec!["KU", "RT", "HW", "MO", "EH"],
        vec!["WM", "RZ"],
        vec!["WH", "WL"],
        vec!["AA", "DS"],
        vec!["TO", "HG"],
        vec!["IK", "EC"],
        vec!["MT"],
        vec!["UY", "DS"],
        vec!["BV", "EZ"],
        vec!["LB", "EH"],
        vec!["BV", "WM"],
        vec!["PB", "BR", "MS", "VC", "MV"],
        vec!["JX", "LB"],
        vec!["PB", "VK"],
        vec!["RN", "EZ"],
        vec!["VC", "LB"],
        vec!["ON", "BV"],
        vec!["AA", "HE"],
        vec!["AA", "BR"],
        vec!["UY", "KX"],
        vec!["VK", "IT"],
        vec!["TI", "DS"],
        vec!["ET", "WM"],
        vec!["LC", "EM"],
        vec!["UY", "XL"],
        vec!["DS", "ZO"],
        vec!["FR", "IK"],
    ];

    let c = Container::build(vaults, links);

    let pool = c
        .has_flow_rate_vaults()
        .iter()
        .map(|v| *v)
        .collect::<Vec<_>>();
    let path = vec![&c.borrow_idxes()["AA"]];
    let mut db = HashMap::new();
    dbg!(c.run3(30, 0, path, pool, &mut db));

    c.run_part2();
}
