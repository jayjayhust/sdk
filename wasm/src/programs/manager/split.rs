// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

use crate::{
    execute_program,
    get_process,
    inclusion_proof,
    log,
    types::{CurrentAleo, CurrentBlockMemory, IdentifierNative, ProcessNative, ProgramNative, TransactionNative},
    PrivateKey,
    RecordPlaintext,
    Transaction,
};

use js_sys::Array;
use rand::{rngs::StdRng, SeedableRng};
use std::{ops::Add, str::FromStr};

#[wasm_bindgen]
impl ProgramManager {
    /// Split an Aleo credits record into two separate records. This function does not require a fee.
    ///
    /// @param private_key The private key of the sender
    /// @param split_amount The amount of the credit split. This amount will be subtracted from the
    /// value of the record and two new records will be created with the split amount and the remainder
    /// @param amount_record The record to split
    /// @param url The url of the Aleo network node to send the transaction to
    /// @param cache Cache the proving and verifying keys in the ProgramManager memory. If this is
    /// set to `true` the keys synthesized (or passed in as optional parameters via the
    /// `split_proving_key` and `split_verifying_key` arguments) will be stored in the
    /// ProgramManager's memory and used for subsequent transactions. If this is set to `false` the
    /// proving and verifying keys will be deallocated from memory after the transaction is executed
    /// @param split_proving_key (optional) Provide a proving key to use for the split function
    /// @param split_verifying_key (optional) Provide a verifying key to use for the split function
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn split(
        &mut self,
        private_key: PrivateKey,
        split_amount: f64,
        amount_record: RecordPlaintext,
        url: String,
        cache: bool,
        split_proving_key: Option<ProvingKey>,
        split_verifying_key: Option<VerifyingKey>,
    ) -> Result<Transaction, String> {
        log("Executing split program");
        let amount_microcredits = Self::validate_amount(split_amount, &amount_record, false)?;

        log("Setup the program and inputs");
        let program = ProgramNative::credits().unwrap().to_string();
        let inputs = Array::new_with_length(2u32);
        inputs.set(0u32, wasm_bindgen::JsValue::from_str(&amount_record.to_string()));
        inputs.set(1u32, wasm_bindgen::JsValue::from_str(&amount_microcredits.to_string().add("u64")));

        let mut new_process;
        let process = get_process!(self, cache, new_process);

        let (_, execution, inclusion, _) =
            execute_program!(process, inputs, program, "split", private_key, split_proving_key, split_verifying_key);
        let execution = inclusion_proof!(process, inclusion, execution, url);

        log("Creating execution transaction for split");
        let transaction = TransactionNative::from_execution(execution, None).map_err(|err| err.to_string())?;
        Ok(Transaction::from(transaction))
    }
}
