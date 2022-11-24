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
    tx_cancel,
    tx_post,
    txs_get,
};
use stack_test_epic_wallet_libwallet::{Error, ErrorKind};
use pyo3::prelude::*;
use serde_json;
use serde_json::json;
use uuid::Uuid;


pub enum ResultData {
    AsStringResult(Result<String, Error>),
    AsVectorResult(Result<Vec<String>, Error>),
    AsStringTuple((String, bool))
}


#[pyfunction]
fn post_request_py(
    config: String, receiver_address: String,
    slate: String, epicbox_config: String, password: String
) -> String {

    let box_cfg = serde_json::from_str::<EpicBoxConfig>(&epicbox_config);
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
    return result_response(ResultData::AsStringTuple((slate_msg, true)))
}

#[pyfunction]
fn subscribe_request_py(
    config: String, epicbox_config: String, password: String) -> String {
    let box_cfg = serde_json::from_str::<EpicBoxConfig>(&epicbox_config).unwrap();
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let key_pair =
        get_wallet_secret_key_pair(&wallet, Some(keychain.unwrap()), 0);
    let subscribe_request =
        _build_subscribe_request(key_pair.unwrap(), box_cfg);

    return result_response(ResultData::AsStringTuple((subscribe_request, true)))
}

#[pyfunction]
fn decrypt_slates_py(config: String, password: String, encrypted_slates: String) -> String {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let key_pair =
        get_wallet_secret_key_pair(&wallet, Some(keychain.unwrap()), 0);
    let decrypted_slates =
        decrypt_epicbox_slates(key_pair.unwrap(), &encrypted_slates);

    return result_response(ResultData::AsVectorResult(decrypted_slates))
}

#[pyfunction]
fn process_slate_py(config: String, password: String, slate: String) -> String {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();

    let processed_slates = process_received_slates(
        &wallet, Some(keychain.unwrap()), &slate);

    return result_response(ResultData::AsStringResult(processed_slates))

}

#[pyfunction]
fn create_tx_py(config: String, password: String, args: (u64, u64, bool)) -> String {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let new_slate = tx_create(&wallet, keychain, args.0,
              args.1, args.2);
    return result_response(ResultData::AsStringResult(new_slate))
}

#[pyfunction]
fn post_tx_py(config: String, password: String, tx_slate_id: &str) -> String {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let post_tx = tx_post(&wallet, keychain, tx_slate_id);

    return result_response(ResultData::AsStringResult(post_tx))
}

#[pyfunction]
fn get_txs_py(config: String, password: String) -> String {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let txs = txs_get(&wallet, keychain, true);

    return result_response(ResultData::AsStringResult(txs))
}

#[pyfunction]
fn cancel_tx_py(config: String, password: String, tx_slate_id: &str) -> String {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let tx_slate_uuid = Uuid::parse_str(tx_slate_id).
        map_err(|e| ErrorKind::GenericError(e.to_string())).unwrap();
    let cancel =
        tx_cancel(&wallet, keychain, tx_slate_uuid);

    return result_response(ResultData::AsStringResult(cancel))
}

#[pyfunction]
fn get_epicbox_address_py(config: String, password: String, domain: String, port: u16) -> String {
    let (wallet, keychain) =
        open_wallet(&config, &password.to_string()).unwrap();
    let key_pair =
        get_wallet_secret_key_pair(&wallet, Some(keychain.unwrap()), 0);
    let address =
        get_epicbox_address(key_pair.unwrap().1, &domain, Some(port));

    return result_response(ResultData::AsStringTuple((address.to_string(), true)))
}

// wrap _py function and make them available as python package
#[pymodule]
fn epic_wallet_rust_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_epicbox_address_py, m)?)?;
    m.add_function(wrap_pyfunction!(subscribe_request_py, m)?)?;
    m.add_function(wrap_pyfunction!(decrypt_slates_py, m)?)?;
    m.add_function(wrap_pyfunction!(process_slate_py, m)?)?;
    m.add_function(wrap_pyfunction!(post_request_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_tx_py, m)?)?;
    m.add_function(wrap_pyfunction!(cancel_tx_py, m)?)?;
    m.add_function(wrap_pyfunction!(post_tx_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_txs_py, m)?)?;
    Ok(())
}


fn result_response(data: ResultData) -> String {
    return match data {
        ResultData::AsStringResult(result) => {
            match result {
                Ok(slate) => {
                    serde_json::to_string(&json!({
                    "error": 0,
                    "message": "success",
                    "result": slate
                })).unwrap()
                },

                Err(err) => {
                    serde_json::to_string(&json!({
                    "error": 1,
                    "message": err.to_string(),
                    "result": 0
                 })).unwrap()
                }
            }
        },
        ResultData::AsVectorResult(result) => {
            match result {
                Ok(slates) => {
                    serde_json::to_string(&json!({
                    "error": 0,
                    "message": "success",
                    "result": slates
                })).unwrap()
                },
                Err(err) => {
                    serde_json::to_string(&json!({
                    "error": 1,
                    "message": err.to_string(),
                    "result": 0
                 })).unwrap()
                }
            }
        },
        ResultData::AsStringTuple(result) => {
            if result.1 {
                serde_json::to_string(&json!({
                    "error": 0,
                    "message": "success",
                    "result": result.0
                })).unwrap()
            } else {
                serde_json::to_string(&json!({
                    "error": 1,
                    "message": result.0,
                    "result": 0
                })).unwrap()
            }
        }
    }
}
