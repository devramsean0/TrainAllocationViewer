use serde::Deserialize;

pub fn handle_payload(payload: &[u8]) -> anyhow::Result<()> {
    let payload_str = std::str::from_utf8(payload)?;
    // Parse the XML
    let xml = quick_xml::de::from_str::<PassengerTrainConsistMessage>(payload_str)?;
    println!("XML: {:?}", xml);
    Ok(())
}

#[derive(Deserialize, Debug)]
struct PassengerTrainConsistMessage {
    #[serde(rename = "MessageHeader")]
    _message_header: MessageHeader,
    #[serde(rename = "MessageStatus")]
    message_status: i32,
    #[serde(rename = "TrainOperationalIdentification")]
    train_operational_identification: TrainOperationalIdentification,
}

#[derive(Deserialize, Debug)]
struct MessageHeader {
    #[serde(rename = "MessageReference")]
    _message_reference: MessageReference,
}

#[derive(Deserialize, Debug)]
struct MessageReference {
    #[serde(rename = "MessageType")]
    _message_type: i64,
    #[serde(rename = "MessageTypeVersion")]
    _message_type_version: String,
    #[serde(rename = "MessageIdentifier")]
    _message_identifier: String,
    #[serde(rename = "MessageDateTime")]
    _message_date_time: String,
}

#[derive(Deserialize, Debug)]
struct TrainOperationalIdentification {
    #[serde(rename = "TransportOperationalIdentifiers")]
    train_operational_identifiers: Vec<TrainOperationalIdentifiers>,
}

#[derive(Deserialize, Debug)]
struct TrainOperationalIdentifiers {
    #[serde(rename = "ObjectType")]
    object_type: String,
    #[serde(rename = "Company")]
    company: String,
    #[serde(rename = "Core")]
    core: String,
    #[serde(rename = "Variant")]
    variant: String,
    #[serde(rename = "TimetableYear")]
    timetable_year: String,
    #[serde(rename = "StartDate")]
    start_date: String,
}

#[derive(Deserialize, Debug)]
struct OperationalTrainNumberIdentifier {}
