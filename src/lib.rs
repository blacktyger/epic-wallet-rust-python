use epic_wallet_rust_lib::{
    get_wallet_secret_key_pair,
    _build_subscribe_request,
    build_post_slate_request,
    process_received_slates,
    decrypt_epicbox_slates,
    get_epicbox_address,
    EpicBoxConfig,
    open_wallet,
    tx_create,
    tx_post,
    txs_get
};

use pyo3::prelude::*;
use serde_json;


#[pyfunction]
fn build_post_slate_request_py(
    config: String, receiver_address: String,
    slate: String, epicbox_config: String, password: String
) -> PyResult<String> {

    //TODO: epic-box cfg changes, keep an eye
    let box_cfg = serde_json::from_str::<EpicBoxConfig>(&epicbox_config);
    // let box_cfg = EpicBoxConfig::from_str(&epicbox_config).unwrap();

    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let key_pair =
        get_wallet_secret_key_pair(&wallet, Some(keychain.unwrap()), 0);

    let slate_msg = build_post_slate_request(
        &receiver_address.to_string(),
        key_pair.unwrap(),
        slate.to_string(),
        box_cfg.unwrap()
    );
    Ok(slate_msg)
}


#[pyfunction]
fn build_subscribe_request_py(
    config: String, epicbox_config: String, password: String) -> PyResult<String> {

    //TODO: cfg changes, keep an eye
    let box_cfg = serde_json::from_str::<EpicBoxConfig>(&epicbox_config).unwrap();
    // let box_cfg = EpicBoxConfig::from_str(&epicbox_config).unwrap();

    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let key_pair =
        get_wallet_secret_key_pair(&wallet, Some(keychain.unwrap()), 0);
    let subscribe_request =
        _build_subscribe_request(key_pair.unwrap(), box_cfg);
    Ok(subscribe_request)
}


#[pyfunction]
fn decrypt_slates_py(config: String, password: String, encrypted_slates: String) -> PyResult<Vec<String>> {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let key_pair =
        get_wallet_secret_key_pair(&wallet, Some(keychain.unwrap()), 0);
    let decrypted_slates =
        decrypt_epicbox_slates(key_pair.unwrap(), &encrypted_slates);
    Ok(decrypted_slates.unwrap())
}


#[pyfunction]
fn process_slate_py(config: String, password: String, slate: String) -> PyResult<String> {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();

    let processed_slates = process_received_slates(
        &wallet, Some(keychain.unwrap()), &slate);
    Ok(processed_slates.unwrap())
}


#[pyfunction]
fn get_txs_py(config: String, password: String) -> PyResult<String> {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let txs = txs_get(&wallet, keychain, true);
    Ok(txs.unwrap())
}


#[pyfunction]
fn create_slate_py(config: String, password: String, args: (u64, u64, bool)) -> PyResult<String> {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let create =
        tx_create(&wallet, keychain, args.0,
                  args.1, args.2).unwrap();

    Ok(create)
}


#[pyfunction]
fn tx_post_py(config: String, password: String, tx_slate_id: &str) -> PyResult<String> {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let post_tx = tx_post(&wallet, keychain, tx_slate_id);

    Ok(post_tx.unwrap())
}


#[pyfunction]
fn get_epicbox_address_py(config: String, password: String, domain: String, port: u16) -> PyResult<String> {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let key_pair =
        get_wallet_secret_key_pair(&wallet, Some(keychain.unwrap()), 0);
    let address =
        get_epicbox_address(key_pair.unwrap().1, &domain, Some(port));

    Ok(address.to_string())
}

// wrap _py function and make them available as python package
#[pymodule]
fn epic_wallet_rust_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build_post_slate_request_py, m)?)?;
    m.add_function(wrap_pyfunction!(build_subscribe_request_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_epicbox_address_py, m)?)?;
    m.add_function(wrap_pyfunction!(decrypt_slates_py, m)?)?;
    m.add_function(wrap_pyfunction!(process_slate_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_slate_py, m)?)?;
    m.add_function(wrap_pyfunction!(tx_post_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_txs_py, m)?)?;
    Ok(())
}