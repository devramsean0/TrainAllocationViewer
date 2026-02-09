use log::error;
use serde::Deserialize;

pub fn handle_payload(payload: &[u8]) -> anyhow::Result<()> {
    let payload_str = std::str::from_utf8(payload)?;
    // Parse the XML
    let xml =
        quick_xml::de::from_str::<PassengerTrainConsistMessage>(payload_str).unwrap_or_else(|e| {
            // Catch the result fails so we can properly log it
            error!("Error parsing XML: {e}");
            return PassengerTrainConsistMessage::default();
        });
    Ok(())
}

#[derive(Deserialize, Debug, Default)]
struct PassengerTrainConsistMessage {
    #[serde(rename = "MessageHeader")]
    _message_header: MessageHeader,
    #[serde(rename = "MessageStatus")]
    _message_status: i32,
    #[serde(rename = "TrainOperationalIdentification")]
    _train_operational_identification: TrainOperationalIdentification,
    #[serde(rename = "OperationalTrainNumberIdentifier")]
    _operational_train_number_identifier: OperationalTrainNumberIdentifier,
    #[serde(rename = "ResponsibleRU")]
    _responsible_ru: String,
    #[serde(rename = "Allocation")]
    _allocation: Option<Vec<Allocation>>,
}

#[derive(Deserialize, Debug, Default)]
struct MessageHeader {
    #[serde(rename = "MessageReference")]
    _message_reference: MessageReference,
}

#[derive(Deserialize, Debug, Default)]
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

#[derive(Deserialize, Debug, Default)]
struct TrainOperationalIdentification {
    #[serde(rename = "TransportOperationalIdentifiers")]
    _train_operational_identifiers: Vec<TrainOperationalIdentifiers>,
}

#[derive(Deserialize, Debug, Default)]
struct TrainOperationalIdentifiers {
    #[serde(rename = "ObjectType")]
    _object_type: String,
    #[serde(rename = "Company")]
    _company: String,
    #[serde(rename = "Core")]
    _core: String,
    #[serde(rename = "Variant")]
    _variant: String,
    #[serde(rename = "TimetableYear")]
    _timetable_year: String,
    #[serde(rename = "StartDate")]
    _start_date: String,
}

#[derive(Deserialize, Debug, Default)]
struct OperationalTrainNumberIdentifier {
    #[serde(rename = "OperationalTrainNumber")]
    _operational_train_number: String,
    #[serde(rename = "ScheduledTimeAtHandover")]
    _scheduled_time_at_handover: String,
    #[serde(rename = "ScheduledDateTimeAtTransfer")]
    _scheduled_date_time_at_transfer: String,
}

#[derive(Deserialize, Debug)]
struct Allocation {
    #[serde(rename = "AllocationSequenceNumber")]
    _allocation_sequence_number: i64,
    #[serde(rename = "TrainOriginDateTime")]
    _train_origin_date_time: String,
    #[serde(rename = "TrainOriginLocation")]
    _train_origin_location: TrainLocation,
    #[serde(rename = "ResourceGroupPosition")]
    _resource_group_position: i64,
    #[serde(rename = "DiagramDate")]
    _diagram_date: String,
    #[serde(rename = "DiagramNo")]
    _diagram_no: Option<String>,
    #[serde(rename = "TrainDestLocation")]
    _train_dest_location: TrainLocation,
    #[serde(rename = "TrainDestDateTime")]
    _train_dest_date_time: String,
    #[serde(rename = "AllocationOriginLocation")]
    _allocation_origin_location: TrainLocation,
    #[serde(rename = "AllocationOriginDateTime")]
    _allocation_origin_date_time: String,
    #[serde(rename = "AllocationOriginMiles")]
    _allocation_origin_miles: i64,
    #[serde(rename = "AllocationDestinationLocation")]
    _allocation_destination_location: TrainLocation,
    #[serde(rename = "AllocationDestinationDateTime")]
    _allocation_destination_date_time: String,
    #[serde(rename = "AllocationDestinationMiles")]
    _allocation_destination_miles: i64,
    #[serde(rename = "Reversed")]
    _reversed: String,
    #[serde(rename = "ResourceGroup")]
    _resource_group: ResourceGroup,
}

#[derive(Deserialize, Debug)]
struct TrainLocation {
    #[serde(rename = "CountryCodeISO")]
    _country_code_iso: String,
    #[serde(rename = "LocationPrimaryCode")]
    _location_primary_code: String,
    #[serde(rename = "LocationSubsidiaryIdentification")]
    _location_subsidiary_identification: LocationSubsidiaryIdentification,
}

#[derive(Deserialize, Debug)]
struct LocationSubsidiaryIdentification {
    #[serde(rename = "LocationSubsidiaryCode")]
    _location_sibsidiary_code: String,
    #[serde(rename = "AllocationCompany")]
    _allocation_company: String,
}

#[derive(Deserialize, Debug)]
struct ResourceGroup {
    #[serde(rename = "ResourceGroupId")]
    _resource_group_id: String,
    #[serde(rename = "TypeOfResource")]
    _type_of_resource: String,
    #[serde(rename = "FleetId")]
    _fleet_id: String,
    #[serde(rename = "ResourceGroupStatus")]
    _resource_group_status: String,
    #[serde(rename = "EndOfDayMiles")]
    _end_of_day_miles: String,
    #[serde(rename = "Vehicle")]
    _vehicle: Vec<Vehicle>,
}

#[derive(Deserialize, Debug)]
struct Vehicle {
    #[serde(rename = "VehicleId")]
    _vehicle_id: String,
    #[serde(rename = "TypeOfVehicle")]
    _type_of_vehicle: String,
    #[serde(rename = "ResourcePosition")]
    _resource_position: i64,
    #[serde(rename = "PlannedResourceGroup")]
    _planned_resource_group: String,
    #[serde(rename = "SpecificType")]
    _specific_type: String,
    #[serde(rename = "Length")]
    _length: Length,
    #[serde(rename = "Weight")]
    _weight: i32,
    #[serde(rename = "Livery")]
    _livery: String,
    #[serde(rename = "Decor")]
    _decor: String,
    #[serde(rename = "SpecialCharacteristics")]
    _special_characteristics: Option<String>,
    #[serde(rename = "NumberOfSeats")]
    _number_of_seats: i32,
    #[serde(rename = "RegisteredStatus")]
    _registered_status: String,
    #[serde(rename = "Cabs")]
    _cabs: Option<i32>,
    #[serde(rename = "DateEnteredService")]
    _date_entered_service: String,
    #[serde(rename = "DateRegistered")]
    _date_registered: String,
    #[serde(rename = "RegisteredCategory")]
    _registered_category: String,
    #[serde(rename = "TrainBrakeType")]
    _train_brake_type: String,
    #[serde(rename = "MaximumSpeed")]
    _maximum_speed: String,
}

#[derive(Deserialize, Debug)]
struct Length {
    #[serde(rename = "Value")]
    _value: String,
    #[serde(rename = "Measure")]
    _measure: String,
}
