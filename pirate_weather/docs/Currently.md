# Currently

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time** | Option<**i32**> | The current time in UNIX format. | [optional]
**summary** | Option<**String**> | A human-readable summary of the current weather. | [optional]
**icon** | Option<**String**> | An icon representing the current weather. | [optional]
**nearest_storm_distance** | Option<**f64**> | The distance to the nearest storm in kilometers. | [optional]
**nearest_storm_bearing** | Option<**i32**> | The direction to the nearest storm in degrees. | [optional]
**precip_intensity** | Option<**f64**> | The intensity of liquid water equivalent precipitation in millimeters per hour. | [optional]
**precip_probability** | Option<**f64**> | The probability of precipitation occurring. | [optional]
**precip_intensity_error** | Option<**f64**> | The standard deviation of the precipitation intensity. | [optional]
**precip_type** | Option<**String**> | The type of precipitation occurring. | [optional]
**temperature** | Option<**f64**> | The air temperature. | [optional]
**apparent_temperature** | Option<**f64**> | The apparent temperature (feels like). | [optional]
**dew_point** | Option<**f64**> | The dew point temperature. | [optional]
**humidity** | Option<**f64**> | The relative humidity. | [optional]
**pressure** | Option<**f64**> | The sea-level pressure in hectopascals. | [optional]
**wind_speed** | Option<**f64**> | The wind speed. | [optional]
**wind_gust** | Option<**f64**> | The wind gust speed. | [optional]
**wind_bearing** | Option<**i32**> | The direction of the wind in degrees. | [optional]
**cloud_cover** | Option<**f64**> | The fraction of the sky covered by clouds. | [optional]
**uv_index** | Option<**f64**> | The UV index. | [optional]
**visibility** | Option<**f64**> | The visibility in kilometers. | [optional]
**ozone** | Option<**f64**> | The ozone concentration in Dobson units. | [optional]
**smoke** | Option<**f64**> | The amount of near-surface smoke in ug/m^3. Only returned when version>1. | [optional]
**fire_index** | Option<**f64**> | The Fosburg fire index. Only returned when version>1. | [optional]
**feels_like** | Option<**f64**> | The apparent temperature reported by NBM and gfs. Only returned when version>1. | [optional]
**current_day_ice** | Option<**f64**> | The ice precipitation that has accumulated so far during the day, from midnight until the forecast request time. Only returned when version>1. | [optional]
**current_day_liquid** | Option<**f64**> | The liquid precipitation that has accumulated so far during the day, from midnight until the forecast request time. Only returned when version>1. | [optional]
**current_day_snow** | Option<**f64**> | The snow precipitation that has accumulated so far during the day, from midnight until the forecast request time. Only returned when version>1. | [optional]
**station_pressure** | Option<**f64**> | The station pressure hectopascals. Only returned when extraVars contains stationPressure. | [optional]
**solar** | Option<**f64**> | The Downward Short-Wave Radiation Flux measured in W/m^2. Only returned when version>1. | [optional]
**cape** | Option<**i32**> | The Convective Available Potential Energy measured in J/kg. Only returned when version>1. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


