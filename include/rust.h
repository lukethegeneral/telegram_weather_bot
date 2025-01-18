typedef struct
{
	double longitude;
	double latitude;
} GPScoordinates;

typedef struct 
{
    const char *temp_unit;
    float temp_value;
} CurrentWeather;

extern CurrentWeather get_current_temperature_c(GPScoordinates gps);