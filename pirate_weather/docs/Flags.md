# Flags

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sources** | Option<**Vec<String>**> | The models used to generate the forecast. | [optional]
**source_times** | Option<[**models::FlagsSourceTimes**](flags_sourceTimes.md)> |  | [optional]
**nearest_station** | Option<**f64**> | The distance to the nearest station (not implemented, always returns 0). | [optional]
**units** | Option<**String**> | The units used in the forecasts. | [optional]
**version** | Option<**String**> | The version of Pirate Weather used to generate the forecast. | [optional]
**source_idx** | Option<[**models::FlagsSourceIdx**](flags_sourceIDX.md)> |  | [optional]
**process_time** | Option<**i32**> | The time taken to process the request in milliseconds. Only returned when version>1. | [optional]
**ingest_version** | Option<**String**> | The ingest version of Pirate Weather used to generate the forecast. Only returned when version>1. | [optional]
**nearest_city** | Option<**String**> | The name of the closest city to your location. Only returned when version>1. | [optional]
**nearest_country** | Option<**String**> | The name of the closest country to your location. Only returned when version>1. | [optional]
**nearest_sub_national** | Option<**String**> | The name of the closest state or province to your location. Only returned when version>1. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


