use darkfi_sdk::{
    crypto::ContractId,
    db::{db_begin_tx, db_end_tx, db_get, db_init, db_lookup, db_set},
    define_contract,
    error::ContractResult,
    msg,
    tx::FuncCall,
    util::set_return_data,
};
use darkfi_serial::{deserialize, serialize, SerialDecodable, SerialEncodable};

/// Available functions for this contract.
/// We identify them with the first byte passed in through the payload.
#[repr(u8)]
pub enum Function {
    Foo = 0x00,
    Bar = 0x01,
}

impl From<u8> for Function {
    fn from(b: u8) -> Self {
        match b {
            0x00 => Self::Foo,
            0x01 => Self::Bar,
            _ => panic!("Invalid function ID: {:#04x?}", b),
        }
    }
}

// An example of deserializing the payload into a struct
#[derive(SerialEncodable, SerialDecodable)]
pub struct FooCallData {
    pub a: u64,
    pub b: u64,
}

impl FooCallData {
    //fn zk_public_values(&self) -> Vec<(String, Vec<DrkCircuitField>)>;

    //fn get_metadata(&self) {
    //}
}

#[derive(SerialEncodable, SerialDecodable)]
pub struct BarArgs {
    pub x: u32,
}

#[derive(SerialEncodable, SerialDecodable)]
pub struct FooUpdate {
    pub name: String,
    pub age: u32,
}

define_contract!(
    init: init_contract,
    exec: process_instruction,
    apply: process_update,
    metadata: get_metadata
);

fn init_contract(cid: ContractId, _ix: &[u8]) -> ContractResult {
    msg!("wakeup wagies!");
    db_init(cid, "wagies")?;
    // TODO: If the deploy execution fails, whatever is initialized with db_init
    //       should be deleted from sled afterwards. There's no way to create a
    //       tree but only apply the creation when we're done, so db_init creates
    //       it and upon failure it should delete it

    // Lets write a value in there
    let tx_handle = db_begin_tx()?;
    db_set(tx_handle, "jason_gulag".as_bytes(), serialize(&110))?;
    let db_handle = db_lookup("wagies")?;
    db_end_tx(db_handle, tx_handle)?;

    // Host will clear delete the batches array after calling this func.

    Ok(())
}

fn get_metadata(_cid: ContractId, ix: &[u8]) -> ContractResult {
    match Function::from(ix[0]) {
        Function::Foo => {
            let tx_data = &ix[1..];
            // ...
            let (func_call_index, func_calls): (u32, Vec<FuncCall>) = deserialize(tx_data)?;
            let _call_data: FooCallData =
                deserialize(&func_calls[func_call_index as usize].call_data)?;

            // Convert call_data to halo2 public inputs
            // Pass this to the env
        }
        Function::Bar => {
            // ...
        }
    }
    Ok(())
}

// This is the main entrypoint function where the payload is fed.
// Through here, you can branch out into different functions inside
// this library.
fn process_instruction(_cid: ContractId, ix: &[u8]) -> ContractResult {
    match Function::from(ix[0]) {
        Function::Foo => {
            let tx_data = &ix[1..];
            // ...
            let (func_call_index, func_calls): (u32, Vec<FuncCall>) = deserialize(tx_data)?;
            let call_data: FooCallData =
                deserialize(&func_calls[func_call_index as usize].call_data)?;
            msg!("call_data {{ a: {}, b: {} }}", call_data.a, call_data.b);
            // ...
            let update = FooUpdate { name: "john_doe".to_string(), age: 110 };

            let mut update_data = vec![Function::Foo as u8];
            update_data.extend_from_slice(&serialize(&update));
            set_return_data(&update_data)?;
            msg!("update is set!");

            // Example: try to get a value from the db
            let db_handle = db_lookup("wagies")?;
            // FIXME: this is just empty right now
            let age_data = db_get(db_handle, "jason_gulag".as_bytes())?;
            msg!("wagie age data: {:?}", age_data);
        }
        Function::Bar => {
            let tx_data = &ix[1..];
            // ...
            let _args: BarArgs = deserialize(tx_data)?;
        }
    }

    Ok(())
}

fn process_update(_cid: ContractId, update_data: &[u8]) -> ContractResult {
    msg!("Make update!");

    match Function::from(update_data[0]) {
        Function::Foo => {
            let update: FooUpdate = deserialize(&update_data[1..])?;

            // Write the wagie to the db
            let tx_handle = db_begin_tx()?;
            db_set(tx_handle, update.name.as_bytes(), serialize(&update.age))?;
            let db_handle = db_lookup("wagies")?;
            db_end_tx(db_handle, tx_handle)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
