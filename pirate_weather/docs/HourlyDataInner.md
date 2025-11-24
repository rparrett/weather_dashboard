# HourlyDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time** | Option<**i32**> | The time of the data point in UNIX format. | [optional]
**summary** | Option<**String**> | A summary of the weather. | [optional]
**icon** | Option<**String**> | An icon representing the weather. | [optional]
**precip_intensity** | Option<**f64**> | The intensity of precipitation. | [optional]
**precip_probability** | Option<**f64**> | The probability of precipitation. | [optional]
**precip_intensity_error** | Option<**f64**> | The standard deviation of the precipitation intensity. | [optional]
**precip_accumulation** | Option<**f64**> | The total amount of precipitation. | [optional]
**precip_type** | Option<**String**> | The type of precipitation occurring. | [optional]
**temperature** | Option<**f64**> | The air temperature. | [optional]
**apparent_temperature** | Option<**f64**> | The apparent temperature (feels like). | [optional]
**dew_point** | Option<**f64**> | The dew point temperature. | [optional]
**humidity** | Option<**f64**> | The relative humidity. | [optional]
**pressure** | Option<**f64**> | The air pressure. | [optional]
**station_pressure** | Option<**f64**> | The station pressure. Only returned when extraVars contains stationPressure. | [optional]
**wind_speed** | Option<**f64**> | The wind speed. | [optional]
**wind_gust** | Option<**f64**> | The wind gust speed. | [optional]
**wind_bearing** | Option<**i32**> | The direction of the wind in degrees. | [optional]
**cloud_cover** | Option<**f64**> | The fraction of the sky covered by clouds. | [optional]
**uv_index** | Option<**f64**> | The UV index. | [optional]
**visibility** | Option<**f64**> | The visibility in kilometers. | [optional]
**ozone** | Option<**f64**> | The ozone concentration in Dobson units. | [optional]
**smoke** | Option<**f64**> | The amount of near-surface smoke in ug/m3. Only returned when version>1. | [optional]
**liquid_accumulation** | Option<**f64**> | The amount of liquid precipitation expected. Only returned when version>1. | [optional]
**snow_accumulation** | Option<**f64**> | The amount of snow precipitation expected. Only returned when version>1. | [optional]
**ice_accumulation** | Option<**f64**> | The amount of ice precipitation expected. Only returned when version>1. | [optional]
**nearest_storm_distance** | Option<**f64**> | The distance to the nearest storm. | [optional]
**nearest_storm_bearing** | Option<**i32**> | The direction to the nearest storm. | [optional]
**fire_index** | Option<**f64**> | The Fosburg fire index. Only returned when version>1. | [optional]
**feels_like** | Option<**f64**> | The apparent temperature reported by NBM and gfs. Only returned when version>1. | [optional]
**solar** | Option<**f64**> | The Downward Short-Wave Radiation Flux measured in W/m^2. Only returned when version>1. | [optional]
**cape** | Option<**i32**> | The Convective Available Potential Energy measured in J/kg. Only returned when version>1. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


