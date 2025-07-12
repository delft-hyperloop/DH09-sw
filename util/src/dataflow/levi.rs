use crate::dataflow::*;

pub fn make_levi_beckhoff_code(df: &DataflowSpec) -> String {
    /*
        r#"
    VAR
        i: UINT;

        test_real: LUINT_AND_BYTES;
        test_Quint: QUINT_Reals;

        send_messages: BOOL := FALSE;
    END_VAR

    IF CAN_INPUTS.RxCounter <> CAN_OUTPUTS.RxCounter THEN
        FOR i:= 0 TO (CAN_INPUTS.NoOfRxMessages - 1) DO
            Incoming_messages[i] := CAN_INPUTS.RxMessages[i];
        END_FOR
        CAN_OUTPUTS.RxCounter := CAN_INPUTS.RxCounter;

    END_IF

    IF send_messages THEN

        //Messages_To_Send[0].length := 1;
        //Messages_To_Send[0].cobId := 450;
        //Messages_To_Send[0].txData[0] := 123;

        test_real.value := 123.0;
        //Messages_To_Send[1].length := 8;
        //Messages_To_Send[1].cobId := 460;
        //Messages_To_Send[1].txData := test_real.bytes;

        test_Quint.values[0] := 41241.25;
        test_Quint.values[1] := 0;
        Messages_To_Send[0].length := 8;
        Messages_To_Send[0].cobId := 420;
        Messages_To_Send[0].txData := test_Quint.bytes;

        No_Messages_Queued := 1;
    END_IF

    //Send new messages
    IF (CAN_OUTPUTS.TxCounter = CAN_INPUTS.TxCounter) AND (No_Messages_Queued <> 0) THEN
        FOR i:= 0 TO (No_Messages_Queued - 1) DO
            CAN_OUTPUTS.TxMessages[i] := Messages_To_Send[i];
        END_FOR
        //Tell interface how many messages to send
        CAN_Outputs.NoOfTxMessages := No_Messages_Queued;
        CAN_OUTPUTS.TxCounter := CAN_INPUTS.TxCounter + 1;
        No_Messages_Queued := 0;
    END_IF

    "#;
    */
    let mut vars = String::new();
    let mut input_vars = String::new();

    let mut code = String::new();

    writeln!(
        &mut vars,
        r#"
VAR
        i: UINT := 0;
        can_out_msgs: INT := 0;
        Incoming_messages: ARRAY[0..10] OF EXTCANTXQUEUE;
        Messages_To_Send: ARRAY[0..10] OF EXTCANTXQUEUE;
        No_Messages_Queued: UINT := 0;
        tx_data: ARRAY[0..7] OF USINT;

        local_u16: UINT_AND_BYTES;
        local_u32: UDINT_AND_BYTES;

"#
    )
    .unwrap();
    writeln!(
        &mut input_vars,
        r#"
    VAR_INPUT
        
    "#
    )
    .unwrap();

    for mp in &df.message_processing {
        if let CanSpec::Can2 { id, comes_from_levi: Some(l) } = &mp.can {
            let mut tx_data_create = String::new();
            for dp in &mp.datapoint_conversion {
                let Some(levi_info) = &dp.comes_from_levi_info else {
                    panic!("no");
                };

                writeln!(
                    &mut input_vars,
                    "        {}",
                    levi_info.levi_type.make_input(&levi_info.name)
                )
                .unwrap();
                match dp.getter.ty {
                    Ty::U8 => {
                        writeln!(
                            &mut tx_data_create,
                            "    tx_data[{}] := {};",
                            dp.getter.can_payload_range.start,
                            levi_info.formula.replace("$", &levi_info.name)
                        )
                        .unwrap();
                    },
                    Ty::U16 => {
                        writeln!(
                            &mut tx_data_create,
                            "    local_u16.value := {};",
                            levi_info.formula.replace("$", &levi_info.name)
                        )
                        .unwrap();
                        writeln!(
                            &mut tx_data_create,
                            "    tx_data[{}] := local_u16.bytes[1];",
                            dp.getter.can_payload_range.start
                        )
                        .unwrap();
                        writeln!(
                            &mut tx_data_create,
                            "    tx_data[{}] := local_u16.bytes[0];",
                            dp.getter.can_payload_range.start + 1
                        )
                        .unwrap();
                    },
                    Ty::U32 => {
                        writeln!(
                            &mut tx_data_create,
                            "    local_u32.value := {};",
                            levi_info.formula.replace("$", &levi_info.name)
                        )
                        .unwrap();
                        writeln!(
                            &mut tx_data_create,
                            "    tx_data[{}] := local_u32.bytes[3];",
                            dp.getter.can_payload_range.start
                        )
                        .unwrap();
                        writeln!(
                            &mut tx_data_create,
                            "    tx_data[{}] := local_u32.bytes[2];",
                            dp.getter.can_payload_range.start + 1
                        )
                        .unwrap();
                        writeln!(
                            &mut tx_data_create,
                            "    tx_data[{}] := local_u32.bytes[1];",
                            dp.getter.can_payload_range.start + 2
                        )
                        .unwrap();
                        writeln!(
                            &mut tx_data_create,
                            "    tx_data[{}] := local_u32.bytes[0];",
                            dp.getter.can_payload_range.start + 3
                        )
                        .unwrap();
                    },
                    _ => panic!("not supported"),
                }
            }
            writeln!(&mut vars, "    can_{id}_periods_since_last_log : INT := 1000;").unwrap();
            writeln!(
                &mut code,
                "IF ({} * can_{id}_periods_since_last_log >= {} AND No_Messages_Queued < 32) THEN",
                df.beckhoff.task_period, l.log_period
            )
            .unwrap();
            writeln!(&mut code, "    Messages_To_Send[No_Messages_Queued].length := 8;").unwrap();
            writeln!(&mut code, "    Messages_To_Send[No_Messages_Queued].cobId := {id};").unwrap();
            writeln!(&mut code, "{tx_data_create}").unwrap();
            writeln!(&mut code, "    Messages_To_Send[No_Messages_Queued].txData := tx_data;")
                .unwrap();
            writeln!(&mut code, "    No_Messages_Queued := No_Messages_Queued + 1;").unwrap();
            writeln!(&mut code, "    can_{id}_periods_since_last_log := 0;").unwrap();
            writeln!(
                &mut code,
                "ELSE\n    can_{id}_periods_since_last_log := can_{id}_periods_since_last_log + 1;"
            )
            .unwrap();
            writeln!(&mut code, "END_IF;").unwrap();
        }
    }

    writeln!(&mut vars, "END_VAR").unwrap();
    writeln!(&mut input_vars, "END_VAR").unwrap();

    format!(
        "
{vars}
{input_vars}
VAR_IN_OUT
    CAN_INPUTS: CANRXQUEUESTRUCT_T_32;
    CAN_OUTPUTS: CANTXQUEUESTRUCT_X_32;
END_VAR

{code}
//Send new messages
IF (CAN_OUTPUTS.TxCounter = CAN_INPUTS.TxCounter) AND (No_Messages_Queued <> 0) THEN
	FOR i:= 0 TO (No_Messages_Queued - 1) DO
		CAN_OUTPUTS.TxMessages[i] := Messages_To_Send[i];
	END_FOR
	//Tell interface how many messages to send
	CAN_Outputs.NoOfTxMessages := No_Messages_Queued;
	CAN_OUTPUTS.TxCounter := CAN_INPUTS.TxCounter + 1;
	No_Messages_Queued := 0;
END_IF
    "
    )
}
