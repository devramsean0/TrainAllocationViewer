use log::error;
use serde::Deserialize;

pub fn handle_payload(payload: &[u8]) -> anyhow::Result<PassengerTrainConsistMessage> {
    let payload_str = std::str::from_utf8(payload)?;
    // Parse the XML
    let xml =
        quick_xml::de::from_str::<PassengerTrainConsistMessage>(payload_str).unwrap_or_else(|e| {
            // Catch the result fails so we can properly log it
            error!("Error parsing XML: {e}");
            return PassengerTrainConsistMessage::default();
        });
    Ok(xml)
}

#[derive(Deserialize, Debug, Default)]
pub struct PassengerTrainConsistMessage {
    #[serde(rename = "MessageHeader")]
    pub _message_header: MessageHeader,
    #[serde(rename = "MessageStatus")]
    pub _message_statius: i32,
    #[serde(rename = "TrainOperationalIdentification")]
    pub _train_operational_identification: TrainOperationalIdentification,
    #[serde(rename = "OperationalTrainNumberIdentifier")]
    pub _operational_train_number_identifier: OperationalTrainNumberIdentifier,
    #[serde(rename = "ResponsibleRU")]
    pub _responsible_ru: String,
    #[serde(rename = "Allocation")]
    pub allocation: Option<Vec<Allocation>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct MessageHeader {
    #[serde(rename = "MessageReference")]
    pub _message_reference: MessageReference,
}

#[derive(Deserialize, Debug, Default)]
pub struct MessageReference {
    #[serde(rename = "MessageType")]
    pub _message_type: i64,
    #[serde(rename = "MessageTypeVersion")]
    pub _message_type_version: String,
    #[serde(rename = "MessageIdentifier")]
    pub _message_identifier: String,
    #[serde(rename = "MessageDateTime")]
    pub _message_date_time: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct TrainOperationalIdentification {
    #[serde(rename = "TransportOperationalIdentifiers")]
    pub _train_operational_identifiers: Vec<TrainOperationalIdentifiers>,
}

#[derive(Deserialize, Debug, Default)]
pub struct TrainOperationalIdentifiers {
    #[serde(rename = "ObjectType")]
    pub _object_type: String,
    #[serde(rename = "Company")]
    pub _company: String,
    #[serde(rename = "Core")]
    pub _core: String,
    #[serde(rename = "Variant")]
    pub _variant: String,
    #[serde(rename = "TimetableYear")]
    pub _timetable_year: String,
    #[serde(rename = "StartDate")]
    pub _start_date: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct OperationalTrainNumberIdentifier {
    #[serde(rename = "OperationalTrainNumber")]
    pub _operational_train_number: String,
    #[serde(rename = "ScheduledTimeAtHandover")]
    pub _scheduled_time_at_handover: String,
    #[serde(rename = "ScheduledDateTimeAtTransfer")]
    pub _scheduled_date_time_at_transfer: String,
}

#[derive(Deserialize, Debug)]
pub struct Allocation {
    #[serde(rename = "AllocationSequenceNumber")]
    pub _allocation_sequence_number: i64,
    #[serde(rename = "TrainOriginDateTime")]
    pub train_origin_date_time: String,
    #[serde(rename = "TrainOriginLocation")]
    pub train_origin_location: TrainLocation,
    #[serde(rename = "ResourceGroupPosition")]
    pub _resource_group_position: i64,
    #[serde(rename = "DiagramDate")]
    pub diagram_date: Option<String>,
    #[serde(rename = "DiagramNo")]
    pub _diagram_no: Option<String>,
    #[serde(rename = "TrainDestLocation")]
    pub train_dest_location: TrainLocation,
    #[serde(rename = "TrainDestDateTime")]
    pub train_dest_date_time: String,
    #[serde(rename = "AllocationOriginLocation")]
    pub allocation_origin_location: TrainLocation,
    #[serde(rename = "AllocationOriginDateTime")]
    pub allocation_origin_date_time: String,
    #[serde(rename = "AllocationOriginMiles")]
    pub _allocation_origin_miles: i64,
    #[serde(rename = "AllocationDestinationLocation")]
    pub allocation_destination_location: TrainLocation,
    #[serde(rename = "AllocationDestinationDateTime")]
    pub allocation_destination_date_time: String,
    #[serde(rename = "AllocationDestinationMiles")]
    pub _allocation_destination_miles: i64,
    #[serde(rename = "Reversed")]
    pub _reversed: String,
    #[serde(rename = "ResourceGroup")]
    pub resource_group: ResourceGroup,
}

#[derive(Deserialize, Debug)]
pub struct TrainLocation {
    #[serde(rename = "CountryCodeISO")]
    pub _country_code_iso: String,
    #[serde(rename = "LocationPrimaryCode")]
    pub location_primary_code: String,
    #[serde(rename = "LocationSubsidiaryIdentification")]
    pub _location_subsidiary_identification: LocationSubsidiaryIdentification,
}

#[derive(Deserialize, Debug)]
pub struct LocationSubsidiaryIdentification {
    #[serde(rename = "LocationSubsidiaryCode")]
    pub _location_sibsidiary_code: String,
    #[serde(rename = "AllocationCompany")]
    pub _allocation_company: String,
}

#[derive(Deserialize, Debug)]
pub struct ResourceGroup {
    #[serde(rename = "ResourceGroupId")]
    pub _resource_group_id: String,
    #[serde(rename = "TypeOfResource")]
    pub _type_of_resource: String,
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    #[serde(rename = "ResourceGroupStatus")]
    pub _resource_group_status: String,
    #[serde(rename = "EndOfDayMiles")]
    pub _end_of_day_miles: String,
    #[serde(rename = "Vehicle")]
    pub vehicle: Vec<Vehicle>,
}

#[derive(Deserialize, Debug)]
pub struct Vehicle {
    #[serde(rename = "VehicleId")]
    pub vehicle_id: i64,
    #[serde(rename = "TypeOfVehicle")]
    pub type_of_vehicle: String,
    #[serde(rename = "ResourcePosition")]
    pub _resource_position: i64,
    #[serde(rename = "PlannedResourceGroup")]
    pub _planned_resource_group: String,
    #[serde(rename = "SpecificType")]
    pub specific_type: String,
    #[serde(rename = "Length")]
    pub _length: Length,
    #[serde(rename = "Weight")]
    pub _weight: i32,
    #[serde(rename = "Livery")]
    pub livery: String,
    #[serde(rename = "Decor")]
    pub decor: String,
    #[serde(rename = "SpecialCharacteristics")]
    pub _special_characteristics: Option<String>,
    #[serde(rename = "NumberOfSeats")]
    pub _number_of_seats: Option<i32>,
    #[serde(rename = "RegisteredStatus")]
    pub _registered_status: String,
    #[serde(rename = "Cabs")]
    pub _cabs: Option<i32>,
    #[serde(rename = "DateEnteredService")]
    pub _date_entered_service: String,
    #[serde(rename = "DateRegistered")]
    pub _date_registered: String,
    #[serde(rename = "RegisteredCategory")]
    pub _registered_category: String,
    #[serde(rename = "TrainBrakeType")]
    pub _train_brake_type: String,
    #[serde(rename = "MaximumSpeed")]
    pub _maximum_speed: String,
}

#[derive(Deserialize, Debug)]
pub struct Length {
    #[serde(rename = "Value")]
    pub _value: String,
    #[serde(rename = "Measure")]
    pub _measure: String,
}
