# DailyDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time** | Option<**i32**> | The time of the data point in UNIX format. | [optional]
**summary** | Option<**String**> | A summary of the weather. | [optional]
**icon** | Option<**String**> | An icon representing the weather. | [optional]
**dawn_time** | Option<**i32**> | The time when the the sun is a specific (6 degrees) height above the horizon after sunrise. Only returned when version>2. | [optional]
**sunrise_time** | Option<**i32**> | The time of sunrise in UNIX format. | [optional]
**sunset_time** | Option<**i32**> | The time of sunset in UNIX format. | [optional]
**dusk_time** | Option<**i32**> | The time when the the sun is a specific (6 degrees) height above the horizon before sunset. Only returned when version>2. | [optional]
**moon_phase** | Option<**f64**> | The fractional lunation number for the given day. | [optional]
**precip_intensity** | Option<**f64**> | The intensity of precipitation. | [optional]
**precip_intensity_max** | Option<**f64**> | The maximum intensity of precipitation. | [optional]
**precip_intensity_max_time** | Option<**i32**> | The time when the maximum precipitation intensity occurs in UNIX format. | [optional]
**precip_probability** | Option<**f64**> | The probability of precipitation. | [optional]
**precip_accumulation** | Option<**f64**> | The total amount of precipitation. | [optional]
**precip_type** | Option<**String**> | The type of precipitation occurring. | [optional]
**temperature_high** | Option<**f64**> | The daytime high temperature. | [optional]
**temperature_high_time** | Option<**i32**> | The time when the high temperature occurs in UNIX format. | [optional]
**temperature_low** | Option<**f64**> | The overnight low temperature. | [optional]
**temperature_low_time** | Option<**i32**> | The time when the low temperature occurs in UNIX format. | [optional]
**apparent_temperature_high** | Option<**f64**> | The apparent daytime high temperature (feels like). | [optional]
**apparent_temperature_high_time** | Option<**i32**> | The time when the apparent high temperature occurs in UNIX format. | [optional]
**apparent_temperature_low** | Option<**f64**> | The apparent overnight low temperature (feels like). | [optional]
**apparent_temperature_low_time** | Option<**i32**> | The time when the apparent low temperature occurs in UNIX format. | [optional]
**dew_point** | Option<**f64**> | The dew point temperature. | [optional]
**humidity** | Option<**f64**> | The relative humidity. | [optional]
**pressure** | Option<**f64**> | The air pressure. | [optional]
**wind_speed** | Option<**f64**> | The wind speed. | [optional]
**wind_gust** | Option<**f64**> | The wind gust speed. | [optional]
**wind_gust_time** | Option<**i32**> | The time when the maximum wind gust occurs in UNIX format. | [optional]
**wind_bearing** | Option<**f64**> | The direction of the wind in degrees. | [optional]
**cloud_cover** | Option<**f64**> | The fraction of the sky covered by clouds. | [optional]
**uv_index** | Option<**f64**> | The max UV index during that day. | [optional]
**uv_index_time** | Option<**i32**> | The time when the maximum UV index occurs in UNIX format. | [optional]
**visibility** | Option<**f64**> | The visibility in kilometers. | [optional]
**temperature_min** | Option<**f64**> | The minimum temperature. | [optional]
**temperature_min_time** | Option<**i32**> | The time when the minimum temperature occurs in UNIX format. | [optional]
**temperature_max** | Option<**f64**> | The maximum temperature. | [optional]
**temperature_max_time** | Option<**i32**> | The time when the maximum temperature occurs in UNIX format. | [optional]
**apparent_temperature_min** | Option<**f64**> | The minimum apparent temperature (feels like). | [optional]
**apparent_temperature_min_time** | Option<**i32**> | The time when the minimum apparent temperature occurs in UNIX format. | [optional]
**apparent_temperature_max** | Option<**f64**> | The maximum apparent temperature (feels like). | [optional]
**apparent_temperature_max_time** | Option<**i32**> | The time when the maximum apparent temperature occurs in UNIX format. | [optional]
**smoke_max** | Option<**f64**> | The maximum amount of near-surface smoke in kg/m^3. Only returned when version>2. | [optional]
**smoke_max_time** | Option<**i32**> | The time when the maximum near-surface smoke occurs in UNIX format. Only returned when version>2. | [optional]
**liquid_accumulation** | Option<**f64**> | The amount of liquid precipitation expected. Only returned when version>2. | [optional]
**snow_accumulation** | Option<**f64**> | The amount of snow precipitation expected. Only returned when version>2. | [optional]
**ice_accumulation** | Option<**f64**> | The amount of ice precipitation expected. Only returned when version>2. | [optional]
**fire_index_max** | Option<**f64**> | The maximum Fosburg fire index. Only returned when version>2. | [optional]
**fire_index_max_time** | Option<**i32**> | The time when the maximum Fosburg fire index occurs in UNIX format. Only returned when version>2. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


